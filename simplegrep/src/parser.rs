use crate::ast::{RegexNode, RepeatRange};
use crate::tokens::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Self {
            lexer,
            current_token,
        }
    }

    pub fn parse(&mut self) -> Result<RegexNode, String> {
        self.parse_alternation()
    }

    fn parse_alternation(&mut self) -> Result<RegexNode, String> {
        let mut nodes = vec![self.parse_concat()?];

        while self.current_token == Token::Alternation {
            self.consume_token(Token::Alternation)?;
            nodes.push(self.parse_concat()?);
        }

        Ok(if nodes.len() == 1 {
            nodes.into_iter().next().unwrap()
        } else {
            RegexNode::Alternation(nodes)
        })
    }

    fn parse_concat(&mut self) -> Result<RegexNode, String> {
        let mut nodes = vec![self.parse_atom()?];

        while self.current_token != Token::Alternation
            && self.current_token != Token::CloseParen
            && self.current_token != Token::EOF
        {
            nodes.push(self.parse_atom()?);
        }

        Ok(if nodes.len() == 1 {
            nodes.into_iter().next().unwrap()
        } else {
            RegexNode::Concat(nodes)
        })
    }

    fn parse_atom(&mut self) -> Result<RegexNode, String> {
        let node = self.parse_primary()?;

        // Handle quantifiers
        match self.current_token {
            Token::Star => {
                self.consume_token(Token::Star)?;
                Ok(RegexNode::Star(Box::new(node)))
            }
            Token::Plus => {
                self.consume_token(Token::Plus)?;
                Ok(RegexNode::Plus(Box::new(node)))
            }
            Token::Question => {
                self.consume_token(Token::Question)?;
                Ok(RegexNode::Question(Box::new(node)))
            }
            Token::Range => {
                self.consume_token(Token::Range)?;
                let range = self.parse_range()?;
                Ok(RegexNode::Repeat(Box::new(node), range))
            }
            _ => Ok(node),
        }
    }

    fn parse_primary(&mut self) -> Result<RegexNode, String> {
        match self.current_token {
            Token::Char(ch) => {
                self.consume_token(Token::Char(ch))?;
                Ok(RegexNode::Char(ch))
            }
            Token::Escape(ch) => {
                self.consume_token(Token::Escape(ch))?;
                Ok(self.escape_to_node(ch))
            }
            Token::AnyChar => {
                self.consume_token(Token::AnyChar)?;
                Ok(RegexNode::AnyChar)
            }
            Token::Digit => {
                self.consume_token(Token::Digit)?;
                Ok(RegexNode::Digit)
            }
            Token::WordChar => {
                self.consume_token(Token::WordChar)?;
                Ok(RegexNode::WordChar)
            }
            Token::Whitespace => {
                self.consume_token(Token::Whitespace)?;
                Ok(RegexNode::Whitespace)
            }
            Token::OpenParen => {
                self.consume_token(Token::OpenParen)?;
                let node = self.parse()?;
                self.consume_token(Token::CloseParen)?;
                Ok(RegexNode::Group(Box::new(node)))
            }
            Token::StartLine => {
                self.consume_token(Token::StartLine)?;
                Ok(RegexNode::StartLine)
            }
            Token::EndLine => {
                self.consume_token(Token::EndLine)?;
                Ok(RegexNode::EndLine)
            }
            Token::StartInput => {
                self.consume_token(Token::StartInput)?;
                Ok(RegexNode::StartInput)
            }
            Token::EndInput => {
                self.consume_token(Token::EndInput)?;
                Ok(RegexNode::EndInput)
            }
            Token::WordBoundary => {
                self.consume_token(Token::WordBoundary)?;
                Ok(RegexNode::WordBoundary)
            }
            _ => Err(format!("Unexpected token: {:?}", self.current_token)),
        }
    }

    fn parse_range(&mut self) -> Result<RepeatRange, String> {
        // Parse {min,max} or {min,} or {min}
        let mut min = 0;
        let mut max = None;

        // Parse min
        if let Token::Char(ch) = self.current_token {
            if ch.is_ascii_digit() {
                min = ch.to_digit(10).unwrap() as usize;
                self.consume_token(Token::Char(ch))?;

                // Check for comma
                if self.current_token == Token::Char(',') {
                    self.consume_token(Token::Char(','))?;

                    // Parse max if present
                    if let Token::Char(ch) = self.current_token {
                        if ch.is_ascii_digit() {
                            max = Some(ch.to_digit(10).unwrap() as usize);
                            self.consume_token(Token::Char(ch))?;
                        }
                    }
                }
            }
        }

        self.consume_token(Token::CloseBracket)?;
        Ok(RepeatRange::new(min, max))
    }

    fn escape_to_node(&self, ch: char) -> RegexNode {
        match ch {
            'd' => RegexNode::Digit,
            'w' => RegexNode::WordChar,
            's' => RegexNode::Whitespace,
            'A' => RegexNode::StartInput,
            'z' => RegexNode::EndInput,
            'b' => RegexNode::WordBoundary,
            _ => RegexNode::Char(ch),
        }
    }

    fn consume_token(&mut self, expected: Token) -> Result<(), String> {
        if self.current_token == expected {
            self.current_token = self.lexer.next_token();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, got {:?}",
                expected, self.current_token
            ))
        }
    }
}
