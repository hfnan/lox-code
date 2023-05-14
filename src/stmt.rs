use crate::error::*;
use crate::token::*;
use crate::expr::*;

pub trait StmtVisitor {
    type Output;
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Result<Self::Output, LoxError>;
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Result<Self::Output, LoxError>;
    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Result<Self::Output, LoxError>;
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Result<Self::Output, LoxError>;
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<Self::Output, LoxError>;
}

pub enum Stmt {
    Block(BlockStmt),
    Expression(ExpressionStmt),
    If(IfStmt),
    Print(PrintStmt),
    Var(VarStmt),
}

pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

pub struct ExpressionStmt {
    pub expression: Expr,
}

pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

pub struct PrintStmt {
    pub expression: Expr,
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

impl Stmt {
    pub fn accept<U>(&self, visitor: &mut impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        match self {
            Stmt::Block(blockstmt) => blockstmt.accept(visitor),
            Stmt::Expression(expressionstmt) => expressionstmt.accept(visitor),
            Stmt::If(ifstmt) => ifstmt.accept(visitor),
            Stmt::Print(printstmt) => printstmt.accept(visitor),
            Stmt::Var(varstmt) => varstmt.accept(visitor),
        }
    }
}

impl BlockStmt {
    pub fn accept<U>(&self, visitor: &mut impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_block_stmt(self)
    }

}

impl ExpressionStmt {
    pub fn accept<U>(&self, visitor: &mut impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_expression_stmt(self)
    }

}

impl IfStmt {
    pub fn accept<U>(&self, visitor: &mut impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_if_stmt(self)
    }

}

impl PrintStmt {
    pub fn accept<U>(&self, visitor: &mut impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_print_stmt(self)
    }

}

impl VarStmt {
    pub fn accept<U>(&self, visitor: &mut impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_var_stmt(self)
    }

}

