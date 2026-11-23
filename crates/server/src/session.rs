use ai::Seated;
use data::GameData;
use engine::{
    Choice, Decision, DecisionKind, EngineError, Outcome, Replay, Responder, Setup, Status,
    Waiting, play,
};
use serde::Deserialize;

use crate::app_error::AppError;
use crate::database::{Database, DatabaseError};
use crate::game_row::GameContent;
use crate::play_mode::{HUMAN_SEAT, PlayMode};
use crate::progress::Progress;
use crate::views::{CombatSpan, is_trade, on_sheet};

/// What to do once the cards in hand are kept, sent with the answer that
/// keeps them (so fighting is one click on the combat page).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum FollowUp {
    /// Fight this round.
    #[serde(rename = "combattre")]
    Fight,
    /// Flee.
    #[serde(rename = "fuir")]
    Flee,
}

/// A game loaded from the database and replayed up to the human's next
/// decision (or its end).
pub struct Session {
    /// Game number.
    pub id: i64,
    /// What is saved.
    pub progress: Progress,
    /// The game as replayed.
    pub outcome: Outcome,
}

impl Session {
    /// Creates a game, lets the AI play up to the human's first decision
    /// and saves it.
    ///
    /// # Errors
    ///
    /// An [`AppError`] if the setup is refused or the database fails.
    pub async fn start(
        db: &Database,
        data: &GameData,
        setup: Setup,
        mode: PlayMode,
    ) -> Result<Self, AppError> {
        let mut progress = Progress {
            setup,
            mode,
            answers: Vec::new(),
            human_moves: Vec::new(),
            seen_events: 0,
        };
        let outcome = advance(data, &progress)?;
        progress.answers.clone_from(&outcome.answers);
        let id = db.insert_game(&content(data, &progress, &outcome)?).await?;
        Ok(Self {
            id,
            progress,
            outcome,
        })
    }

    /// Loads and replays a saved game. A game whose answers no longer fit
    /// the rules (saved before a rules change) is deleted.
    ///
    /// # Errors
    ///
    /// [`AppError::NotFound`] if there is no such game,
    /// [`AppError::Outdated`] if it was deleted, or another [`AppError`] if
    /// it cannot be read or replayed.
    pub async fn load(db: &Database, data: &GameData, id: i64) -> Result<Self, AppError> {
        let row = db.game(id).await?.ok_or(AppError::NotFound)?;
        let progress: Progress = serde_json::from_str(&row.record)?;
        let outcome = match advance(data, &progress) {
            Err(error) if outdated(&error) => {
                tracing::warn!(game = id, %error, "game no longer fits the rules: deleted");
                db.delete_game(id).await?;
                return Err(AppError::Outdated);
            }
            other => other?,
        };
        Ok(Self {
            id,
            progress,
            outcome,
        })
    }

    /// The decision the human must take, unless the game is over.
    pub fn decision(&self) -> Option<&Decision> {
        match &self.outcome.status {
            Status::Waiting(decision) => Some(decision),
            Status::Over(_) => None,
        }
    }

    /// Where the human goes after a move, as a path from the application
    /// root without a leading `/`: the combat page when a combat of
    /// theirs ended since their last answer, else [`Session::decision_page`].
    pub fn page(&self) -> String {
        let ended = self
            .combat()
            .is_some_and(|span| span.ended_since(self.progress.seen_events));
        if ended && !self.in_combat() {
            return format!("partie/{}/combat", self.id);
        }
        self.decision_page()
    }

    /// Where the human answers the decision: the investigator's page for
    /// the sliders, the trade page for a trade, the combat page while
    /// facing a monster, the game page otherwise.
    pub fn decision_page(&self) -> String {
        match self.decision() {
            Some(decision) if on_sheet(decision) => format!("partie/{}/investigateurs", self.id),
            Some(decision) if is_trade(decision) => format!("partie/{}/echange", self.id),
            Some(_) if self.in_combat() => format!("partie/{}/combat", self.id),
            _ => format!("partie/{}", self.id),
        }
    }

    /// Who plays the investigators.
    pub fn mode(&self) -> PlayMode {
        self.progress.mode
    }

