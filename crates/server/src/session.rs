use ai::Seated;
use data::GameData;
use engine::{Decision, EngineError, Outcome, Replay, Setup, Status, play};

use crate::app_error::AppError;
use crate::database::{Database, DatabaseError};
use crate::game_row::GameContent;
use crate::progress::Progress;
use crate::views::on_sheet;

/// Seat of the human player; the AI plays every other seat.
pub const HUMAN_SEAT: usize = 0;

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
    pub async fn start(db: &Database, data: &GameData, setup: Setup) -> Result<Self, AppError> {
        let mut progress = Progress {
            setup,
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

    /// Where the human answers next: the investigator's page for the
    /// sliders and the hands, the game page otherwise.
    pub fn page(&self) -> String {
        match self.decision() {
            Some(decision) if on_sheet(decision) => format!("/partie/{}/investigateurs", self.id),
            _ => format!("/partie/{}", self.id),
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
    fn replay(&mut self, data: &GameData) -> Result<(), AppError> {
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
        title: title(data, outcome),
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
/// human's.
fn advance(data: &GameData, progress: &Progress) -> Result<Outcome, AppError> {
    let mut seated = Seated::new(HUMAN_SEAT);
    let mut replay = Replay::new(&progress.answers, &mut seated);
    Ok(play(data, &progress.setup, &mut replay)?)
}

/// « Amanda Sharpe et 2 IA contre Azathoth ».
fn title(data: &GameData, outcome: &Outcome) -> String {
    let state = &outcome.state;
    let human = state
        .investigators
        .get(HUMAN_SEAT)
        .and_then(|inv| {
            data.investigators
                .investigators
                .iter()
                .find(|sheet| sheet.id == inv.id)
        })
        .map_or("?", |sheet| sheet.name.as_str());
    let ancient_one = data
        .ancient_ones
        .ancient_ones
        .iter()
        .find(|sheet| sheet.id == state.ancient_one)
        .map_or("?", |sheet| sheet.name.as_str());
    match state.players().saturating_sub(1) {
        0 => format!("{human}, seul contre {ancient_one}"),
        ais => format!("{human} et {ais} IA contre {ancient_one}"),
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
