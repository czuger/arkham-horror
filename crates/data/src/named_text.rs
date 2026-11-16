//! Named rule texts printed on the board and on sheets.

use serde::{Deserialize, Serialize};

/// A named ability or power with its printed French text. The engine codes
/// its effect by name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NamedText {
    /// Name, in French (« Prêt Bancaire », « Esprit Fort »).
    pub name: String,
    /// Text as printed, in French.
    pub text: String,
}
