//! Identifiers that link the data files together.
//!
//! They are plain English snake_case strings in the RON files (`"train_station"`),
//! wrapped in distinct types so a street id cannot be used where a location id
//! is expected.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Declares a string identifier newtype with the usual conversions.
macro_rules! string_id {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Builds an identifier from its text form.
            pub fn new(id: impl Into<String>) -> Self {
                Self(id.into())
            }

            /// Returns the identifier as written in the data files.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

string_id!(
    /// Identifies an Arkham location (`"train_station"`).
    LocationId
);

string_id!(
    /// Identifies a street area (`"northside"`).
    StreetId
);

string_id!(
    /// Identifies an Other World (`"rlyeh"`).
    OtherWorldId
);

string_id!(
    /// Identifies a monster kind (`"byakhee"`).
    MonsterId
);

string_id!(
    /// Identifies an investigator (`"amanda_sharpe"`).
    InvestigatorId
);

string_id!(
    /// Identifies an Ancient One (`"cthulhu"`).
    AncientOneId
);

string_id!(
    /// Identifies a card of any deck (`"revolver_38"`, `"clacv02"`).
    CardId
);

string_id!(
    /// Names an effect coded by hand in the engine (« effet spécial »).
    SpecialId
);
