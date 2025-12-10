#[derive(Debug, Clone, PartialEq)]
pub enum RegexNode {
    Char(char),
    AnyChar,
    Digit,
    WordChar,
    Whitespace,

    // Sequences
    Concat(Vec<RegexNode>),
    Alternation(Vec<RegexNode>),

    // Quantifiers
    Repeat(Box<RegexNode>, RepeatRange),
    Plus(Box<RegexNode>),
    Star(Box<RegexNode>),
    Question(Box<RegexNode>),

    // Groups
    Group(Box<RegexNode>),

    // Anchors
    StartLine,
    EndLine,
    StartInput,
    EndInput,
    WordBoundary,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RepeatRange {
    pub min: usize,
    pub max: Option<usize>,
}

impl RepeatRange {
    pub fn new(min: usize, max: Option<usize>) -> Self {
        Self { min, max }
    }

    pub fn exactly(n: usize) -> Self {
        Self::new(n, Some(n))
    }

    pub fn min(n: usize) -> Self {
        Self::new(n, None)
    }
}
