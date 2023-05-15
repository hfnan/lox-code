use crate::token::{Token, TokenType};

#[derive(Debug)]
pub enum LoxError {
    ScanError,
    ParseError,
    RuntimeError,
    ObjectError(String),
}


impl LoxError {
    pub fn report(line: usize, locate: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, locate, message);
    }

    pub fn scan_error(line: usize, message: &str) -> Self {
        Self::report(line, "", message);
        Self::ScanError
    }

    pub fn parse_error(token: &Token, message: &str) -> Self {
        match token.ttype {
            TokenType::Eof => Self::report(token.line, "at end", message),
            _ => Self::report(token.line, &format!("at '{}'", &token.lexeme), message),
        }
        LoxError::ParseError
    }

    pub fn object_error(message: &str) -> Self {
        Self::ObjectError(message.to_owned())
    }

    pub fn runtime_error(token: &Token, message: &str) -> Self {
        Self::report(token.line, "", message);
        Self::RuntimeError
    }

}
