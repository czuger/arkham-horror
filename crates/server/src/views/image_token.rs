use crate::views::tip::Tip;

/// A game element shown by its name or image (monster, gate, card, Ancient
/// One), with its hover bubble.
#[derive(Debug, Clone)]
pub struct ImageToken {
    /// French name.
    pub name: String,
    /// Image path under `/assets/` (empty when there is none).
    pub image: String,
    /// Bubble shown on hover.
    pub tip: Tip,
}
