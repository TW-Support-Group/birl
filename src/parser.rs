use crate::ast::{Expr, Node, Stmt};
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    stmts: Vec<Node>,
    current: usize,
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(Token, Token),
    UnexpectedEof,
    InvalidStartOfStmt,
    InvalidStartOfExpr,
}

type Result<T> = std::result::Result<T, ParserError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            stmts: Vec::new(),
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            self.current += 1;
            return Some(&self.tokens[self.current - 1]);
        }
        None
    }

    fn peek(&self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            return Some(&self.tokens[self.current]);
        }
        None
    }

    fn expect(&mut self, expected: Token) -> Result<()> {
        // TODO: This can probably be written better.
        if let Some(actual) = self.advance() {
            if actual == &expected {
                Ok(())
            } else {
                Err(ParserError::UnexpectedToken(expected, actual.clone()))
            }
        } else {
            Err(ParserError::UnexpectedEof)
        }
    }

    pub fn parse(mut self) -> Result<Node> {
        self.expect(Token::HoraDoShow)?;

        while let Some(token) = self.peek() {
            let token = token.clone();

            if token == Token::Birl {
                break;
            }

            self.advance();

            match token {
                Token::Print => self.parse_print_stmt()?,
                Token::Return => self.parse_return_stmt()?,
                _ => return Err(ParserError::InvalidStartOfStmt),
            }
        }

        self.expect(Token::Birl)?;

        Ok(Node::Program(self.stmts))
    }

    fn parse_print_stmt(&mut self) -> Result<()> {
        self.expect(Token::Lparen)?;

        // We need to some way to match tokens without constructing them.
        // TODO: Add a tag to the Token enum.

        if let Some(t) = self.advance() {
            match t.clone() {
                Token::BirlString(s) => self.stmts.push(Stmt::print(&s)),
                _ => {
                    return Err(ParserError::UnexpectedToken(
                        t.clone(),
                        Token::BirlString("".to_string()),
                    ))
                }
            }
        } else {
            return Err(ParserError::UnexpectedEof);
        }

        self.expect(Token::Rparen)?;
        self.expect(Token::Semicolon)?;

        Ok(())
    }

    fn parse_return_stmt(&mut self) -> Result<()> {
        let value = self.parse_expr()?;
        self.expect(Token::Semicolon)?;
        self.stmts.push(Stmt::return_(value));
        Ok(())
    }

    fn parse_expr(&mut self) -> Result<Box<Node>> {
        // TODO: Create helper for creating expressions in ast.rs.
        // For now we're just contemplating strings and numbers.
        let expr = match self.advance() {
            Some(Token::BirlString(s)) => Box::new(Node::Expr(Expr::String(s.clone()))),
            Some(Token::Number(n)) => Box::new(Node::Expr(Expr::Number(*n))),
            _ => return Err(ParserError::InvalidStartOfExpr),
        };
        Ok(expr)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parser_can_parse_hello_world() {
        let test = "HORA DO SHOW\n    CE QUER VER ESSA PORRA? (\"Hello, World! Porra!\\n\");\n    BORA CUMPADE 0;\nBIRL";
        let tokens = Lexer::new(test).lex().unwrap();

        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let stmts = ast.stmts().unwrap();

        assert!(matches!(ast, Node::Program(_)));
        assert_eq!(stmts.len(), 2);
        assert!(matches!(stmts[0], Node::Stmt(Stmt::PrintStmt { .. })));
        assert!(matches!(stmts[1], Node::Stmt(Stmt::ReturnStmt { .. })));
    }
}
