use crate::error::*;
use crate::token::*;
use crate::object::*;

pub trait ExprVisitor {
    type Output;
    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<Self::Output, LoxError>;
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<Self::Output, LoxError>;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<Self::Output, LoxError>;
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<Self::Output, LoxError>;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<Self::Output, LoxError>;
    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<Self::Output, LoxError>;
}

pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
}

pub struct AssignExpr {
    pub name: Token,
    pub value: Box<Expr>,
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
    pub value: Option<Object>,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct VariableExpr {
    pub name: Token,
}

impl Expr {
    pub fn accept<U>(&self, visitor: &mut impl ExprVisitor<Output = U>) -> Result<U, LoxError> {
        match self {
            Expr::Assign(assign) => assign.accept(visitor),
            Expr::Binary(binary) => binary.accept(visitor),
            Expr::Grouping(grouping) => grouping.accept(visitor),
            Expr::Literal(literal) => literal.accept(visitor),
            Expr::Unary(unary) => unary.accept(visitor),
            Expr::Variable(variable) => variable.accept(visitor),
        }
    }
}

impl AssignExpr {
    pub fn accept<U>(&self, visitor: &mut impl ExprVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_assign_expr(self)
    }

}

impl BinaryExpr {
    pub fn accept<U>(&self, visitor: &mut impl ExprVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_binary_expr(self)
    }

}

impl GroupingExpr {
    pub fn accept<U>(&self, visitor: &mut impl ExprVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_grouping_expr(self)
    }

}

impl LiteralExpr {
    pub fn accept<U>(&self, visitor: &mut impl ExprVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_literal_expr(self)
    }

}

impl UnaryExpr {
    pub fn accept<U>(&self, visitor: &mut impl ExprVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_unary_expr(self)
    }

}

impl VariableExpr {
    pub fn accept<U>(&self, visitor: &mut impl ExprVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_variable_expr(self)
    }

}

