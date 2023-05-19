use crate::error::*;
use crate::token::*;
use std::rc::Rc;
use std::hash::Hash;
use crate::object::*;

pub trait ExprVisitor {
    type Output;
    fn visit_assign_expr(&mut self, expr: Rc<AssignExpr>) -> Result<Self::Output, LoxError>;
    fn visit_binary_expr(&mut self, expr: Rc<BinaryExpr>) -> Result<Self::Output, LoxError>;
    fn visit_call_expr(&mut self, expr: Rc<CallExpr>) -> Result<Self::Output, LoxError>;
    fn visit_grouping_expr(&mut self, expr: Rc<GroupingExpr>) -> Result<Self::Output, LoxError>;
    fn visit_literal_expr(&mut self, expr: Rc<LiteralExpr>) -> Result<Self::Output, LoxError>;
    fn visit_logical_expr(&mut self, expr: Rc<LogicalExpr>) -> Result<Self::Output, LoxError>;
    fn visit_unary_expr(&mut self, expr: Rc<UnaryExpr>) -> Result<Self::Output, LoxError>;
    fn visit_variable_expr(&mut self, expr: Rc<VariableExpr>) -> Result<Self::Output, LoxError>;
}

pub enum Expr {
    Assign(Rc<AssignExpr>),
    Binary(Rc<BinaryExpr>),
    Call(Rc<CallExpr>),
    Grouping(Rc<GroupingExpr>),
    Literal(Rc<LiteralExpr>),
    Logical(Rc<LogicalExpr>),
    Unary(Rc<UnaryExpr>),
    Variable(Rc<VariableExpr>),
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

pub struct CallExpr {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

pub struct LiteralExpr {
    pub value: Option<Object>,
}

pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
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
            Expr::Assign(assignstmt) => assignstmt.accept(visitor),
            Expr::Binary(binarystmt) => binarystmt.accept(visitor),
            Expr::Call(callstmt) => callstmt.accept(visitor),
            Expr::Grouping(groupingstmt) => groupingstmt.accept(visitor),
            Expr::Literal(literalstmt) => literalstmt.accept(visitor),
            Expr::Logical(logicalstmt) => logicalstmt.accept(visitor),
            Expr::Unary(unarystmt) => unarystmt.accept(visitor),
            Expr::Variable(variablestmt) => variablestmt.accept(visitor),
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Assign(a), Expr::Assign(b)) => Rc::ptr_eq(a, b),
            (Expr::Binary(a), Expr::Binary(b)) => Rc::ptr_eq(a, b),
            (Expr::Call(a), Expr::Call(b)) => Rc::ptr_eq(a, b),
            (Expr::Grouping(a), Expr::Grouping(b)) => Rc::ptr_eq(a, b),
            (Expr::Literal(a), Expr::Literal(b)) => Rc::ptr_eq(a, b),
            (Expr::Logical(a), Expr::Logical(b)) => Rc::ptr_eq(a, b),
            (Expr::Unary(a), Expr::Unary(b)) => Rc::ptr_eq(a, b),
            (Expr::Variable(a), Expr::Variable(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl Eq for Expr {}

impl Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        match self {
            Expr::Assign(a) => hasher.write_usize(Rc::as_ptr(a) as usize),
            Expr::Binary(a) => hasher.write_usize(Rc::as_ptr(a) as usize),
            Expr::Call(a) => hasher.write_usize(Rc::as_ptr(a) as usize),
            Expr::Grouping(a) => hasher.write_usize(Rc::as_ptr(a) as usize),
            Expr::Literal(a) => hasher.write_usize(Rc::as_ptr(a) as usize),
            Expr::Logical(a) => hasher.write_usize(Rc::as_ptr(a) as usize),
            Expr::Unary(a) => hasher.write_usize(Rc::as_ptr(a) as usize),
            Expr::Variable(a) => hasher.write_usize(Rc::as_ptr(a) as usize),
        }
    }
}

impl AssignExpr {
    pub fn accept<U>(self: &Rc<AssignExpr>, visitor: &mut impl ExprVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_assign_expr(Rc::clone(self))
    }

}

impl BinaryExpr {
    pub fn accept<U>(self: &Rc<BinaryExpr>, visitor: &mut impl ExprVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_binary_expr(Rc::clone(self))
    }

}

impl CallExpr {
    pub fn accept<U>(self: &Rc<CallExpr>, visitor: &mut impl ExprVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_call_expr(Rc::clone(self))
    }

}

impl GroupingExpr {
    pub fn accept<U>(self: &Rc<GroupingExpr>, visitor: &mut impl ExprVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_grouping_expr(Rc::clone(self))
    }

}

impl LiteralExpr {
    pub fn accept<U>(self: &Rc<LiteralExpr>, visitor: &mut impl ExprVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_literal_expr(Rc::clone(self))
    }

}

impl LogicalExpr {
    pub fn accept<U>(self: &Rc<LogicalExpr>, visitor: &mut impl ExprVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_logical_expr(Rc::clone(self))
    }

}

impl UnaryExpr {
    pub fn accept<U>(self: &Rc<UnaryExpr>, visitor: &mut impl ExprVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_unary_expr(Rc::clone(self))
    }

}

impl VariableExpr {
    pub fn accept<U>(self: &Rc<VariableExpr>, visitor: &mut impl ExprVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_variable_expr(Rc::clone(self))
    }

}

