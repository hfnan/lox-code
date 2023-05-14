use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct LoxError {
    token: Option<Token>,
    line: usize,
    message: String,
}

impl LoxError {
    pub fn error(line: usize, message: &str) -> LoxError {
        let err = LoxError { token: None, line, message: message.to_owned() };
        err.report("");
        err
    }

    pub fn report(&self, loc: &str) {
        eprintln!("[line {}] Error {}: {}", self.line, loc, self.message);
    }

    pub fn parse_error(token: Token, message: &str) -> LoxError {
        let err = LoxError {token: Some(token.clone()), line: token.line, message: message.to_owned()};
        match token.ttype {
            TokenType::Eof => err.report("at end"),
            _ => err.report(&format!("at '{}'", &token.lexeme)),
        }
        err
    }

    pub fn runtime_error(token: Option<&Token>, message: Option<&str>) -> LoxError {
        let mut err = LoxError::basic_runtime_error();
        if let Some(message) = message {
            err.message = message.to_owned();
        }

        if let Some(token) = token {
            err.token = Some(token.clone());
            err.line = token.line;
        }
        err
    }

    fn basic_runtime_error() -> LoxError {
        LoxError { token: None, line: 0, message: "there is something wrong when interpreting.".to_owned() }
    }
}
