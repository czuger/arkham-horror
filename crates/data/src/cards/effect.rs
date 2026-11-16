//! The effect primitives cards are written with.
//!
//! An effect applies to the investigator who resolves the card, unless it says
//! otherwise (`EachInvestigator`, `MonstersInStreet`…). A card runs a list of
//! effects in order.

use serde::{Deserialize, Serialize};

use crate::cards::card_filter::CardFilter;
use crate::cards::check_kind::CheckKind;
use crate::cards::condition::Condition;
use crate::cards::resource::Resource;
use crate::deck::Deck;
use crate::ids::{CardId, SpecialId, StreetId};

/// One step of a card effect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Effect {
    /// Gagnez N.
    Gain(Resource, u8),
    /// Lancez un dé et gagnez autant.
    GainRolled(Resource),
    /// Perdez N. Losing more than you have is allowed (you drop to zero).
    Lose(Resource, u8),
    /// Lancez un dé et perdez autant.
    LoseRolled(Resource),
    /// Perdez tout (« tout votre argent »).
    LoseAll(Resource),
    /// Payez N, or dépensez N Indices. Only possible with the full amount, so
    /// an option starting with it is offered only when it can be paid.
    Pay(Resource, u8),
    /// Restore Sanity or Stamina to its maximum.
    RestoreToMax(Resource),
    /// Faites un test : `pass` on success, `fail` on failure.
    Check {
        /// The check made.
        skill: CheckKind,
        /// Modifier printed in brackets.
        modifier: i8,
        /// Successes needed, printed as `[2]`; 1 when not printed.
        #[serde(default = "one")]
        successes: u8,
        /// Effects when the check passes.
        #[serde(default)]
        pass: Vec<Effect>,
        /// Effects when the check fails.
        #[serde(default)]
        fail: Vec<Effect>,
    },
    /// Lancez un dé : apply `effects` when the result is `at_most` or less.
    RollDie {
        /// Highest face that triggers the effects.
        at_most: u8,
        /// What happens on those faces.
        effects: Vec<Effect>,
    },
    /// Piochez : draw cards from a deck and keep some of them.
    Draw {
        /// The deck drawn from.
        deck: Deck,
        /// Cards drawn.
        #[serde(default = "one")]
        count: u8,
        /// Cards kept, chosen by the investigator; all of them when absent.
        #[serde(default)]
        keep: Option<u8>,
    },
    /// Take this named card from its deck; `otherwise` if nobody can give it.
    TakeCard {
        /// The card taken.
        card: CardId,
        /// What happens when the card is not available.
        #[serde(default)]
        otherwise: Vec<Effect>,
    },
    /// Prenez la carte de votre choix dans ce paquet.
    SearchDeck(Deck),
    /// Look at the top card of each deck; buy any of them at the printed price.
    Shop(Vec<Deck>),
    /// You may sell one of your cards of this deck at `multiplier` × its price.
    Sell {
        /// The deck the card belongs to.
        deck: Deck,
        /// Price multiplier.
        multiplier: u8,
    },
    /// Défaussez N cartes de votre choix.
    Discard {
        /// Which cards.
        cards: CardFilter,
        /// How many.
        count: u8,
    },
    /// Défaussez toutes ces cartes.
    DiscardAll(CardFilter),
    /// Discard the card that carries this effect.
    DiscardThis,
    /// Put the card that carries this effect back in the box for good.
    ReturnThisToBox,
    /// Allez dans la rue : move to the street area of the current district.
    MoveToStreet,
    /// Move to any location or street of Arkham, chosen by the investigator.
    MoveAnywhereInArkham,
    /// Return to Arkham from an Other World, as at the end of an exploration.
    ReturnToArkham,
    /// Arrêté : taken to the Police Station jail cell.
    Arrested,
    /// Perdu dans le temps et l'espace.
    LostInTimeAndSpace,
    /// Restez ici au prochain tour : the investigator is delayed.
    Delayed,
    /// Perdez votre prochain tour.
    LoseNextTurn,
    /// Un monstre apparaît : fight or evade a monster drawn from the cup.
    MonsterAppears,
    /// Un portail et un monstre apparaissent, here.
    GateAndMonsterAppear,
    /// Monsters drawn from the cup are placed in a street area.
    MonstersInStreet {
        /// The street area.
        street: StreetId,
        /// Number of monsters.
        count: u8,
    },
    /// The investigator must pick one option among those possible; nothing
    /// happens if none is (« si possible »).
    Choice(Vec<ChoiceOption>),
    /// Vous pouvez : the investigator may apply the effects, or skip them.
    Optional(ChoiceOption),
    /// Applies `then` or `otherwise` depending on a condition.
    If {
        /// The condition tested.
        condition: Condition,
        /// Effects when it holds.
        then: Vec<Effect>,
        /// Effects when it does not.
        #[serde(default)]
        otherwise: Vec<Effect>,
    },
    /// Applies the effects to every investigator matching `only`, or to all.
    EachInvestigator {
        /// Filter on investigators.
        #[serde(default)]
        only: Option<Condition>,
        /// Effects applied to each of them.
        effects: Vec<Effect>,
    },
    /// Put N tokens on the card that carries this effect (Rumeurs).
    AddTokensToThis(u8),
    /// Bonus to Combat checks until the end of the current combat.
    CombatBonus(i8),
    /// Reduce the loss being suffered by N (reaction).
    ReduceLoss(u8),
    /// Re-roll the check just made (reaction).
    Reroll,
    /// Effect coded by name in the engine.
    Special(SpecialId),
}

/// One labelled branch of a [`Effect::Choice`] or [`Effect::Optional`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChoiceOption {
    /// What the player sees, in French.
    pub label: String,
    /// Effects applied when this option is taken.
    pub effects: Vec<Effect>,
}

/// Serde default for counts printed only when they differ from 1.
fn one() -> u8 {
    1
}
