use crate::{object::Object, expr::*, error::LoxError, token::*, stmt::*, environment::Environment};

pub struct Interpreter {
    environment: Environment,
}

impl ExprVisitor for Interpreter {
    type Output = Object;

    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<Self::Output, LoxError> {
        let value = self.evaluate(&expr.value)?;
        self.environment.assign(expr.name.clone(), value.clone())?;
        Ok(value)
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<Self::Output, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Plus => left + right,
            TokenType::Minus => left - right,
            TokenType::Star => left * right,
            TokenType::Slash => left / right,
            TokenType::Greater => left.greater(right),
            TokenType::GreaterEqual => left.greaterequal(right),
            TokenType::Less => left.less(right),
            TokenType::LessEqual => left.lessequal(right),
            TokenType::BangEqual => left.bangequal(right),
            TokenType::Equal => left.equal(right),
            _ => Err(LoxError::runtime_error(Some(&expr.operator), Some("Unexpected operator in binary expression.")))
        }
    }   

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<Self::Output, LoxError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<Self::Output, LoxError> {
        expr.value.clone().ok_or(LoxError::runtime_error(None, Some("There is no valid literal!")))
    }
    

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<Self::Output, LoxError> {
        // a kind of awkward
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Minus => - right,
            TokenType::Bang => ! right,
            _ => Err(LoxError::runtime_error(Some(&expr.operator), Some("cannot use operator like unary.")))
        }
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<Self::Output, LoxError> {
        self.environment.get(expr.name.clone())
    }
}

impl StmtVisitor for Interpreter {
    type Output = ();
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Result<Self::Output, LoxError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Result<Self::Output, LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<Self::Output, LoxError> {
        let value = if stmt.initializer.is_some() {
            self.evaluate(stmt.initializer.as_ref().unwrap())?
        } else {
            Object::Nil
        };
        
        self.environment.define(stmt.name.lexeme.clone(), value);
        Ok(())
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new()
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
        stmt.accept(self)
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            if let Err(e) = self.execute(&stmt) {
                e.report("");
            }
        }
    }
}
