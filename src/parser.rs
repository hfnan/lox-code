use crate::{error::LoxError, expr::*, token::*, object::Object, stmt::{Stmt, PrintStmt, ExpressionStmt, VarStmt, BlockStmt}};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    had_error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0, had_error: false }
    }

    pub fn success(&self) -> bool {
        !self.had_error
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current).cloned()
    }

    fn advance(&mut self) -> Token {
        let res = self.tokens.get(self.current).unwrap().clone();
        self.current += 1;
        res
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.equality()?;

        match self.peek() {
            Some(token) if token.ttype == TokenType::Assign => {
                let equals = token;
                self.advance();
                let value = self.assignment()?;

                if let Expr::Variable(variable) = expr {
                    let name = variable.name;
                    return Ok(Expr::Assign(AssignExpr { name, value: Box::new(value) }))
                } 
                self.had_error = true;
                LoxError::parse_error(equals, "Invalid Assignment Target.");
            },
            _ => ()
        }
        Ok(expr)
    }

    // fn comma(&mut self) -> Result<Expr, LoxError> {
    //     let mut expr = self.equality()?;

    //     while matches!(self.peek(), Some(token) if matches!(token.ttype, TokenType::Comma)) {
    //         let operator = self.peek().unwrap();
    //         self.advance();
    //         let right = self.equality()?;
    //         expr = Expr::Binary(BinaryExpr {
    //             left: Box::new(expr),
    //             operator,
    //             right: Box::new(right),
    //         });
    //     }

    //     Ok(expr)
    // }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while matches!(self.peek(), Some(token) if matches!(token.ttype, TokenType::Equal | TokenType::BangEqual))
        {
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

        while matches!(self.peek(), Some(token) if matches!(token.ttype, TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual))
        {
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

        while matches!(self.peek(), Some(token) if matches!(token.ttype, TokenType::Plus | TokenType::Minus)) {
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

        while matches!(self.peek(), Some(token) if matches!(token.ttype, TokenType::Star | TokenType::Slash)) {
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
        if matches!(self.peek(), Some(token) if matches!(token.ttype, TokenType::Bang | TokenType::Minus)) {
            let operator = self.peek().unwrap();
            self.advance();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        match self.peek() {
            Some(token) => {
                self.advance();
                match token.ttype {
                    TokenType::False => Ok(Expr::Literal(LiteralExpr {
                        value: Some(Object::Bool(false)),
                    })),
                    TokenType::True => Ok(Expr::Literal(LiteralExpr {
                        value: Some(Object::Bool(true)),
                    })),
                    TokenType::Nil => Ok(Expr::Literal(LiteralExpr {
                        value: Some(Object::Nil),
                    })),
                    TokenType::Number | TokenType::String => Ok(Expr::Literal(LiteralExpr {
                        value: token.literal,
                    })),
                    TokenType::Identifier => Ok(Expr::Variable(VariableExpr { name: token })),
                    TokenType::LeftParen => {
                        let expr = self.expression()?;
                        self.consume(TokenType::RightParen, "Expect ')' after Expression")?;
                        Ok(Expr::Grouping(GroupingExpr {
                            expression: Box::new(expr),
                        }))
                    }
                    _ => Err(LoxError::parse_error(
                        token,
                        "Expect expression.")),
                }
            }
            _ => Err(LoxError::error(0, "Failed primary parser.")),
        }
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token, LoxError> {
        match self.peek() {
            Some(token) if token.ttype == ttype => {
                self.advance();
                Ok(token)
            }
            _ => Err(LoxError::parse_error(
                self.peek().unwrap(),
                message,
            )),
        }
    }

    fn synchronize(&mut self) {
        while let Some(token) = self.peek() {
            match token.ttype {
                TokenType::Eof
                | TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => break,
                _ => {
                    self.advance();
                }
            }
            if matches!(self.peek(), Some(token) if matches!(token.ttype, TokenType::SemiColon)) {
                break;
            }
        }
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        match self.peek() {
            Some(token) if token.ttype == TokenType::Print => {
                self.advance();
                self.print_statement()
            },
            Some(token) if token.ttype == TokenType::LeftBrace => {
                self.advance();
                Ok(Stmt::Block(BlockStmt {statements: self.block()?}))
            }
            _ => self.expression_statement(),
        }
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements = Vec::new();

        while let Some(token) = self.peek() {
            if token.ttype == TokenType::RightBrace {
                break;
            }
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after value.")?;
        Ok(Stmt::Print(PrintStmt { expression: Box::new(value) }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expression = self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(ExpressionStmt { expression: Box::new(expression) }))
    }

    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        let res = match self.peek() {
            Some(token) if token.ttype == TokenType::Var => {
                self.advance();
                self.var_declaration()
            }
            _ => self.statement()
        };

        if res.is_err() {
            self.synchronize();
        }
        res
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::Identifier, "Expect Variable name.")?;
        let initializer = match self.peek() {
            Some(token) if token.ttype == TokenType::Assign => {
                self.advance();
                Some(self.expression()?)
            },
            _ => None,
        };

        self.consume(TokenType::SemiColon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var(VarStmt { name, initializer }))
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements = Vec::new();
        while matches!(self.peek(), Some(token) if !matches!(token.ttype, TokenType::Eof)) {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }
}

