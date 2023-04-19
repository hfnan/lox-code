use crate::{expr::*, error::LoxError};

pub struct AstPrinter {}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, LoxError> {
        self.parenthesize(&expr.operator.lexeme, vec![&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, LoxError> {
        self.parenthesize("group", vec![&expr.expression])
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<String, LoxError> {
        match &expr.value {
            None => Ok("nil".to_owned()),
            Some(value) => Ok(value.to_string())
        }
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<String, LoxError> {
        self.parenthesize(&expr.operator.lexeme, vec![&expr.right])
    }
}

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> Result<String, LoxError> {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: Vec<&Expr>) -> Result<String, LoxError> {
        let mut builder = format!("({name}");

        for expr in exprs {
            builder.push_str(&format!(" {}", expr.accept(self)?));
        }
        builder.push(')');

        Ok(builder)
    } 
}