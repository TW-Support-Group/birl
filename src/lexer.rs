use log;
use std::str;

use crate::token::Token;

/// Lexer for the BIRL language.
pub struct Lexer {
    current: usize,
    start: usize,
    source_bytes: Vec<u8>,
    tokens: Vec<Token>,
}

#[derive(Debug)]
pub enum LexerError {
    UnexpectedSequence(String),
    InvalidEscapeSequence(u8),
    UnterminatedStringLiteral(u8),
    InvalidNumericLiteral,
}

type Result<T> = std::result::Result<T, LexerError>;

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            current: 0,
            start: 0,
            source_bytes: source.bytes().collect(),
            tokens: Vec::new(),
        }
    }

    fn peek(&self) -> Option<u8> {
        if self.current < self.source_bytes.len() {
            return self.source_bytes[self.current].into();
        }
        None
    }

    fn advance(&mut self) -> Option<u8> {
        if self.current < self.source_bytes.len() {
            self.current += 1;
            return self.source_bytes[self.current - 1].into();
        }
        None
    }

    fn token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn lex(mut self) -> Result<Vec<Token>> {
        while let Some(c) = self.advance() {
            match c {
                b'(' => self.token(Token::Lparen),
                b')' => self.token(Token::Rparen),
                b';' => self.token(Token::Semicolon),
                b'H' => self.hora_do_show()?,
                b'C' => self.print()?,
                b'B' => match self.peek() {
                    Some(b'I') => self.birl()?,
                    Some(b'O') => self.return_()?,
                    _ => {}
                },
                b'"' => self.string()?,
                b'0'..b'9' => self.number()?,
                c => {
                    log::debug!(c; "skipping character: {c:?}");
                }
            }

            self.start = self.current;
        }

        Ok(self.tokens)
    }

    fn expect(&mut self, expected: &str) -> Result<()> {
        let left = self.source_bytes.len() - self.start;
        if expected.len() > left {
            let left_str = str::from_utf8(&self.source_bytes[self.start..]);
            log::warn!(expected; "there were fewer bytes than expected: {expected:?} {left:?}");
            log::warn!("string left: {left_str:?}");
            return Err(LexerError::UnexpectedSequence(expected.to_string()));
        }

        let actual = &self.source_bytes[self.start..self.start + expected.len()];
        if actual != expected.as_bytes() {
            log::warn!(
                "expected is not equal to actual: {:?} {:?}",
                str::from_utf8(&actual),
                expected
            );
            return Err(LexerError::UnexpectedSequence(expected.to_string()));
        }

        self.current += expected.len();

        Ok(())
    }

    fn hora_do_show(&mut self) -> Result<()> {
        self.expect("HORA DO SHOW")?;
        self.token(Token::HoraDoShow);
        Ok(())
    }

    fn print(&mut self) -> Result<()> {
        self.expect("CE QUER VER ESSA PORRA?")?;
        self.token(Token::Print);
        Ok(())
    }

    fn birl(&mut self) -> Result<()> {
        self.expect("BIRL")?;
        self.token(Token::Birl);
        Ok(())
    }

    fn return_(&mut self) -> Result<()> {
        self.expect("BORA CUMPADE")?;
        self.token(Token::Return);
        Ok(())
    }

    fn string(&mut self) -> Result<()> {
        assert!(self.source_bytes[self.start] == b'"');

        let mut s = String::new();

        while let Some(c) = self.peek() {
            if c == b'"' {
                break;
            }

            if c != b'\\' {
                s.push(char::from_u32(c.into()).unwrap());
            }

            // TODO: Make this prettier.

            if c == b'\\'
                && self.advance().is_some()
                && let Some(c) = self.peek()
                && c != b'n'
            {
                log::warn!("invalid escape sequence");
                return Err(LexerError::InvalidEscapeSequence(c));
            } else if c == b'\\' {
                s.push('\n');
            }

            self.advance();
        }

        if let c = self.advance()
            && (c.is_none() || c.unwrap() != b'"')
        {
            log::warn!("unterminated string literal");
            return Err(LexerError::UnterminatedStringLiteral(c.unwrap()));
        }

        self.token(Token::BirlString(s));

        Ok(())
    }

    fn number(&mut self) -> Result<()> {
        if self.source_bytes[self.start] == b'0' {
            self.token(Token::Number(0.));
            Ok(())
        } else {
            Err(LexerError::InvalidNumericLiteral)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_lex_simple_program() {
        let test = "HORA DO SHOW\nBIRL";
        let lexer = Lexer::new(test);
        let tokens = lexer.lex();

        assert!(matches!(tokens, Ok(_)));
        assert_eq!(tokens.as_ref().unwrap().len(), 2);
        assert_eq!(tokens.unwrap(), vec![Token::HoraDoShow, Token::Birl]);
    }

    #[test]
    fn test_can_lex_hello_world() {
        let test = "HORA DO SHOW\n    CE QUER VER ESSA PORRA? (\"Hello, World! Porra!\\n\");\n    BORA CUMPADE 0;\nBIRL";
        let lexer = Lexer::new(test);
        let tokens = lexer.lex();

        assert!(matches!(tokens, Ok(_)));
        assert_eq!(tokens.as_ref().unwrap().len(), 10);

        let tokens = tokens.unwrap();
        let mut iter = tokens.iter();
        assert_eq!(iter.next().unwrap(), &Token::HoraDoShow);
        assert_eq!(iter.next().unwrap(), &Token::Print);
        assert_eq!(iter.next().unwrap(), &Token::Lparen);
        assert!(matches!(iter.next().unwrap(), Token::BirlString(_)));
        assert_eq!(iter.next().unwrap(), &Token::Rparen);
        assert_eq!(iter.next().unwrap(), &Token::Semicolon);
        assert_eq!(iter.next().unwrap(), &Token::Return);
        assert!(matches!(iter.next().unwrap(), Token::Number(_)));
        assert_eq!(iter.next().unwrap(), &Token::Semicolon);
        assert_eq!(iter.next().unwrap(), &Token::Birl);
    }
}
