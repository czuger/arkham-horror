//! Command-line arguments, in French.

use data::{AncientOneId, InvestigatorId};
use engine::Setup;

/// Help text.
pub const USAGE: &str = "Usage :
  simulator jouer [--graine N] [--joueurs N] [--investigateur ID]... [--grand-ancien ID]
      Joue une partie en mode texte : vous répondez à chaque décision par son numéro.
  simulator masse [--parties N] [--graine N] [--joueurs N] [--grand-ancien ID]
      Joue N parties avec des réponses au hasard et affiche le bilan.
Les données sont lues dans le dossier ARKHAM_DATA (par défaut : data).";

/// What to do.
pub enum Command {
    /// Show the help.
    Help,
    /// Play one game in text mode.
    Play(Setup),
    /// Play many games with random answers.
    Bulk {
        /// Number of games.
        games: u64,
        /// Setup of the first game; the seed goes up by one per game.
        setup: Setup,
    },
}

/// Parses the arguments.
pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Command, String> {
    let Some(verb) = args.next() else {
        return Ok(Command::Help);
    };
    let mut setup = Setup::random(seed_from_clock(), 1);
    let mut games = 100;
    while let Some(flag) = args.next() {
        let value = args
            .next()
            .ok_or_else(|| format!("valeur manquante après {flag}"))?;
        match flag.as_str() {
            "--graine" => setup.seed = number(&flag, &value)?,
            "--joueurs" => {
                setup.players =
                    usize::try_from(number(&flag, &value)?).map_err(|e| e.to_string())?
            }
            "--investigateur" => setup.investigators.push(InvestigatorId::new(value)),
            "--grand-ancien" => setup.ancient_one = Some(AncientOneId::new(value)),
            "--parties" => games = number(&flag, &value)?,
            other => return Err(format!("option inconnue : {other}")),
        }
    }
    setup.players = setup.players.max(setup.investigators.len());
    match verb.as_str() {
        "jouer" => Ok(Command::Play(setup)),
        "masse" => Ok(Command::Bulk { games, setup }),
        "aide" | "--help" | "-h" => Ok(Command::Help),
        other => Err(format!("commande inconnue : {other}")),
    }
}

/// Parses a number.
fn number(flag: &str, value: &str) -> Result<u64, String> {
    value
        .parse()
        .map_err(|_| format!("{flag} attend un nombre, pas « {value} »"))
}

/// A seed from the clock, when none is given.
fn seed_from_clock() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| {
            u64::try_from(elapsed.as_nanos() % u128::from(u64::MAX)).unwrap_or(0)
        })
}
