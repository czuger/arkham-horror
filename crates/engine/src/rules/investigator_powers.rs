//! Abilities printed on the investigator sheets, coded by investigator id.

use data::{CardId, Deck, LocationId, Resource};

use crate::decision::{Choice, DecisionKind};
use crate::game::{Game, option};
use crate::halt::Flow;
use crate::state::Place;

/// Ashcan Pete: may draw from the bottom of the item and spell decks.
const ASHCAN_PETE: &str = "ashcan_pete";
/// Carolyn Fern: restores 1 Sanity at Upkeep.
const CAROLYN_FERN: &str = "carolyn_fern";
/// Darrell Simmons: draws one more location card and picks one.
const DARRELL_SIMMONS: &str = "darrell_simmons";
/// Gloria Goldberg: draws one more gate card and picks one.
const GLORIA_GOLDBERG: &str = "gloria_goldberg";
/// Harvey Walters: every Sanity loss is reduced by 1.
const HARVEY_WALTERS: &str = "harvey_walters";
/// Jenny Barnes: gains $1 at Upkeep.
const JENNY_BARNES: &str = "jenny_barnes";
/// Joe Diamond: one more die per clue token.
const JOE_DIAMOND: &str = "joe_diamond";
/// Kate Winthrop: no monster nor gate appears where she stands.
const KATE_WINTHROP: &str = "kate_winthrop";
/// Mandy Thompson: once per turn, a failed check may reroll its failed dice.
const MANDY_THOMPSON: &str = "mandy_thompson";
/// Michael McGlen: every Stamina loss is reduced by 1.
const MICHAEL_MCGLEN: &str = "michael_mcglen";
/// Sister Mary: never lost in time and space.
pub(crate) const SISTER_MARY: &str = "sister_mary";
/// Vincent Lee: restores 1 Stamina at Upkeep.
const VINCENT_LEE: &str = "vincent_lee";
/// Where Sister Mary goes when she would be lost with Sanity and Stamina left.
const SOUTH_CHURCH: &str = "south_church";
/// Kate's home, which gets no clue token at setup.
const SCIENCE_BUILDING: &str = "science_building";

/// Who draws one more card from which deck, then discards one.
const EXTRA_DRAWS: [(&str, Deck); 4] = [
    ("amanda_sharpe", Deck::Skill),
    ("bob_jenkins", Deck::CommonItem),
    ("dexter_drake", Deck::Spell),
    ("monterey_jack", Deck::UniqueItem),
];

