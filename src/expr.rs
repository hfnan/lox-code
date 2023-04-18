use crate::error::*;
use crate::token::*;

pub trait ExprVisitor<T> {
   fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, LoxError>;
   fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, LoxError>;
   fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, LoxError>;
   fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, LoxError>;
}

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

pub struct LiteralExpr {
    pub value: Option<Literal>,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Expr {
    pub fn accept<T>(&self, visitor: &impl ExprVisitor<T>) -> Result<T, LoxError> {
        match self {
            Expr::Binary(binary) => binary.accept(visitor),
            Expr::Grouping(grouping) => grouping.accept(visitor),
            Expr::Literal(literal) => literal.accept(visitor),
            Expr::Unary(unary) => unary.accept(visitor),
        }
    }
}

impl BinaryExpr {
    pub fn accept<T>(&self, visitor: &impl ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_binary_expr(self)
    }

}

impl GroupingExpr {
    pub fn accept<T>(&self, visitor: &impl ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_grouping_expr(self)
    }

}

impl LiteralExpr {
    pub fn accept<T>(&self, visitor: &impl ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_literal_expr(self)
    }

}

impl UnaryExpr {
    pub fn accept<T>(&self, visitor: &impl ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_unary_expr(self)
    }

}