    /// The seat the human plays now: the one who decides in a hot-seat
    /// game, else the human's seat.
    pub fn player_seat(&self) -> usize {
        match (self.mode(), self.decision()) {
            (PlayMode::HotSeat, Some(decision)) => decision.who,
            _ => HUMAN_SEAT,
        }
    }

    /// The seat whose combat the combat page shows: the human's in a solo
    /// game; in a hot-seat game, the one about to face a monster, else the
    /// one who fought last.
    pub fn combat_seat(&self) -> usize {
        match (self.mode(), self.decision()) {
            (PlayMode::Solo, _) => HUMAN_SEAT,
            (PlayMode::HotSeat, Some(decision)) if decision.kind == DecisionKind::FaceMonster => {
                decision.who
            }
            (PlayMode::HotSeat, _) => {
                CombatSpan::last_seat(&self.outcome.shown).unwrap_or_else(|| self.player_seat())
            }
        }
    }

    /// The last time the investigator at [`Session::combat_seat`] faced a
    /// monster.
    pub fn combat(&self) -> Option<CombatSpan> {
        CombatSpan::last(&self.outcome.shown, self.combat_seat())
    }

    /// Whether the human's decision is about a monster faced: whether to
    /// evade or fight it, or a step of the combat under way.
    pub fn in_combat(&self) -> bool {
        match self.decision() {
            Some(decision) if decision.kind == DecisionKind::FaceMonster => true,
            Some(decision) => {
                decision.who == self.combat_seat()
                    && self.combat().is_some_and(|span| span.ongoing())
            }
            None => false,
        }
    }

    /// Number of answers given so far; forms carry it so that a form sent
    /// twice, or from an old page, is ignored.
    pub fn move_number(&self) -> usize {
        self.progress.answers.len()
    }

    /// Applies the human's answer, then lets the AI play up to the human's
    /// next decision.
    ///
    /// # Arguments
    ///
    /// * `choice` - index of the chosen option.
    /// * `move_number` - [`Session::move_number`] when the form was shown.
    ///
    /// # Returns
    ///
    /// `false` when the form was out of date and nothing was done.
    ///
    /// # Errors
    ///
    /// [`AppError::BadRequest`] if the game is over or the option does not
    /// exist; another [`AppError`] if the replay fails.
    pub fn answer(
        &mut self,
        data: &GameData,
        choice: usize,
        move_number: usize,
    ) -> Result<bool, AppError> {
        if move_number != self.move_number() {
            return Ok(false);
        }
        let options = self
            .decision()
            .ok_or_else(|| AppError::BadRequest("La partie est terminée.".to_owned()))?
            .options
            .len();
        if choice >= options {
            return Err(AppError::BadRequest(format!(
                "Choix n° {} inconnu : il y a {options} option(s).",
                choice + 1
            )));
        }
        self.progress
            .human_moves
            .push((self.progress.answers.len(), self.progress.seen_events));
        self.progress.seen_events = self.outcome.events.len();
        self.progress.answers.push(choice);
        self.replay(data)?;
        Ok(true)
    }

    /// After the cards in hand were kept: fights or flees at once, when the
    /// combat asks it next. Undoing takes both answers back.
    ///
    /// # Errors
    ///
    /// An [`AppError`] if the replay fails.
    pub fn follow_up(&mut self, data: &GameData, follow_up: FollowUp) -> Result<(), AppError> {
        let wanted = match follow_up {
            FollowUp::Fight => Choice::Attack,
            FollowUp::Flee => Choice::Flee,
        };
        let Some(index) = self
            .decision()
            .filter(|decision| decision.kind == DecisionKind::FightOrFlee)
            .and_then(|decision| {
                decision
                    .options
                    .iter()
                    .position(|option| option.choice == wanted)
            })
        else {
            return Ok(());
        };
        self.progress.answers.push(index);
        self.replay(data)
    }

    /// Takes back the human's last answer and everything played after it.
    ///
    /// # Returns
    ///
    /// `false` when there is nothing to undo.
    ///
    /// # Errors
    ///
    /// An [`AppError`] if the replay fails.
    pub fn undo(&mut self, data: &GameData) -> Result<bool, AppError> {
        let Some((position, seen)) = self.progress.human_moves.pop() else {
            return Ok(false);
        };
        self.progress.answers.truncate(position);
        self.progress.seen_events = seen;
        self.replay(data)?;
        Ok(true)
    }

