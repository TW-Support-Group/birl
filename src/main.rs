#![feature(let_chains)]

mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;

use lexer::Lexer;

fn main() {
    env_logger::init();
}
