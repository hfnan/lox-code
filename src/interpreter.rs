use crate::{object::Object, expr::*, error::LoxError, token::*, stmt::*};

pub struct Interpreter {

}

impl ExprVisitor for Interpreter {
    type Output = Object;
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Self::Output, LoxError> {
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

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Self::Output, LoxError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Self::Output, LoxError> {
        expr.value.clone().ok_or(LoxError::runtime_error(None, Some("There is no valid literal!")))
    }
    

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Self::Output, LoxError> {
        // a kind of awkward
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Minus => - right,
            TokenType::Bang => ! right,
            _ => Err(LoxError::runtime_error(Some(&expr.operator), Some("cannot use operator like unary.")))
        }
    }
}

impl StmtVisitor for Interpreter {
    type Output = ();
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<Self::Output, LoxError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<Self::Output, LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }
}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    fn execute(&self, stmt: &Stmt) -> Result<(), LoxError> {
        stmt.accept(self)
    }

    pub fn interpret(&self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            if let Err(e) = self.execute(&stmt) {
                e.report("");
            }
        }
    }
}
