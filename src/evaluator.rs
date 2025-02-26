use crate::ast::{Node, Stmt};

use std::io::Write;
use std::str;

pub struct Evaluator<'a> {
    output_writer: &'a mut dyn Write,
    error_writer: &'a mut dyn Write,
}

impl<'a> Evaluator<'a> {
    fn new(output_writer: &'a mut dyn Write, error_writer: &'a mut dyn Write) -> Self {
        Self {
            output_writer,
            error_writer,
        }
    }

    pub fn eval(&mut self, ast: Node) {
        match ast {
            Node::Program(stmts) => stmts.into_iter().for_each(|stmt| self.eval(stmt)),
            Node::Stmt(stmt) => self.do_eval_stmt(stmt),
            _ => write!(&mut self.error_writer, "invalid start of statement")
                .expect("failed to write to error output"),
        }
    }

    fn do_eval_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::PrintStmt { format } => self.do_eval_print_stmt(&format),
            Stmt::ReturnStmt { .. } => {}
        }
    }

    fn do_eval_print_stmt(&mut self, format: &str) {
        write!(self.output_writer, "{}", format).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_can_evaluate_hello_world_program() {
        let test = "HORA DO SHOW\n    CE QUER VER ESSA PORRA? (\"Hello, World! Porra!\\n\");\n    BORA CUMPADE 0;\nBIRL";
        let tokens = Lexer::new(test).lex().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();

        let mut out = Vec::new();
        let mut err = Vec::new();
        let mut evaluator = Evaluator::new(&mut out, &mut err);

        evaluator.eval(ast);

        let output = str::from_utf8(&out).unwrap();
        assert_eq!(output, "Hello, World! Porra!\n");
    }
}
