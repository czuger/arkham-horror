//! A monster's values on the combat page, each with the rule it stands for,
//! shown in a bubble on hover.

use data::{Check, Monster, MonsterAbility, Movement};
use engine::MonsterStats;

use crate::views::tip::Tip;
use crate::views::tips::check_line;

/// One line about a monster, and what it means.
#[derive(Debug, Clone)]
pub struct Fact {
    /// The line (« Conscience -1 »).
    pub text: String,
    /// The rule behind it; `None` for a text that explains itself.
    pub tip: Option<Tip>,
}

impl Fact {
    fn new(text: String, title: &str, rule: String) -> Self {
        Self {
            text,
            tip: Some(Tip::new(title.to_owned()).line(rule)),
        }
    }
}

/// The monster's values as they stand now: movement, awareness, Horror,
/// Combat, toughness, each ability, then its special text.
pub fn monster_facts(monster: &Monster, stats: &MonsterStats) -> Vec<Fact> {
    let mut facts = vec![
        Fact::new(
            format!("Mouvement {}", stats.movement.label()),
            "Mouvement",
            movement_rule(stats.movement).to_owned(),
        ),
        Fact::new(
            format!("Conscience {:+}", stats.awareness),
            "Conscience",
            format!(
                "Modifie le test d'Évasion : votre Discrétion {:+}. Il faut 1 succès. \
                 Une évasion ratée vous fait subir ses dégâts de combat, puis le combat commence.",
                stats.awareness
            ),
        ),
        Fact::new(
            check_line("Horreur", stats.horror, "Santé Mentale"),
            "Test d'Horreur",
            horror_rule(stats.horror),
        ),
        Fact::new(
            check_line("Combat", stats.combat, "Résistance"),
            "Test de Combat",
            combat_rule(stats.combat),
        ),
        Fact::new(
            format!("Force {}", stats.toughness),
            "Force",
            format!(
                "Succès nécessaires au test de Combat pour vaincre le monstre, en un seul jet : \
                 les succès ne s'additionnent pas d'un round à l'autre. Vaincu, il devient un \
                 trophée qui vaut {}.",
                stats.toughness
            ),
        ),
    ];
    facts.extend(
        stats
            .abilities
            .iter()
            .map(|ability| Fact::new(ability.label(), "Capacité", ability_rule(*ability))),
    );
    facts.extend(monster.special.clone().map(|text| Fact { text, tip: None }));
    facts
}

/// The three checks against the monster, with how their dice are counted.
pub fn evade_rule() -> Tip {
    Tip::new("Test d'Évasion".to_owned()).line(
        "Discrétion + Conscience du monstre + vos bonus. Il faut 1 succès. \
         Il sert à lui échapper et à fuir le combat."
            .to_owned(),
    )
}

/// How the dice of the Horror check are counted.
pub fn horror_dice_rule() -> Tip {
    Tip::new("Test d'Horreur".to_owned()).line(
        "Volonté + valeur d'Horreur du monstre + vos bonus. Il faut 1 succès. \
         Un seul par combat, au début."
            .to_owned(),
    )
}

/// How the dice of the Combat check are counted.
pub fn combat_dice_rule() -> Tip {
    Tip::new("Test de Combat".to_owned()).line(
        "Vigueur + bonus des armes et sorts en main + valeur de Combat du monstre + vos autres \
         bonus. Il faut autant de succès que sa Force, en un seul jet."
            .to_owned(),
    )
}

fn movement_rule(movement: Movement) -> &'static str {
    match movement {
        Movement::Normal => {
            "Liseré noir : pendant le Mythe, avance d'une zone en suivant la flèche de sa couleur."
        }
        Movement::Fast => {
            "Liseré rouge : pendant le Mythe, avance de deux zones ; s'arrête s'il rencontre un \
             investigateur."
        }
        Movement::Stationary => "Liseré jaune : ne bouge jamais.",
        Movement::Flying => {
            "Liseré bleu : fond sur l'investigateur d'une rue voisine (celui qui a la Discrétion \
             la plus faible), sinon part dans le Ciel, d'où il peut fondre sur n'importe quelle rue."
        }
        Movement::Special => "Liseré vert : mouvement particulier, décrit sur son marqueur.",
    }
}

fn horror_rule(check: Option<Check>) -> String {
    match check {
        None => "Ce monstre ne provoque pas de test d'Horreur.".to_owned(),
        Some(check) => format!(
            "Au début du combat (pas si vous lui échappez) : test de Volonté {:+}, 1 succès \
             nécessaire. {}",
            check.modifier,
            damage("Santé Mentale", check.damage)
        ),
    }
}

fn combat_rule(check: Option<Check>) -> String {
    match check {
        None => "On ne peut pas combattre ce monstre.".to_owned(),
        Some(check) => format!(
            "Test de Vigueur {:+}, plus les bonus des armes et sorts en main. {} Cela vaut aussi \
             pour chaque évasion ou fuite ratée.",
            check.modifier,
            damage("Résistance", check.damage)
        ),
    }
}

fn damage(resource: &str, amount: u8) -> String {
    if amount == 0 {
        "En cas d'échec, effet spécial décrit sur le marqueur.".to_owned()
    } else {
        format!("En cas d'échec, vous perdez {amount} {resource}.")
    }
}

fn ability_rule(ability: MonsterAbility) -> String {
    match ability {
        MonsterAbility::Ambush => {
            "Embuscade : une fois le combat engagé, impossible de fuir. Le combat dure jusqu'à la \
             victoire de l'un des deux."
                .to_owned()
        }
        MonsterAbility::Endless => {
            "Inépuisable : ne devient jamais un trophée. Vaincu, il retourne dans la réserve."
                .to_owned()
        }
        MonsterAbility::Undead => {
            "Mort-vivant : certaines cartes (Croix…) sont plus fortes contre lui.".to_owned()
        }
        MonsterAbility::Mask => {
            "Masque : n'est dans la réserve que si le Grand Ancien est Nyarlathotep.".to_owned()
        }
        MonsterAbility::PhysicalResistance => {
            "Résistance Physique : les bonus de Combat des armes physiques sont divisés par 2 \
             (arrondi au-dessus)."
                .to_owned()
        }
        MonsterAbility::PhysicalImmunity => {
            "Immunité Physique : les bonus de Combat des armes physiques valent 0.".to_owned()
        }
        MonsterAbility::MagicalResistance => {
            "Résistance Magique : les bonus de Combat des armes magiques et des sorts sont divisés \
             par 2 (arrondi au-dessus)."
                .to_owned()
        }
        MonsterAbility::MagicalImmunity => {
            "Immunité Magique : les bonus de Combat des armes magiques et des sorts valent 0."
                .to_owned()
        }
        MonsterAbility::Nightmarish(n) => format!(
            "Cauchemardesque {n} : même si vous réussissez le test d'Horreur, vous perdez {n} \
             Santé Mentale."
        ),
        MonsterAbility::Overwhelming(n) => format!(
            "Irrésistible {n} : même si vous battez le monstre, vous perdez {n} Résistance."
        ),
    }
}
