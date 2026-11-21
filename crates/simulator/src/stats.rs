//! Statistics over many games: endings, length and what happened.

use std::collections::BTreeMap;

use engine::{Outcome, Status, Tally};

/// Totals over the games played.
#[derive(Default)]
pub struct Stats {
    /// Games by ending label.
    endings: BTreeMap<&'static str, u64>,
    /// (games, victories) by Ancient One.
    by_ancient_one: BTreeMap<String, (u64, u64)>,
    /// Games where the Ancient One awoke, by reason.
    awakenings: BTreeMap<&'static str, u64>,
    /// Games counted.
    games: u64,
    /// Turns over all games.
    turns: u64,
    /// Decisions over all games.
    decisions: u64,
    /// Summed tallies.
    tally: Tally,
}

impl Stats {
    /// Counts one finished game.
    pub fn add(&mut self, outcome: &Outcome) {
        let victory = match outcome.status {
            Status::Over(ending) => {
                *self.endings.entry(ending.label()).or_insert(0) += 1;
                ending.is_victory()
            }
            Status::Waiting(_) => {
                *self.endings.entry("en attente (anormal)").or_insert(0) += 1;
                false
            }
        };
        let entry = self
            .by_ancient_one
            .entry(outcome.state.ancient_one.to_string())
            .or_insert((0, 0));
        entry.0 += 1;
        entry.1 += u64::from(victory);
        if let Some(reason) = outcome.tally.awakening {
            *self.awakenings.entry(reason).or_insert(0) += 1;
        }
        self.games += 1;
        self.turns += u64::from(outcome.state.turn);
        self.decisions += u64::try_from(outcome.answers.len()).unwrap_or(0);
        let t = &mut self.tally;
        let o = &outcome.tally;
        t.knocked_out += o.knocked_out;
        t.devoured += o.devoured;
        t.lost_in_time += o.lost_in_time;
        t.monsters_defeated += o.monsters_defeated;
        t.gates_closed += o.gates_closed;
        t.gates_sealed += o.gates_sealed;
        t.gates_opened += o.gates_opened;
    }

    /// Prints the summary, in French.
    pub fn print(&self) {
        let games = self.games.max(1) as f64;
        let victories: u64 = self.by_ancient_one.values().map(|(_, won)| won).sum();
        for (label, count) in &self.endings {
            println!("  {count:>6}  {label}");
        }
        let awakened: u64 = self.awakenings.values().sum();
        println!(
            "Victoires : {:.1} %. Réveil du Grand Ancien : {:.1} %.",
            percent(victories, self.games),
            percent(awakened, self.games)
        );
        for (reason, count) in &self.awakenings {
            println!(
                "  {:>5.1} %  réveil : {reason}",
                percent(*count, self.games)
            );
        }
        println!(
            "Durée moyenne : {:.1} tours, {:.0} décisions.",
            self.turns as f64 / games,
            self.decisions as f64 / games
        );
        let t = &self.tally;
        let per_game = |count: u32| f64::from(count) / games;
        println!(
            "Par partie : {:.1} portails ouverts, {:.1} fermés dont {:.1} scellés, {:.1} monstres vaincus.",
            per_game(t.gates_opened),
            per_game(t.gates_closed),
            per_game(t.gates_sealed),
            per_game(t.monsters_defeated)
        );
        println!(
            "Par partie : {:.2} K.-O. ou folies, {:.2} perdus dans le temps, {:.2} dévorés.",
            per_game(t.knocked_out),
            per_game(t.lost_in_time),
            per_game(t.devoured)
        );
        println!("Victoires par Grand Ancien :");
        for (one, (played, won)) in &self.by_ancient_one {
            println!(
                "  {one:<14} {:>5.1} %  ({won} / {played})",
                percent(*won, *played)
            );
        }
    }
}

/// A share in percent.
fn percent(part: u64, whole: u64) -> f64 {
    if whole == 0 {
        0.0
    } else {
        part as f64 * 100.0 / whole as f64
    }
}
