use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct LoxError {
    token: Option<Token>,
    line: usize,
    message: String,
}

impl LoxError {
    pub fn error(line: usize, message: String) -> LoxError {
        let err = LoxError { token: None, line, message };
        err.report("");
        err
    }

    pub fn report(&self, loc: &str) {
        eprintln!("[line {}] Error {}: {}", self.line, loc, self.message);
    }

    pub fn parsererror(token: Token, message: String) -> LoxError {
        let err = LoxError {token: Some(token.clone()), line: token.line, message};
        match token.ttype {
            TokenType::Eof => err.report("at end"),
            _ => err.report(&format!("at '{}'", &token.lexeme)),
        }
        err
    }

    pub fn evalerror() -> LoxError {
        LoxError {token: None, line: 0, message: "".to_owned()}
    }
}
