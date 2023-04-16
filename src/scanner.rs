use crate::error::LoxError;
use crate::tokentype::TokenType;
use crate::token::Token;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source, 
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }
    
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        
        self.tokens.push(Token::eof(self.line));
        Ok(&self.tokens)
    }

    pub fn scan_token(&self) {}

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

