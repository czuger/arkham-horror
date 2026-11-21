//! The running game: data, state, responder and journal together.

use data::{
    AncientOne, CardId, GameData, Investigator, InvestigatorCard, Location, LocationId, Monster,
    MonsterId, OtherWorld, OtherWorldId, Street, StreetId,
};

use crate::decision::{Choice, Context, Decision, DecisionKind, DecisionOption, Responder};
use crate::halt::{Flow, Halt};
use crate::state::{GameState, InvestigatorState, Place};
use crate::view::View;

/// A game being played. Rule code lives in `impl Game` blocks under `rules/`.
pub(crate) struct Game<'a> {
    /// The game data.
    pub data: &'a GameData,
    /// The state.
    pub state: GameState,
    /// Who answers decisions; taken out while it answers.
    responder: Option<&'a mut dyn Responder>,
    /// Answers accepted so far, in order.
    pub answers: Vec<usize>,
    /// Journal lines, in French.
    pub events: Vec<String>,
    /// Short-lived values passed between rule steps (not part of the state).
    pub scratch: Scratch,
    /// Cards and paragraphs played so far (see [`crate::coverage`]).
    pub covered: std::collections::BTreeSet<String>,
    /// Counts of notable events.
    pub tally: crate::tally::Tally,
    /// Dice results forced by unit tests, used before the generator.
    #[cfg(test)]
    pub loaded_dice: std::collections::VecDeque<u8>,
}

/// Values that live only while one rule step runs.
#[derive(Debug, Clone, Default)]
pub(crate) struct Scratch {
    /// Movement points left in the current movement.
    pub movement_points: u32,
    /// Loss reduction granted by the last ability used.
    pub reduce_loss: u32,
    /// Combat bonus granted by the last ability used.
    pub combat_bonus: i32,
    /// The last ability used asks for a reroll.
    pub reroll: bool,
    /// Care that ignores « no Stamina/Sanity gain » (Asylum, Hospital,
    /// Carolyn Fern, Vincent Lee) is being given.
    pub care: bool,
    /// Each 6 rolled counts as 2 successes (Shotgun in hand).
    pub sixes_double: bool,
    /// The Flute was played: every monster in the area is defeated.
    pub flute: bool,
    /// The last ability used rerolls one die.
    pub reroll_one_die: bool,
    /// The last ability used makes the coming check pass.
    pub pass_check: bool,
    /// The last ability used cancels the monster's combat damage this round.
    pub no_combat_damage: bool,
    /// The last spell cast enchants a physical weapon.
    pub enchant_weapon: bool,
    /// The last spell cast weakens the monster fought (the Red Sign).
    pub red_sign: bool,
    /// Monsters of a Rumor that just ended, for its last effects.
    pub rumor_monsters: Vec<data::MonsterId>,
    /// A gate closing check is being made.
    pub closing_gate: bool,
    /// The encounter being resolved is to be drawn again.
    pub redraw: bool,
    /// Value rolled or chosen, read by [`data::Amount::X`].
    pub x: u32,
    /// Successes of the last check or roll, read by [`data::Amount::Successes`].
    pub successes: u32,
}

impl<'a> Game<'a> {
    /// Wraps a state.
    pub fn new(data: &'a GameData, state: GameState, responder: &'a mut dyn Responder) -> Self {
        Self {
            data,
            state,
            responder: Some(responder),
            answers: Vec::new(),
            events: Vec::new(),
            scratch: Scratch::default(),
            covered: std::collections::BTreeSet::new(),
            tally: crate::tally::Tally::default(),
            #[cfg(test)]
            loaded_dice: std::collections::VecDeque::new(),
        }
    }

    /// Rolls one die.
    pub fn die(&mut self) -> u8 {
        #[cfg(test)]
        if let Some(value) = self.loaded_dice.pop_front() {
            return value;
        }
        self.state.rng.die()
    }

    /// Adds a line to the journal.
    pub fn log(&mut self, line: impl Into<String>) {
        self.events.push(line.into());
    }

    /// Asks a decision and returns the chosen option. A single option is
    /// taken without asking.
    pub fn ask(
        &mut self,
        who: usize,
        kind: DecisionKind,
        prompt: impl Into<String>,
        options: Vec<DecisionOption>,
    ) -> Flow<Choice> {
        self.ask_about(who, kind, Context::None, prompt, options)
    }

    /// Asks a decision, with facts about what it is about.
    pub fn ask_about(
        &mut self,
        who: usize,
        kind: DecisionKind,
        context: Context,
        prompt: impl Into<String>,
        options: Vec<DecisionOption>,
    ) -> Flow<Choice> {
        match options.len() {
            0 => return Err(Halt::Broken("décision sans option".to_owned())),
            1 => return Ok(options[0].choice.clone()),
            _ => {}
        }
        let decision = Decision {
            who,
            kind,
            prompt: prompt.into(),
            options,
            context,
        };
        let Some(responder) = self.responder.take() else {
            return Err(Halt::Broken(
                "réponse demandée pendant une réponse".to_owned(),
            ));
        };
        let answer = responder.answer(&View::new(self), &decision);
        self.responder = Some(responder);
        let Some(answer) = answer else {
            return Err(Halt::Waiting(decision));
        };
        let Some(option) = decision.options.get(answer) else {
            return Err(Halt::Illegal {
                position: self.answers.len(),
                answer,
                decision: Box::new(decision),
            });
        };
        let choice = option.choice.clone();
        self.answers.push(answer);
        Ok(choice)
    }