impl Game<'_> {
    /// Whether the investigator in a seat plays this sheet.
    pub fn is(&self, seat: usize, id: &str) -> bool {
        self.inv(seat).id.as_str() == id
    }

    /// Seat of an investigator still in the game.
    fn seat_of(&self, id: &str) -> Option<usize> {
        self.state
            .seat_order()
            .into_iter()
            .find(|&seat| self.is(seat, id))
    }

    /// Cards drawn on top of the usual count (Amanda, Bob, Dexter, Monterey).
    pub fn draw_bonus(&self, seat: usize, deck: Deck) -> u8 {
        u8::from(
            EXTRA_DRAWS
                .iter()
                .any(|&(id, bonus_deck)| bonus_deck == deck && self.is(seat, id)),
        )
    }

    /// Draws one card for an investigator. Ashcan Pete may take the bottom
    /// card of the item and spell decks instead of the top one.
    pub fn draw_for(&mut self, seat: usize, deck: Deck) -> Flow<Option<CardId>> {
        let scrounge = self.is(seat, ASHCAN_PETE)
            && matches!(deck, Deck::CommonItem | Deck::UniqueItem | Deck::Spell);
        let cards = self.state.decks.get(deck);
        let bottom = cards.last().cloned();
        let (true, true, Some(bottom)) = (scrounge, cards.len() > 1, bottom) else {
            return Ok(self.draw_from(deck));
        };
        let options = vec![
            option(Choice::Done, "Prendre la carte du dessus"),
            option(
                Choice::Card(bottom.clone()),
                format!("Prendre la carte du dessous ({})", self.card_name(&bottom)),
            ),
        ];
        let prompt = "Pique-Assiette : dessus ou dessous du paquet ?";
        if let Choice::Card(_) = self.ask(seat, DecisionKind::PickCard, prompt, options)? {
            return Ok(self.state.decks.get_mut(deck).pop());
        }
        Ok(self.draw_from(deck))
    }

    /// Location cards drawn for an encounter: Darrell draws one more (two
    /// at least) and picks one.
    pub fn location_draws(&self, seat: usize, draw: u8) -> u8 {
        if self.is(seat, DARRELL_SIMMONS) {
            draw.max(1) + 1
        } else {
            draw.max(1)
        }
    }

    /// Gate cards drawn for an Other World encounter: two for Gloria.
    pub fn gate_draws(&self, seat: usize) -> usize {
        if self.is(seat, GLORIA_GOLDBERG) { 2 } else { 1 }
    }

    /// Loss reduction printed on the sheet (Harvey: Sanity, Michael: Stamina).
    pub fn sheet_reduction(&self, seat: usize, resource: Resource) -> u32 {
        let tough = match resource {
            Resource::Sanity => self.is(seat, HARVEY_WALTERS),
            Resource::Stamina => self.is(seat, MICHAEL_MCGLEN),
            _ => false,
        };
        u32::from(tough)
    }

    /// Dice per clue token added by the sheet (Joe Diamond).
    pub fn sheet_clue_dice(&self, seat: usize) -> u32 {
        u32::from(self.is(seat, JOE_DIAMOND))
    }

    /// Whether Kate Winthrop's flux stabilizer protects a place.
    pub fn flux_stabilized(&self, place: &Place) -> bool {
        self.seat_of(KATE_WINTHROP)
            .is_some_and(|seat| &self.inv(seat).place == place)
    }

    /// Whether Kate Winthrop protects a location.
    pub fn stabilized_location(&self, location: &LocationId) -> bool {
        self.flux_stabilized(&Place::Location(location.clone()))
    }

    /// Setup: no clue token on the Science Building when Kate plays.
    pub fn kate_setup(&mut self) {
        if self.seat_of(KATE_WINTHROP).is_some() {
            self.state.clues.remove(&LocationId::new(SCIENCE_BUILDING));
        }
    }

    /// Upkeep abilities: Jenny's dollar, Carolyn's and Vincent's care.
    pub fn sheet_upkeep(&mut self, seat: usize) -> Flow<()> {
        if self.is(seat, JENNY_BARNES) {
            self.gain(seat, Resource::Money, 1)?;
        }
        if self.is(seat, CAROLYN_FERN) {
            self.restore_one(seat, Resource::Sanity)?;
        }
        if self.is(seat, VINCENT_LEE) {
            self.restore_one(seat, Resource::Stamina)?;
        }
        Ok(())
    }

    /// Restores 1 point to the healer or someone at the same place, never
    /// above the maximum.
    fn restore_one(&mut self, seat: usize, resource: Resource) -> Flow<()> {
        let mut options = vec![option(Choice::Done, "Personne")];
        for other in self.investigators_at(&self.inv(seat).place.clone()) {
            if self.amount(other, resource) < u32::from(self.maximum(other, resource)?) {
                options.push(option(Choice::Investigator(other), self.name(other)));
            }
        }
        let prompt = "Qui soignez-vous de 1 point ?";
        let Choice::Investigator(target) =
            self.ask(seat, DecisionKind::Optional, prompt, options)?
        else {
            return Ok(());
        };
        self.scratch.care = true;
        let healed = self.gain(target, resource, 1);
        self.scratch.care = false;
        healed
    }

    /// Mandy Thompson, once per turn, after a failed check: whether the
    /// investigator may reroll the failed dice.
    pub fn mandy_research(&mut self, seat: usize) -> Flow<bool> {
        let Some(mandy) = self.seat_of(MANDY_THOMPSON) else {
            return Ok(false);
        };
        if self.inv(mandy).power_used {
            return Ok(false);
        }
        let prompt = format!("Recherche : {} relance ses dés ratés ?", self.name(seat));
        if !self.ask_yes_no(mandy, DecisionKind::Optional, prompt)? {
            return Ok(false);
        }
        self.inv_mut(mandy).power_used = true;
        self.log(format!("{} utilise Recherche.", self.name(mandy)));
        Ok(true)
    }

    /// Sister Mary instead of getting lost: the Asylum at 0 Sanity, the
    /// Hospital at 0 Stamina, otherwise South Church. Returns whether she
    /// was moved.
    pub fn guardian_angel(&mut self, seat: usize) -> Flow<bool> {
        if !self.is(seat, SISTER_MARY) {
            return Ok(false);
        }
        let inv = self.inv(seat);
        let target = if inv.sanity == 0 {
            crate::rules::status::ASYLUM
        } else if inv.stamina == 0 {
            crate::rules::status::HOSPITAL
        } else {
            SOUTH_CHURCH
        };
        let target = LocationId::new(target);
        self.log(format!(
            "Ange Gardien : {} n'est pas perdue et va à {}.",
            self.name(seat),
            self.location(&target)?.name
        ));
        self.move_to(seat, Place::Location(target));
        Ok(true)
    }
}
