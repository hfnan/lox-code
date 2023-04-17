use crate::error::*;
use crate::token::*;

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

pub trait ExprVisitor<T> {
   fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, LoxError>;
   fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, LoxError>;
   fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, LoxError>;
   fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, LoxError>;
}

pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

pub struct GroupingExpr {
    expression: Box<Expr>,
}

pub struct LiteralExpr {
    value: Literal,
}

pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

impl BinaryExpr {
    fn accept<T>(&self, visitor: &impl ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_binary_expr(self)
    }

}

impl GroupingExpr {
    fn accept<T>(&self, visitor: &impl ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_grouping_expr(self)
    }

}

impl LiteralExpr {
    fn accept<T>(&self, visitor: &impl ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_literal_expr(self)
    }

}

impl UnaryExpr {
    fn accept<T>(&self, visitor: &impl ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_unary_expr(self)
    }

}

