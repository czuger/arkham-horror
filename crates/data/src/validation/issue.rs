//! Problems found by the consistency checks.

use std::fmt;
use std::path::PathBuf;

use crate::special_area::SpecialAreaKind;

/// One consistency problem in the data files. Displayed in French.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Issue {
    /// Two entries of the same family share an id.
    DuplicateId {
        /// Family, in French (« lieu », « rue »…).
        family: &'static str,
        /// The repeated id.
        id: String,
    },
    /// An entry refers to an id that does not exist.
    UnknownReference {
        /// Where the reference was found, in French.
        context: String,
        /// Family of the missing target, in French.
        family: &'static str,
        /// The missing id.
        id: String,
    },
    /// A street lists itself as a neighbour.
    SelfLink {
        /// The street.
        street: String,
    },
    /// A street link is not listed on both sides.
    AsymmetricLink {
        /// Street that lists the link.
        from: String,
        /// Street that does not list it back.
        to: String,
    },
    /// A monster arrow points to a street that is not a neighbour.
    ArrowNotNeighbour {
        /// The street the arrow leaves.
        street: String,
        /// Arrow colour, in French.
        colour: &'static str,
        /// Where the arrow points.
        target: String,
    },
    /// A street area has no location.
    EmptyDistrict {
        /// The street.
        street: String,
    },
    /// A required special area is missing.
    MissingSpecialArea {
        /// The missing area.
        kind: SpecialAreaKind,
    },
    /// An Other World has no encounter colour.
    NoEncounterColour {
        /// The Other World.
        world: String,
    },
    /// An Other World has no gate marker leading to it.
    WorldWithoutGate {
        /// The Other World.
        world: String,
    },
    /// A count that must be positive is zero.
    ZeroCopies {
        /// What has no copies, in French.
        context: String,
    },
    /// A skill track does not have one value per slider stop.
    WrongTrackLength {
        /// The investigator.
        investigator: String,
        /// The skill, in French.
        skill: &'static str,
        /// Number of values found.
        found: usize,
    },
    /// An Ancient One defense is not a resistance or an immunity.
    NotADefense {
        /// The Ancient One.
        ancient_one: String,
    },
    /// A number that must be positive is zero.
    Zero {
        /// What is zero, in French.
        context: String,
    },
    /// A location card has an encounter for a location of another district.
    LocationNotInDistrict {
        /// The card.
        card: String,
        /// The misplaced location.
        location: String,
    },
    /// A location card does not have exactly one normal encounter for a
    /// location of its district.
    EncounterCount {
        /// The card.
        card: String,
        /// The location.
        location: String,
        /// Number of normal encounters found.
        found: usize,
    },
    /// A Mythos card puts a clue or a gate on a stable location.
    StableLocation {
        /// The card.
        card: String,
        /// The stable location.
        location: String,
    },
    /// The parts of an entry do not fit together.
    Mismatch {
        /// The entry, in French.
        context: String,
        /// What is wrong, in French.
        reason: &'static str,
    },
    /// An image cited by the data is not in `assets/`.
    MissingImage {
        /// Expected path.
        path: PathBuf,
    },
}

impl fmt::Display for Issue {
    #[expect(clippy::too_many_lines, reason = "one flat arm per issue")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateId { family, id } => write!(f, "{family} « {id} » défini deux fois"),
            Self::UnknownReference {
                context,
                family,
                id,
            } => write!(f, "{context} : {family} « {id} » inconnu"),
            Self::SelfLink { street } => write!(f, "la rue « {street} » est reliée à elle-même"),
            Self::AsymmetricLink { from, to } => write!(
                f,
                "la rue « {from} » est reliée à « {to} », mais pas l'inverse"
            ),
            Self::ArrowNotNeighbour {
                street,
                colour,
                target,
            } => write!(
                f,
                "la flèche {colour} de « {street} » mène à « {target} », qui n'est pas une rue voisine"
            ),
            Self::EmptyDistrict { street } => write!(f, "le quartier « {street} » n'a aucun lieu"),
            Self::MissingSpecialArea { kind } => write!(f, "case spéciale absente : {kind:?}"),
            Self::NoEncounterColour { world } => {
                write!(
                    f,
                    "l'Autre Monde « {world} » n'a aucune couleur de rencontre"
                )
            }
            Self::WorldWithoutGate { world } => {
                write!(f, "aucun marqueur Portail ne mène à « {world} »")
            }
            Self::ZeroCopies { context } => write!(f, "{context} : nombre d'exemplaires nul"),
            Self::WrongTrackLength {
                investigator,
                skill,
                found,
            } => write!(
                f,
                "investigateur « {investigator} » : {found} valeurs de {skill} au lieu de 4"
            ),
            Self::NotADefense { ancient_one } => write!(
                f,
                "Grand Ancien « {ancient_one} » : seules les résistances et immunités sont des défenses"
            ),
            Self::Zero { context } => write!(f, "{context} : valeur nulle"),
            Self::LocationNotInDistrict { card, location } => write!(
                f,
                "carte Lieu « {card} » : le lieu « {location} » n'est pas dans ce quartier"
            ),
            Self::EncounterCount {
                card,
                location,
                found,
            } => write!(
                f,
                "carte Lieu « {card} » : {found} rencontres pour « {location} » au lieu d'une"
            ),
            Self::StableLocation { card, location } => write!(
                f,
                "carte Mythe « {card} » : indice ou portail sur le lieu stable « {location} »"
            ),
            Self::Mismatch { context, reason } => write!(f, "{context} : {reason}"),
            Self::MissingImage { path } => write!(f, "image absente : {}", path.display()),
        }
    }
}
