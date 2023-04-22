use crate::error::*;
use crate::token::*;
use crate::object::*;
use crate::expr::*;

pub trait StmtVisitor {
    type Output;
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<Self::Output, LoxError>;
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<Self::Output, LoxError>;
    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<Self::Output, LoxError>;
}

pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
}

pub struct ExpressionStmt {
    pub expression: Box<Expr>,
}

pub struct PrintStmt {
    pub expression: Box<Expr>,
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

impl Stmt {
    pub fn accept<U>(&self, visitor: &impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        match self {
            Stmt::Expression(expression) => expression.accept(visitor),
            Stmt::Print(print) => print.accept(visitor),
            Stmt::Var(var) => var.accept(visitor),
        }
    }
}

impl ExpressionStmt {
    pub fn accept<U>(&self, visitor: &impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_expression_stmt(self)
    }

}

impl PrintStmt {
    pub fn accept<U>(&self, visitor: &impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_print_stmt(self)
    }

}

impl VarStmt {
    pub fn accept<U>(&self, visitor: &impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_var_stmt(self)
    }

}

