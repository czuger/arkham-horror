//! The text of the hover bubbles.

use data::{
    AncientOne, Check, CombatRating, GameData, InvestigatorCard, Monster, MythosCard, Resource,
};
use engine::{MonsterStats, View};

use crate::views::names::{card, investigator_name, sheet};
use crate::views::tip::Tip;

/// A monster's values as they stand now, with its abilities and special text.
pub fn monster(view: &View<'_, '_>, monster: &Monster) -> Tip {
    let stats = view
        .monster_stats(&monster.id)
        .unwrap_or_else(|| printed(monster));
    let abilities: Vec<String> = stats.abilities.iter().map(|a| a.label()).collect();
    Tip::new(monster.name.clone())
        .line(format!(
            "Mouvement {} · Conscience {:+}",
            stats.movement.label(),
            stats.awareness
        ))
        .line(check_line("Horreur", stats.horror, "Santé Mentale"))
        .line(check_line("Combat", stats.combat, "Résistance"))
        .line(format!("Force {}", stats.toughness))
        .maybe(Some(abilities.join(", ")))
        .maybe(monster.special.clone())
}

/// The values printed on a token, when the engine cannot tell the current ones.
fn printed(monster: &Monster) -> MonsterStats {
    MonsterStats {
        awareness: i32::from(monster.awareness),
        horror: monster.horror,
        combat: monster.combat,
        toughness: u32::from(monster.toughness),
        abilities: monster.abilities.clone(),
        movement: monster.movement,
    }
}

/// « Horreur -1 · perte 2 Santé Mentale ».
fn check_line(name: &str, check: Option<Check>, resource: &str) -> String {
    match check {
        None => format!("Pas de test de {name}"),
        Some(check) if check.damage == 0 => {
            format!("{name} {:+} · effet spécial", check.modifier)
        }
        Some(check) => format!(
            "{name} {:+} · perte {} {resource}",
            check.modifier, check.damage
        ),
    }
}

/// An investigator card: subtitle, price, hands and text.
pub fn investigator_card(card: &InvestigatorCard) -> Tip {
    Tip::new(card.name.clone())
        .maybe(card.subtitle.clone())
        .maybe(card.price.map(|price| format!("Prix : {price} $")))
        .maybe((card.hands > 0).then(|| format!("Mains : {}", card.hands)))
        .maybe(Some(card.text.clone()))
}

/// A Mythos card and its text.
pub fn mythos_card(card: &MythosCard) -> Tip {
    Tip::new(card.name.clone()).maybe(Some(card.text.clone()))
}

/// An open gate: its world, where it stands and how hard it is to close.
pub fn gate(world: &str, place: &str, close_modifier: i8) -> Tip {
    Tip::new(format!("Portail vers {world}"))
        .line(format!("À : {place}"))
        .line(format!("Modificateur pour le fermer : {close_modifier:+}"))
}

/// The Ancient One sheet: combat rating, defenses, worshippers, power, attack.
pub fn ancient_one(sheet: &AncientOne) -> Tip {
    let rating = match sheet.combat_rating {
        CombatRating::Fixed(value) => format!("{value:+}"),
        CombatRating::MinusTerror => "-X (X = niveau de Terreur)".to_owned(),
        CombatRating::Infinite => "-∞".to_owned(),
    };
    let defenses: Vec<String> = sheet.defenses.iter().map(|a| a.label()).collect();
    Tip::new(sheet.name.clone())
        .line(format!("Combat : {rating}"))
        .maybe((!defenses.is_empty()).then(|| format!("Défenses : {}", defenses.join(", "))))
        .maybe(sheet.defenses_note.clone())
        .line(format!("Adorateurs : {}", sheet.worshippers))
        .line(format!("{} : {}", sheet.power.name, sheet.power.text))
        .maybe(
            sheet
                .start_of_battle
                .as_ref()
                .map(|text| format!("Début de la Bataille : {text}")),
        )
        .line(format!("Attaque : {}", sheet.attack))
}

/// An investigator: place, Sanity, Stamina, money, clues, cards and ability.
pub fn investigator(view: &View<'_, '_>, seat: usize) -> Tip {
    let data = view.data();
    let Some(inv) = view.state().investigators.get(seat) else {
        return Tip::default();
    };
    let found = sheet(data, &inv.id);
    let cards: Vec<String> = inv
        .cards
        .iter()
        .map(|held| card(data, &held.card).name)
        .collect();
    Tip::new(investigator_name(data, &inv.id))
        .maybe(found.map(|s| s.title.clone()))
        .line(format!("À : {}", view.place_name(&inv.place)))
        .line(format!(
            "Santé Mentale {}/{} · Résistance {}/{}",
            inv.sanity,
            view.maximum(seat, Resource::Sanity),
            inv.stamina,
            view.maximum(seat, Resource::Stamina)
        ))
        .line(format!("Argent {} $ · Indices {}", inv.money, inv.clues))
        .maybe((!cards.is_empty()).then(|| format!("Cartes : {}", cards.join(", "))))
        .maybe(found.map(|s| format!("{} : {}", s.ability.name, s.ability.text)))
}

/// Name of the world a gate leads to.
pub fn world_name(data: &GameData, world: &data::OtherWorldId) -> String {
    data.board
        .other_world(world)
        .map_or_else(|| world.to_string(), |w| w.name.clone())
}
