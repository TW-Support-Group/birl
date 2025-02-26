#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    HoraDoShow,
    Birl,
    Print,
    Lparen,
    Rparen,
    BirlString(String),
    Number(f64),
    Semicolon,
    Return,
}