    /// Saves the game.
    ///
    /// # Errors
    ///
    /// An [`AppError`] if the database fails.
    pub async fn save(&self, db: &Database, data: &GameData) -> Result<(), AppError> {
        let content = content(data, &self.progress, &self.outcome)?;
        db.update_game(self.id, &content).await?;
        Ok(())
    }

    /// Replays from the saved answers and keeps the AI's new ones.
    pub(crate) fn replay(&mut self, data: &GameData) -> Result<(), AppError> {
        self.outcome = advance(data, &self.progress)?;
        self.progress.answers.clone_from(&self.outcome.answers);
        Ok(())
    }
}

/// What the database keeps for a game.
fn content(
    data: &GameData,
    progress: &Progress,
    outcome: &Outcome,
) -> Result<GameContent, AppError> {
    Ok(GameContent {
        title: title(data, outcome, progress.mode),
        summary: summary(outcome),
        over: matches!(outcome.status, Status::Over(_)),
        record: serde_json::to_string(progress)?,
    })
}

/// Deletes the saved games whose answers no longer fit the rules.
///
/// # Returns
///
/// How many were deleted.
///
/// # Errors
///
/// A [`DatabaseError`] if the database fails. A game that cannot be read
/// or replayed for another reason is left alone.
pub async fn delete_outdated_games(db: &Database, data: &GameData) -> Result<usize, DatabaseError> {
    let mut deleted = 0;
    for row in db.all_games().await? {
        let Ok(progress) = serde_json::from_str::<Progress>(&row.record) else {
            continue;
        };
        if let Err(error) = advance(data, &progress)
            && outdated(&error)
        {
            tracing::warn!(game = row.id, %error, "game no longer fits the rules: deleted");
            db.delete_game(row.id).await?;
            deleted += 1;
        }
    }
    Ok(deleted)
}

/// Whether a replay failed because a saved answer no longer fits the rules.
fn outdated(error: &AppError) -> bool {
    matches!(error, AppError::Engine(EngineError::IllegalAnswer { .. }))
}

/// Replays the saved answers, then lets the AI answer every seat but the
/// human's (none in a hot-seat game).
fn advance(data: &GameData, progress: &Progress) -> Result<Outcome, AppError> {
    let mut seated = Seated::new(HUMAN_SEAT);
    let mut waiting = Waiting;
    let then: &mut dyn Responder = match progress.mode {
        PlayMode::Solo => &mut seated,
        PlayMode::HotSeat => &mut waiting,
    };
    let mut replay = Replay::new(&progress.answers, then);
    Ok(play(data, &progress.setup, &mut replay)?)
}

/// « Amanda Sharpe et 2 IA contre Azathoth », or in a hot-seat game
/// « Amanda Sharpe, Bob Jenkins et Joe Diamond contre Azathoth ».
fn title(data: &GameData, outcome: &Outcome, mode: PlayMode) -> String {
    let state = &outcome.state;
    let ancient_one = data
        .ancient_ones
        .ancient_ones
        .iter()
        .find(|sheet| sheet.id == state.ancient_one)
        .map_or("?", |sheet| sheet.name.as_str());
    let names: Vec<String> = state
        .investigators
        .iter()
        .map(|inv| crate::views::investigator_name(data, &inv.id))
        .collect();
    let human = names.get(HUMAN_SEAT).map_or("?", String::as_str);
    match (mode, names.split_last()) {
        (_, None | Some((_, []))) => format!("{human}, seul contre {ancient_one}"),
        (PlayMode::Solo, Some((_, others))) => {
            format!("{human} et {} IA contre {ancient_one}", others.len())
        }
        (PlayMode::HotSeat, Some((last, others))) => {
            format!("{} et {last} contre {ancient_one}", others.join(", "))
        }
    }
}

/// « Tour 4 · Mouvement », or how the game ended.
fn summary(outcome: &Outcome) -> String {
    match &outcome.status {
        Status::Over(ending) => ending.label().to_owned(),
        Status::Waiting(_) => format!(
            "Tour {} · {}",
            outcome.state.turn,
            outcome.state.phase.label()
        ),
    }
}
