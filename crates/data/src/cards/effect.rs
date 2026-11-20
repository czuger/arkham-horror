//! The effect primitives cards are written with.
//!
//! An effect applies to the investigator who resolves the card, unless it says
//! otherwise (`EachInvestigator`, `MonstersInStreet`…). A card runs a list of
//! effects in order.

use serde::{Deserialize, Serialize};

use crate::cards::amount::Amount;
use crate::cards::card_filter::CardFilter;
use crate::cards::check_kind::CheckKind;
use crate::cards::condition::Condition;
use crate::cards::monster_area::MonsterArea;
use crate::cards::resource::Resource;
use crate::deck::Deck;
use crate::ids::{CardId, LocationId, MonsterId, OtherWorldId, SpecialId, StreetId};

/// One step of a card effect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Effect {
    /// Gagnez N.
    Gain(Resource, u8),
    /// Lancez un dé et gagnez autant.
    GainRolled(Resource),
    /// Gain a worked-out amount.
    GainAmount(Resource, Amount),
    /// N points split between Stamina and Sanity, as the investigator likes.
    GainSplit(Amount),
    /// Perdez N. Losing more than you have is allowed (you drop to zero). Two
    /// `Lose` in a row are suffered at the same time.
    Lose(Resource, u8),
    /// Lancez un dé et perdez autant.
    LoseRolled(Resource),
    /// Lose a worked-out amount of each resource listed, at the same time.
    LoseAmount(Vec<Resource>, Amount),
    /// Perdez tout (« tout votre argent »).
    LoseAll(Resource),
    /// Payez N, or dépensez N Indices. Only possible with the full amount, so
    /// an option starting with it is offered only when it can be paid.
    Pay(Resource, u8),
    /// Restore Sanity or Stamina to its maximum.
    RestoreToMax(Resource),
    /// Lower the maximum of Sanity or Stamina for the rest of the game.
    LowerMax(Resource, u8),
    /// Faites un test : `pass` on success, `fail` on failure.
    Check {
        /// The check made.
        skill: CheckKind,
        /// Modifier printed in brackets.
        modifier: i8,
        /// Successes needed, printed as `[2]`; 1 when not printed.
        #[serde(default = "one")]
        successes: u8,
        /// A held card that may be discarded to pass without rolling.
        #[serde(default)]
        or_discard: Option<CardId>,
        /// Effects when the check passes.
        #[serde(default)]
        pass: Vec<Effect>,
        /// Effects when the check fails.
        #[serde(default)]
        fail: Vec<Effect>,
    },
    /// Faites un test et consultez la table : the row with the highest
    /// `from` not above the successes rolled applies.
    CheckTable {
        /// The check made.
        skill: CheckKind,
        /// Modifier printed in brackets.
        modifier: i8,
        /// Rows, by number of successes.
        rows: Vec<TableRow>,
    },
    /// Lancez un dé : apply `effects` when the result is `at_most` or less.
    /// The value rolled is kept as [`Amount::X`].
    RollDie {
        /// Highest face that triggers the effects.
        at_most: u8,
        /// What happens on those faces.
        effects: Vec<Effect>,
    },
    /// Roll N dice; their total is kept as [`Amount::X`].
    RollDice(u8),
    /// Roll N dice; each success applies `success`, each failure `failure`.
    RollEach {
        /// Dice rolled.
        #[serde(default = "one")]
        dice: u8,
        /// Effects for each success.
        #[serde(default)]
        success: Vec<Effect>,
        /// Effects for each failure.
        #[serde(default)]
        failure: Vec<Effect>,
    },
    /// One die per point of a resource; one point lost per failure, all at
    /// once. Successes are kept as [`Amount::Successes`].
    RollPerPoint {
        /// Sanity or Stamina.
        resource: Resource,
        /// Dropping to 0 devours instead of knocking out.
        #[serde(default)]
        devoured_at_zero: bool,
    },
    /// The investigator picks a number from 0 to N, kept as [`Amount::X`].
    ChooseUpTo(u8),
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
    /// Prenez la première carte de ce genre du paquet (un Tome, une Arme…).
    TakeFirst {
        /// The deck searched from the top.
        deck: Deck,
        /// Which cards qualify.
        cards: CardFilter,
    },
    /// Prenez la carte de votre choix dans ce paquet.
    SearchDeck(Deck),
    /// Look at the top card of each deck; buy any of them at the printed price.
    Shop(Vec<Deck>),
    /// Draw one card per deck listed; buy any of them at `price`; the rest
    /// is discarded.
    Buy {
        /// One card is drawn from each entry.
        draw: Vec<Deck>,
        /// Price asked.
        #[serde(default)]
        price: Price,
        /// Every investigator may buy, the active one first.
        #[serde(default)]
        everyone: bool,
    },
    /// Search one of these decks for any card and buy it at its price.
    BuyAny(Vec<Deck>),
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
    /// Défaussez la moitié de ces cartes, au choix.
    DiscardHalf {
        /// Which cards.
        cards: CardFilter,
        /// Rounded up instead of down.
        #[serde(default)]
        round_up: bool,
    },
    /// Discard one held copy of this card, if any.
    DiscardCard(CardId),
    /// Discard the card that carries this effect.
    DiscardThis,
    /// Put the card that carries this effect back in the box for good.
    ReturnThisToBox,
    /// Put a random card of this deck back in the box for good.
    ReturnRandomCardToBox(Deck),
    /// Allez dans la rue : move to the street area of the current district.
    MoveToStreet,
    /// Move to this location (no monster faced, no clue picked up).
    MoveTo(LocationId),
    /// Move to any location or street of Arkham, chosen by the investigator;
    /// on a location, have an encounter there at once.
    MoveAnywhereInArkham,
    /// Draw a Mythos card and move to its gate location (R22).
    MoveToMythosGate,
    /// Have an Arkham encounter where you stand: draw `draw` cards and pick one.
    EncounterHere {
        /// Location cards drawn.
        #[serde(default = "one")]
        draw: u8,
    },
    /// Draw cards again for the encounter being resolved (Œil du Dragon).
    RedrawEncounter,
    /// Have an encounter in this Other World, then come straight back.
    VisitOtherWorld(OtherWorldId),
    /// Return to Arkham from an Other World, as at the end of an exploration.
    ReturnToArkham,
    /// From the first area of an Other World to the second; from the second,
    /// back to Arkham.
    AdvanceInOtherWorld,
    /// Arrêté : taken to the Police Station jail cell.
    Arrested,
    /// Perdu dans le temps et l'espace.
    LostInTimeAndSpace,
    /// Vous êtes dévoré : the investigator is out and the player starts
    /// again with a new one.
    Devoured,
    /// Restez ici au prochain tour : the investigator is delayed.
    Delayed,
    /// Perdez votre prochain tour.
    LoseNextTurn,
    /// Movement points added to the current movement.
    GainMovementPoints(u8),
    /// +N to every skill check until the end of the turn.
    BonusUntilEndOfTurn(i8),
    /// Un monstre apparaît : fight or evade a monster drawn from the cup.
    MonsterAppears,
    /// Un portail et un monstre apparaissent, here.
    GateAndMonsterAppear,
    /// Un portail s'ouvre ici et vous aspire, without a monster (R20).
    GateAppears,
    /// Monsters drawn from the cup are placed in a street area.
    MonstersInStreet {
        /// The street area.
        street: StreetId,
        /// Number of monsters.
        count: u8,
    },
    /// Monsters of an area go back to the cup.
    ReturnMonsters {
        /// Where.
        area: MonsterArea,
        /// Only these kinds; all of them when empty.
        #[serde(default)]
        only: Vec<MonsterId>,
        /// The Terror level rises by 1 if at least one monster went back.
        #[serde(default)]
        terror_if_any: bool,
    },
    /// Take a monster from the cup as a trophy, Endless ones included.
    TrophyFromCup,
    /// Take a monster of the board as a trophy.
    TrophyFromBoard,
    /// Spend monster trophies worth this much toughness. A cost.
    SpendMonsterTrophies(u8),
    /// Spend this many gate trophies. A cost.
    SpendGateTrophies(u8),
    /// Close the gate the investigator went through, without a trophy.
    CloseEntryGate,
    /// Close a gate of the investigator's choice, without a trophy.
    CloseGateOfChoice,
    /// Add a doom token to the Ancient One's track.
    AddDoom,
    /// Remove a doom token from the track.
    RemoveDoom,
    /// The Terror level rises by N.
    RaiseTerror(u8),
    /// The Terror level rises to N.
    TerrorTo(u8),
    /// Every Elder Sign token leaves the board.
    RemoveAllElderSigns,
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
    /// Applies the effects to one investigator in the same area, the
    /// investigator resolving the card included, at their choice.
    OneInvestigatorHere(Vec<Effect>),
    /// « L'investigateur de votre choix » : one investigator anywhere.
    AnyInvestigator(Vec<Effect>),
    /// Put N tokens on the card that carries this effect (Rumeur, objet).
    AddTokensToThis(u8),
    /// Remove N tokens from the card that carries this effect.
    RemoveTokensFromThis(u8),
    /// Put N monsters from the cup on the Rumor in play.
    AddMonstersToThis(u8),
    /// The Rumor in play passes.
    RumorPasses,
    /// Bonus to Combat checks until the end of the current combat.
    CombatBonus(i8),
    /// Reduce the loss being suffered by N (reaction).
    ReduceLoss(u8),
    /// Cancel the loss being suffered (reaction).
    CancelLoss,
    /// The check about to be made passes without rolling (reaction).
    PassCheck,
    /// The monster's combat damage of this round drops to 0 (reaction).
    NoCombatDamage,
    /// Re-roll the check just made (reaction).
    Reroll,
    /// Re-roll one die of the check just made (reaction).
    RerollOneDie,
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

/// One row of a [`Effect::CheckTable`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableRow {
    /// Fewest successes for this row.
    pub from: u8,
    /// What happens.
    #[serde(default)]
    pub effects: Vec<Effect>,
}

/// Price asked by a [`Effect::Buy`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Price {
    /// The printed price.
    #[default]
    Printed,
    /// N dollars more.
    Plus(u8),
    /// N dollars less (never below 0).
    Minus(u8),
    /// Half the price, rounded up.
    Half,
}

/// Serde default for counts printed only when they differ from 1.
fn one() -> u8 {
    1
}
