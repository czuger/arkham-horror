use engine::{Deck, Exhibit, Shown, View};

use crate::views::image_token::ImageToken;
use crate::views::names::card;

/// One journal line, with the cards drawn and the dice rolled just before
/// it.
#[derive(Debug, Clone)]
pub struct JournalLine {
    /// The text (empty for cards or dice not followed by a line yet).
    pub text: String,
    /// Cards drawn.
    pub cards: Vec<DrawnCard>,
    /// Dice rolled.
    pub rolls: Vec<DiceRoll>,
}

/// A Mythos, location or gate card drawn.
#[derive(Debug, Clone)]
pub struct DrawnCard {
    /// Name, image and bubble.
    pub token: ImageToken,
    /// « Carte Mythe », « Carte Lieu tirée par Amanda Sharpe »…
    pub caption: String,
}

/// Dice rolled at once.
#[derive(Debug, Clone)]
pub struct DiceRoll {
    /// Who rolled.
    pub who: String,
    /// Each die.
    pub dice: Vec<Die>,
    /// Successes rolled.
    pub successes: u32,
}

/// One die rolled.
#[derive(Debug, Clone)]
pub struct Die {
    /// Face, from 1 to 6.
    pub face: u8,
    /// Successes it gives (0, 1, or 2 for a doubled six).
    pub successes: u8,
}

impl Die {
    /// The face as a die character, ⚀ to ⚅.
    pub fn glyph(&self) -> char {
        char::from_u32(0x2680 + u32::from(self.face.clamp(1, 6)) - 1).unwrap_or('?')
    }
}

/// The journal lines from `from` (included) to `to` (excluded), with what
/// was drawn and rolled before each. When `to` is the end of the journal,
/// what was drawn or rolled after the last line comes in a last line
/// without text.
pub fn journal_lines(
    view: &View<'_, '_>,
    events: &[String],
    shown: &[Shown],
    from: usize,
    to: usize,
) -> Vec<JournalLine> {
    let to = to.min(events.len());
    let mut lines: Vec<JournalLine> = events
        .get(from..to)
        .unwrap_or_default()
        .iter()
        .map(|text| JournalLine {
            text: text.clone(),
            cards: Vec::new(),
            rolls: Vec::new(),
        })
        .collect();
    let last = if to == events.len() {
        to
    } else {
        to.saturating_sub(1)
    };
    if to == events.len() {
        lines.push(JournalLine {
            text: String::new(),
            cards: Vec::new(),
            rolls: Vec::new(),
        });
    }
    for item in shown
        .iter()
        .filter(|item| (from..=last).contains(&item.line))
    {
        if let Some(line) = lines.get_mut(item.line - from) {
            attach(view, line, &item.what);
        }
    }
    if lines
        .last()
        .is_some_and(|line| line.text.is_empty() && line.cards.is_empty() && line.rolls.is_empty())
    {
        lines.pop();
    }
    lines
}

/// Adds a card drawn or a roll to a line.
fn attach(view: &View<'_, '_>, line: &mut JournalLine, what: &Exhibit) {
    match what {
        Exhibit::Card {
            deck,
            card: id,
            seat,
        } => {
            let deck = match deck {
                Deck::Mythos => "Carte Mythe",
                Deck::Location => "Carte Lieu",
                Deck::Gate => "Carte Portail",
            };
            let caption = match seat {
                Some(seat) => format!("{deck} tirée par {}", view.name(*seat)),
                None => deck.to_owned(),
            };
            line.cards.push(DrawnCard {
                token: card(view.data(), id),
                caption,
            });
        }
        Exhibit::Dice { faces, .. } if faces.is_empty() => {}
        Exhibit::Dice {
            seat,
            faces,
            success_from,
            sixes_double,
        } => line.rolls.push(DiceRoll {
            who: view.name(*seat),
            dice: faces
                .iter()
                .map(|&face| Die {
                    face,
                    successes: u8::try_from(Exhibit::successes(
                        &[face],
                        *success_from,
                        *sixes_double,
                    ))
                    .unwrap_or(0),
                })
                .collect(),
            successes: Exhibit::successes(faces, *success_from, *sixes_double),
        }),
        Exhibit::CombatStart { .. } | Exhibit::CombatEnd { .. } => {}
    }
}
