/// One place drawn on the board, with what stands there.
#[derive(Debug, Clone, Default)]
pub struct Spot {
    /// French name of the place.
    pub name: String,
    /// Horizontal position, in percent of the board width.
    pub left: String,
    /// Vertical position, in percent of the board height.
    pub top: String,
    /// Investigators standing there.
    pub investigators: Vec<InvestigatorToken>,
    /// Monsters standing there.
    pub monsters: Vec<ImageToken>,
    /// The open gate, if any.
    pub gate: Option<ImageToken>,
    /// Clue tokens.
    pub clues: u8,
    /// Whether an Elder Sign seals the location.
    pub sealed: bool,
    /// Whether the location is closed.
    pub closed: bool,
    /// Answers of the current decision that pick this place: index and label.
    pub choices: Vec<(usize, String)>,
}

impl Spot {
    /// Whether there is anything to draw.
    pub fn is_empty(&self) -> bool {
        self.investigators.is_empty()
            && self.monsters.is_empty()
            && self.gate.is_none()
            && self.clues == 0
            && !self.sealed
            && !self.closed
            && self.choices.is_empty()
    }
}

/// An investigator pawn.
#[derive(Debug, Clone)]
pub struct InvestigatorToken {
    /// Full name, shown on hover.
    pub name: String,
    /// Initials printed on the pawn.
    pub initials: String,
    /// Whether it is the human's investigator.
    pub human: bool,
}

/// A token shown by its image (monster or gate).
#[derive(Debug, Clone)]
pub struct ImageToken {
    /// French name, shown on hover.
    pub name: String,
    /// Image path under `/assets/`.
    pub image: String,
}
