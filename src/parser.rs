use crate::{tokentype::TokenType, token::*, error::LoxError, expr::*};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(token) => token.ttype == TokenType::Eof,
            None => false,
        }
    }

    fn peek(&self) -> Option<Token> {
        match self.tokens.get(self.current) {
            Some(token) => Some(token.clone()),
            None => None
        }
    }

    fn advance(&mut self) -> Token {
        let res = self.tokens.get(self.current).unwrap().clone();
        self.current += 1;
        res
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while let TokenType::Equal | TokenType::BangEqual = match self.peek() {
            Some(token) => token.ttype,
            None => TokenType::Eof
        } {
            let operator = self.peek().unwrap();
            self.advance();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while let TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual = match self.peek() {
            Some(token) => token.ttype,
            None => TokenType::Eof
        } {
            let operator = self.peek().unwrap();
            self.advance();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr), 
                operator,
                right: Box::new(right),
            });            
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while let TokenType::Minus | TokenType::Plus = match self.peek() {
            Some(token) => token.ttype,
            None => TokenType::Eof
        } {
            let operator = self.peek().unwrap();
            self.advance();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr), 
                operator,
                right: Box::new(right),
            });            
        }

        Ok(expr)
    }
    
    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while let TokenType::Star | TokenType::Slash = match self.peek() {
            Some(token) => token.ttype,
            None => TokenType::Eof
        } {
            let operator = self.peek().unwrap();
            self.advance();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr), 
                operator,
                right: Box::new(right),
            });            
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if let TokenType::Bang | TokenType::Minus = match self.peek() {
            Some(token) => token.ttype,
            None => TokenType::Eof
        } {
            let operator = self.peek().unwrap();
            self.advance();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator, right: Box::new(right)
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        match self.peek() { 
            Some(token) => {
                self.advance();
                match token.ttype {
                    TokenType::False => Ok(Expr::Literal(LiteralExpr { value: Some(Literal::False) })),
                    TokenType::True => Ok(Expr::Literal(LiteralExpr { value: Some(Literal::True) })),
                    TokenType::Nil => Ok(Expr::Literal(LiteralExpr { value: Some(Literal::Nil) })),
                    TokenType::Number | TokenType::String => Ok(Expr::Literal(LiteralExpr{ value: token.literal})),
                    TokenType::LeftParen => {
                        let expr = self.expression()?;
                        // self.consume(TokenType::RightParen, "Expect ')' after Expression");
                        Ok(Expr::Grouping(GroupingExpr { expression: Box::new(expr) }))
                    }
                    _ => Err(LoxError::error(0, "Unexpected token in primary expression.".to_owned())) 
                }
            },
            _ => Err(LoxError::error(0, "Failed primary parser.".to_owned()))
        }
    }
}