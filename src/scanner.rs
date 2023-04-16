use crate::error::LoxError;
use crate::tokentype::TokenType;
use crate::token::{Token, Object};

// todo: these are looked like an OOP theme code which do not even fit rust
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(), 
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }
    
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        let mut had_error: Option<LoxError> = None;
        while !self.is_at_end() {
            self.start = self.current;
            if let Err(e) = self.scan_token() {
                e.report("scanner.rs/scan_tokens()");
                had_error = Some(e);
            }
        }
        
        self.tokens.push(Token::eof(self.line));
        /*
        match had_error {
            Some(e) => Err(e),
            None => Ok(&self.tokens)
        } */
        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let tok = match self.is_match('=') {true => TokenType::BangEqual, false => TokenType::Bang };
                self.add_token(tok);
            },
            '=' => {
                let tok = match self.is_match('=') {true => TokenType::Equal, false => TokenType::Assign};
                self.add_token(tok);
            },
            '<' => {
                let tok = match self.is_match('=') {true => TokenType::LessEqual, false => TokenType::Less};
                self.add_token(tok);
            },
            '>' => {
                let tok = match self.is_match('=') {true => TokenType::GreaterEqual, false => TokenType::Greater};
                self.add_token(tok);
            },
            '/' => match self.is_match('/'){
                true => while let Some(ch) = self.peek(0) { match ch {
                    '\n' => break,
                    _ => self.advance(),
                };}, 
                false => self.add_token(TokenType::Slash),
            },
            '"' => self.string()?,
            '0'..='9' => self.number(),
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            ch => return Err(LoxError::error(self.line, format!("Unexpected Charactor: '{}'", ch))),
        }
        Ok(())
    }

    fn number(&mut self) { // what a nice code!
        while let Some('0'..='9') = self.peek(0) { self.advance();}        

        if let (Some('.'), Some('0'..='9')) = (self.peek(0), self.peek(1)) { 
            self.advance();
            while let Some('0'..='9') = self.peek(0) { self.advance();}        
        }
        
        let value: String = self.source[self.start..self.current].iter().collect();
        let num: f64 = value.parse().unwrap();                       
        self.add_token_object(TokenType::Number, Some(Object::Num(num)));
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while let Some(ch) = self.peek(0) { 
            match ch {
            '"' => break,
            '\n' => self.line += 1,
            _ => {},
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::error(self.line, format!("Unterminated String.")));
        }

        self.advance(); // advance after check
        
        let value: String = self.source[self.start + 1 .. self.current - 1].into_iter().collect();
        self.add_token_object(TokenType::String, Some(Object::Str(value)));
        Ok(())
    }

    fn is_match(&mut self, expected: char) -> bool {
        match self.source.get(self.current) {
            Some(ch) if *ch == expected => {
                self.current += 1; 
                true 
            },
            _ => false
        }
    }

    // todo: maybe we can use a more rusty way to implement this, such as iterator
    fn advance(&mut self) -> char {
        let res = *self.source.get(self.current).unwrap();
        self.current += 1;
        res
    }

    fn peek(&self, step: usize) -> Option<char> {
        self.source.get(self.current + step).copied() // option<&T>.copied() -> Option<T>
    } 

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, ttype: TokenType) {
        self.add_token_object(ttype, None);
    }

    fn add_token_object(&mut self, ttype: TokenType, literal: Option<Object>) {
        let s: String = self.source[self.start..self.current].into_iter().collect(); // convert a [char] to String
        self.tokens.push(Token::new(ttype, s, literal, self.line));
    }
}

