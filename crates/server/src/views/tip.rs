use askama::Template;

/// A bubble shown when the pointer rests on a game element: a title and a few
/// lines of text. Rendered by `tip.html`, already escaped.
#[derive(Debug, Clone, Default, Template)]
#[template(path = "tip.html")]
pub struct Tip {
    /// First line, in bold.
    pub title: String,
    /// The other lines.
    pub lines: Vec<String>,
}

impl Tip {
    /// A bubble with only a title.
    pub fn new(title: String) -> Self {
        Self {
            title,
            lines: Vec::new(),
        }
    }

    /// Adds a line.
    pub fn line(mut self, line: String) -> Self {
        self.lines.push(line);
        self
    }

    /// Adds a line when there is one and it is not empty.
    pub fn maybe(self, line: Option<String>) -> Self {
        match line {
            Some(line) if !line.is_empty() => self.line(line),
            _ => self,
        }
    }
}
