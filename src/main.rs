#![feature(let_chains)]

mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;

use std::fs;
use std::io;

use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;

fn main() {
    env_logger::init();

    // TODO: Expose the interpreter over HTTP
    // You can use axum for this, though a more lightweight server would be more appropriate.

    let file_contents = fs::read_to_string("test.birl").unwrap();
    let tokens = Lexer::new(&file_contents).lex().unwrap();
    let ast = Parser::new(tokens).parse().unwrap();

    let mut out = io::stdout();
    let mut err = io::stderr();
    let mut evaluator = Evaluator::new(&mut out, &mut err);

    evaluator.eval(ast);
}
