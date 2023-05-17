use crate::{error::LoxError, expr::*, token::*, object::Object, stmt::*};

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
        self.comma()
    }

    fn comma(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.assignment()?;

        while matches!(self.peek(), Some(token) if matches!(token.ttype, TokenType::Comma)) {
            let operator = self.peek().unwrap();
            self.advance();
            let right = self.assignment()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.or()?;

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
                LoxError::parse_error(&equals, "Invalid Assignment Target.");
            },
            _ => ()
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.and()?;
        while matches!(self.peek(), Some(token) if token.ttype == TokenType::Or) {
            let operator = self.peek().unwrap();
            self.advance();
            let right = Box::new(self.and()?);
            expr = Expr::Logical(LogicalExpr { left: Box::new(expr), operator, right })
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.equality()?;
        while matches!(self.peek(), Some(token) if token.ttype == TokenType::And) {
            let operator = self.peek().unwrap();
            self.advance();
            let right = Box::new(self.and()?);
            expr = Expr::Logical(LogicalExpr { left: Box::new(expr), operator, right })
        }
     
        Ok(expr)
    }

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

        self.call()
    }

    fn call(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.primary()?;

        loop {
            match self.peek() {
                Some(token) if token.ttype == TokenType::LeftParen => {
                    self.advance();
                    expr = self.finish_call(expr)?;
                },
                _ => break,
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, LoxError> {
        let mut arguments = Vec::new();

        if matches!(self.peek(), Some(token) if token.ttype != TokenType::RightParen) {
            arguments.push(self.expression()?);
            while matches!(self.peek(), Some(token) if token.ttype == TokenType::Comma) {
                self.advance();
                if arguments.len() >= 255 {
                    if let Some(token) = self.peek() {
                        LoxError::parse_error(&token, "Can't have more than 255 arguments.");
                        self.had_error = true;
                    }
                }
                arguments.push(self.expression()?);
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

        Ok(Expr::Call(CallExpr { callee: Box::new(callee), paren, arguments }))
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
                        &token,
                        "Expect expression.")),
                }
            }
            _ => Err(LoxError::scan_error(0, "Failed primary parser.")),
        }
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token, LoxError> {
        match self.peek() {
            Some(token) if token.ttype == ttype => {
                self.advance();
                Ok(token)
            }
            _ => Err(LoxError::parse_error(
                &self.peek().unwrap(),
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
            Some(token) if token.ttype == TokenType::Break => {
                self.advance();
                self.consume(TokenType::SemiColon, "Expect ';' after break.")?;
                Ok(Stmt::Break(BreakStmt { line: token.line})) 
            }
            Some(token) if token.ttype == TokenType::Print => {
                self.advance();
                self.print_statement()
            },
            Some(token) if token.ttype == TokenType::LeftBrace => {
                self.advance();
                Ok(Stmt::Block(BlockStmt {statements: self.block()?}))
            },
            Some(token) if token.ttype == TokenType::If => {
                self.advance();
                self.if_statement()
            },
            Some(token) if token.ttype == TokenType::While => {
                self.advance();
                self.while_statement()
            },
            Some(token) if token.ttype == TokenType::For => {
                self.advance();
                self.for_statement()       
            }
            _ => self.expression_statement(),
        }
    }

    fn for_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = match self.peek() {
            Some(token) => {
                self.advance();
                match token.ttype {
                    TokenType::SemiColon => None, 
                    TokenType::Var => Some(self.var_declaration()?),
                    _ => Some(self.expression_statement()?)
                }
            }
            _ => None,
        };

        let condition = match self.peek() {
            Some(token) if token.ttype != TokenType::SemiColon => {
                self.expression()?
            },
            _ => Expr::Literal(LiteralExpr { value: Some(Object::Bool(true)) }),
        };

        self.consume(TokenType::SemiColon, "Expect ';' after loop condition.")?;

        let increment = match self.peek() {
            Some(token) if token.ttype != TokenType::RightParen => {
                Some(self.expression()?)
            },
            _ => None,
        };

        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Stmt::Block(BlockStmt { statements: vec![body, Stmt::Expression(ExpressionStmt { expression: increment }) ] });
        }

        body = Stmt::While(WhileStmt { condition, body: Box::new(body) });
        
        if let Some(initializer) = initializer {
            body = Stmt::Block(BlockStmt { statements: vec![initializer, body] });
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after condition.")?;
    
        let body = Box::new(self.statement()?);
        
        Ok(Stmt::While(WhileStmt {condition, body}))
    }

    fn if_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = match self.peek() {
            Some(token) if token.ttype == TokenType::Else => {
                self.advance();
                Some(Box::new(self.statement()?))
            },
            _ => None 
        };

        Ok(Stmt::If(IfStmt{ condition, then_branch, else_branch}))
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
        let expression = self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after value.")?;
        Ok(Stmt::Print(PrintStmt { expression }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expression = self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(ExpressionStmt { expression }))
    }

    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        let res = match self.peek() {
            Some(token) if token.ttype == TokenType::Var => {
                self.advance();
                self.var_declaration()
            },
            Some(token) if token.ttype == TokenType::Fun => {
                self.advance();
                self.function("function")
            }
            _ => self.statement()
        };

        if res.is_err() {
            self.synchronize();
        }
        res
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {kind} name."))?;
        self.consume(TokenType::LeftParen, &format!("Expect '(' after {kind} name."))?;
        let mut parameters = Vec::new();
        
        if matches!(self.peek(), Some(token) if token.ttype != TokenType::RightParen) {
            parameters.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);
            while matches!(self.peek(), Some(token) if token.ttype == TokenType::Comma) {
                self.advance();
                if parameters.len() >= 255 {
                    if let Some(token) = self.peek() {
                        LoxError::parse_error(&token, "Can't have more than 255 arguments.");
                        self.had_error = true;
                    }
                }
                parameters.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
        self.consume(TokenType::LeftBrace, &format!("Expect '{{' before {kind} body."))?;
        let body = self.block()?;
        Ok(Stmt::Function(FunctionStmt { name, parameters, body }))
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

