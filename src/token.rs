use crate::tokentype::TokenType;
use std::fmt;

#[derive(Debug)] 
pub enum Literal {
    Num(f64),
    Str(String),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(x) => write!(f, "{x}"),
            Self::Str(x) => write!(f, "\"{x}\""),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String, 
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Token {
        Token {ttype, lexeme, literal, line}
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {}", self.ttype, self.lexeme, match &self.literal {
            Some(literal) => literal.to_string(),
            None => "None".to_owned()
        })
    }
}