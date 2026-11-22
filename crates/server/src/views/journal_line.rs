use engine::{Deck, DiceCount, Exhibit, Shown, View};

use crate::illustrations::{IllustrationSize, illustration_url};
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
    /// Monsters an investigator starts to fight.
    pub foes: Vec<FoughtMonster>,
}

/// A monster an investigator faces, with its illustrated portrait.
#[derive(Debug, Clone)]
pub struct FoughtMonster {
    /// Who faces it.
    pub who: String,
    /// The monster's name.
    pub name: String,
    /// Link to its portrait thumbnail.
    pub portrait: String,
}

/// A Mythos, location or gate card drawn, or an investigator card received.
#[derive(Debug, Clone)]
pub struct DrawnCard {
    /// Name, image and bubble.
    pub token: ImageToken,
    /// « Carte Mythe tirée par », « Carte reçue par »…
    pub action: &'static str,
    /// Who drew it (the first player for a Mythos card).
    pub who: String,
}

/// Dice rolled at once.
#[derive(Debug, Clone)]
pub struct DiceRoll {
    /// Who rolled.
    pub who: String,
    /// Each die.
    pub dice: Vec<Die>,
    /// What the roll gives: « 1 succès », « total : 7 »…
    pub summary: String,
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
            foes: Vec::new(),
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
            foes: Vec::new(),
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
    if lines.last().is_some_and(|line| {
        line.text.is_empty()
            && line.cards.is_empty()
            && line.rolls.is_empty()
            && line.foes.is_empty()
    }) {
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
            let action = match deck {
                Deck::Mythos => "Carte Mythe tirée par",
                Deck::Location => "Carte Lieu tirée par",
                Deck::Gate => "Carte Portail tirée par",
                Deck::Investigator => "Carte reçue par",
            };
            line.cards.push(DrawnCard {
                token: card(view.data(), id),
                action,
                who: view.name(*seat),
            });
        }
        Exhibit::Dice { seat, faces, count } => {
            line.rolls.push(dice_roll(view, *seat, faces, *count));
        }
        Exhibit::CombatStart { seat, foe } => {
            let data = view.data();
            if let Some(monster) = data.monsters.monsters.iter().find(|m| &m.id == foe) {
                line.foes.push(FoughtMonster {
                    who: view.name(*seat),
                    name: monster.name.clone(),
                    portrait: illustration_url(IllustrationSize::Thumbnail, &monster.portrait),
                });
            }
        }
        Exhibit::CombatEnd { .. } => {}
    }
}

/// A roll as shown: each die, and the successes or the total. A roll of no
/// dice is shown too.
fn dice_roll(view: &View<'_, '_>, seat: Option<usize>, faces: &[u8], count: DiceCount) -> DiceRoll {
    let who = seat.map_or_else(|| "Monstre".to_owned(), |seat| view.name(seat));
    let dice = faces
        .iter()
        .map(|&face| Die {
            face,
            successes: match count {
                DiceCount::Successes {
                    success_from,
                    sixes_double,
                } => u8::try_from(Exhibit::successes(&[face], success_from, sixes_double))
                    .unwrap_or(0),
                DiceCount::Total => 0,
            },
        })
        .collect();
    let total: u32 = faces.iter().map(|&face| u32::from(face)).sum();
    let summary = match count {
        _ if faces.is_empty() => "aucun dé : 0 succès".to_owned(),
        DiceCount::Successes {
            success_from,
            sixes_double,
        } => format!(
            "{} succès",
            Exhibit::successes(faces, success_from, sixes_double)
        ),
        DiceCount::Total if faces.len() == 1 => format!("résultat : {total}"),
        DiceCount::Total => format!("total : {total}"),
    };
    DiceRoll { who, dice, summary }
}