    /// Asks a yes/no question.
    pub fn ask_yes_no(
        &mut self,
        who: usize,
        kind: DecisionKind,
        prompt: impl Into<String>,
    ) -> Flow<bool> {
        let options = vec![option(Choice::Yes, "Oui"), option(Choice::Done, "Non")];
        Ok(self.ask(who, kind, prompt, options)? == Choice::Yes)
    }

    /// Asks whether to apply some card effects.
    pub fn ask_yes_with(
        &mut self,
        who: usize,
        prompt: impl Into<String>,
        effects: &[data::Effect],
    ) -> Flow<bool> {
        let options = vec![
            option_with(Choice::Yes, "Oui", effects),
            option(Choice::Done, "Non"),
        ];
        Ok(self.ask(who, DecisionKind::Optional, prompt, options)? == Choice::Yes)
    }

    /// The investigator at a seat.
    pub fn inv(&self, seat: usize) -> &InvestigatorState {
        &self.state.investigators[seat]
    }

    /// The investigator at a seat, to change it.
    pub fn inv_mut(&mut self, seat: usize) -> &mut InvestigatorState {
        &mut self.state.investigators[seat]
    }

    /// The sheet of the investigator at a seat.
    pub fn sheet(&self, seat: usize) -> Flow<&'a Investigator> {
        let data: &'a GameData = self.data;
        let id = &self.state.investigators[seat].id;
        data.investigators
            .investigators
            .iter()
            .find(|sheet| &sheet.id == id)
            .ok_or_else(|| broken("investigateur", id.as_str()))
    }

    /// The investigator's name.
    pub fn name(&self, seat: usize) -> String {
        self.sheet(seat).map_or_else(
            |_| self.inv(seat).id.to_string(),
            |sheet| sheet.name.clone(),
        )
    }

    /// A location.
    pub fn location(&self, id: &LocationId) -> Flow<&'a Location> {
        let data: &'a GameData = self.data;
        data.board
            .location(id)
            .ok_or_else(|| broken("lieu", id.as_str()))
    }

    /// A street area.
    pub fn street(&self, id: &StreetId) -> Flow<&'a Street> {
        let data: &'a GameData = self.data;
        data.board
            .street(id)
            .ok_or_else(|| broken("rue", id.as_str()))
    }

    /// An Other World.
    pub fn world(&self, id: &OtherWorldId) -> Flow<&'a OtherWorld> {
        let data: &'a GameData = self.data;
        data.board
            .other_world(id)
            .ok_or_else(|| broken("Autre Monde", id.as_str()))
    }

    /// A monster.
    pub fn monster(&self, id: &MonsterId) -> Flow<&'a Monster> {
        let data: &'a GameData = self.data;
        data.monsters
            .monsters
            .iter()
            .find(|monster| &monster.id == id)
            .ok_or_else(|| broken("monstre", id.as_str()))
    }

    /// An investigator card.
    pub fn card(&self, id: &CardId) -> Flow<&'a InvestigatorCard> {
        let data: &'a GameData = self.data;
        data.investigator_cards
            .card(id)
            .ok_or_else(|| broken("carte", id.as_str()))
    }

    /// The Ancient One of the game.
    pub fn ancient_one(&self) -> Flow<&'a AncientOne> {
        let data: &'a GameData = self.data;
        let id = &self.state.ancient_one;
        data.ancient_ones
            .ancient_ones
            .iter()
            .find(|one| &one.id == id)
            .ok_or_else(|| broken("Grand Ancien", id.as_str()))
    }

    /// French name of a place.
    pub fn place_name(&self, place: &Place) -> String {
        match place {
            Place::Location(id) => self
                .location(id)
                .map_or_else(|_| id.to_string(), |l| l.name.clone()),
            Place::Street(id) => self
                .street(id)
                .map_or_else(|_| id.to_string(), |s| format!("rues de {}", s.name)),
            Place::OtherWorld { world, .. } => self
                .world(world)
                .map_or_else(|_| world.to_string(), |w| w.name.clone()),
            Place::LostInTimeAndSpace => "Perdu dans le Temps et l'Espace".to_owned(),
            Place::JailCell => "Cellule de Prison".to_owned(),
        }
    }

    /// French name of a monster.
    pub fn monster_name(&self, id: &MonsterId) -> String {
        self.monster(id)
            .map_or_else(|_| id.to_string(), |m| m.name.clone())
    }

    /// French name of a card.
    pub fn card_name(&self, id: &CardId) -> String {
        self.card(id)
            .map_or_else(|_| id.to_string(), |c| c.name.clone())
    }
}

/// Builds a decision option.
pub(crate) fn option(choice: Choice, label: impl Into<String>) -> DecisionOption {
    DecisionOption {
        choice,
        label: label.into(),
        effects: Vec::new(),
    }
}

/// Builds a decision option that leads to card effects.
pub(crate) fn option_with(
    choice: Choice,
    label: impl Into<String>,
    effects: &[data::Effect],
) -> DecisionOption {
    DecisionOption {
        choice,
        label: label.into(),
        effects: effects.to_vec(),
    }
}

/// A missing data entry: the data and the code disagree.
fn broken(family: &str, id: &str) -> Halt {
    Halt::Broken(format!("{family} « {id} » introuvable"))
}
