//! A trade set in one form: the engine asks it one item at a time, so the
//! items picked are turned into those answers here.

use data::{CardId, GameData};
use engine::Choice;

use crate::app_error::AppError;
use crate::session::Session;
use crate::views::is_trade;

/// What the human picked on the trade page.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TradePlan {
    /// Items given.
    pub give: Vec<CardId>,
    /// Items taken.
    pub take: Vec<CardId>,
    /// Dollars given.
    pub give_money: u16,
    /// Dollars taken.
    pub take_money: u16,
}

impl TradePlan {
    /// The answers that put the offer together, then propose it; only a
    /// cancellation when nothing would change hands.
    fn answers(&self) -> Vec<Choice> {
        let mut answers: Vec<Choice> = self.give.iter().cloned().map(Choice::Give).collect();
        answers.extend(self.take.iter().cloned().map(Choice::Take));
        if self.give_money > 0 {
            answers.push(Choice::GiveMoney(self.give_money));
        }
        if self.take_money > 0 {
            answers.push(Choice::TakeMoney(self.take_money));
        }
        answers.push(if answers.is_empty() {
            Choice::Done
        } else {
            Choice::Yes
        });
        answers
    }
}

impl Session {
    /// Puts the trade under way together as `plan` says and proposes it
    /// (`None`: cancels it); the other investigator then answers. Undoing
    /// takes the whole trade back.
    ///
    /// # Returns
    ///
    /// `false` when the form was out of date and nothing was done.
    ///
    /// # Errors
    ///
    /// [`AppError::BadRequest`] if no trade is under way or an item or an
    /// amount cannot be traded; another [`AppError`] if the replay fails.
    pub fn trade(
        &mut self,
        data: &GameData,
        plan: Option<&TradePlan>,
        move_number: usize,
    ) -> Result<bool, AppError> {
        if move_number != self.move_number() {
            return Ok(false);
        }
        let seat = self
            .decision()
            .filter(|decision| is_trade(decision))
            .map(|decision| decision.who)
            .ok_or_else(|| bad("Aucun échange en cours."))?;
        let answers = plan.map_or_else(|| vec![Choice::Done], TradePlan::answers);
        self.progress
            .human_moves
            .push((self.progress.answers.len(), self.progress.seen_events));
        self.progress.seen_events = self.outcome.events.len();
        for wanted in answers {
            let index = self
                .decision()
                .filter(|decision| is_trade(decision) && decision.who == seat)
                .and_then(|decision| decision.find(&wanted))
                .ok_or_else(|| bad("Cet échange n'est pas possible."))?;
            self.progress.answers.push(index);
            self.replay(data)?;
        }
        Ok(true)
    }
}

fn bad(message: &str) -> AppError {
    AppError::BadRequest(message.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_plan_only_cancels() {
        assert_eq!(TradePlan::default().answers(), [Choice::Done]);
        let plan = TradePlan {
            give: vec![CardId::new("lantern")],
            take_money: 2,
            ..TradePlan::default()
        };
        assert_eq!(
            plan.answers(),
            [
                Choice::Give(CardId::new("lantern")),
                Choice::TakeMoney(2),
                Choice::Yes
            ]
        );
    }
}
