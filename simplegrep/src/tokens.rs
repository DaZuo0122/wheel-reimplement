#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Char(char),
    Escape(char),

    // Operators
    Concat,      // implicit concatenation
    Alternation, // |
    Star,        // *
    Plus,        // +
    Question,    // ?
    Range,       // {n,m}

    // Groups
    OpenParen,    // (
    CloseParen,   // )
    OpenBracket,  // [
    CloseBracket, // ]

    // Character classes
    AnyChar,    // .
    Digit,      // \d
    WordChar,   // \w
    Whitespace, // \s

    // Anchors
    StartLine,    // ^
    EndLine,      // $
    StartInput,   // \A
    EndInput,     // \z
    WordBoundary, // \b

    EOF,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let ch = self.input[self.position..].chars().next().unwrap();

        match ch {
            ' ' | '\t' | '\n' | '\r' => {
                self.position += 1;
                self.next_token() // skip whitespace
            }
            '(' => {
                self.position += 1;
                Token::OpenParen
            }
            ')' => {
                self.position += 1;
                Token::CloseParen
            }
            '[' => {
                self.position += 1;
                Token::OpenBracket
            }
            ']' => {
                self.position += 1;
                Token::CloseBracket
            }
            '*' => {
                self.position += 1;
                Token::Star
            }
            '+' => {
                self.position += 1;
                Token::Plus
            }
            '?' => {
                self.position += 1;
                Token::Question
            }
            '|' => {
                self.position += 1;
                Token::Alternation
            }
            '.' => {
                self.position += 1;
                Token::AnyChar
            }
            '^' => {
                self.position += 1;
                Token::StartLine
            }
            '$' => {
                self.position += 1;
                Token::EndLine
            }
            '\\' => {
                self.position += 1;
                self.handle_escape()
            }
            '{' => {
                self.position += 1;
                Token::Range
            }
            _ => {
                self.position += 1;
                Token::Char(ch)
            }
        }
    }

    fn handle_escape(&mut self) -> Token {
        if self.position >= self.input.len() {
            return Token::Char('\\');
        }

        let ch = self.input[self.position..].chars().next().unwrap();
        self.position += 1;

        match ch {
            'd' => Token::Digit,
            'w' => Token::WordChar,
            's' => Token::Whitespace,
            'A' => Token::StartInput,
            'z' => Token::EndInput,
            'b' => Token::WordBoundary,
            _ => Token::Escape(ch),
        }
    }
}
