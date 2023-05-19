use crate::error::*;
use crate::token::*;
use std::rc::Rc;
use crate::expr::*;

pub trait StmtVisitor {
    type Output;
    fn visit_break_stmt(&mut self, stmt: Rc<BreakStmt>) -> Result<Self::Output, LoxError>;
    fn visit_block_stmt(&mut self, stmt: Rc<BlockStmt>) -> Result<Self::Output, LoxError>;
    fn visit_expression_stmt(&mut self, stmt: Rc<ExpressionStmt>) -> Result<Self::Output, LoxError>;
    fn visit_function_stmt(&mut self, stmt: Rc<FunctionStmt>) -> Result<Self::Output, LoxError>;
    fn visit_if_stmt(&mut self, stmt: Rc<IfStmt>) -> Result<Self::Output, LoxError>;
    fn visit_print_stmt(&mut self, stmt: Rc<PrintStmt>) -> Result<Self::Output, LoxError>;
    fn visit_return_stmt(&mut self, stmt: Rc<ReturnStmt>) -> Result<Self::Output, LoxError>;
    fn visit_var_stmt(&mut self, stmt: Rc<VarStmt>) -> Result<Self::Output, LoxError>;
    fn visit_while_stmt(&mut self, stmt: Rc<WhileStmt>) -> Result<Self::Output, LoxError>;
}

pub enum Stmt {
    Break(Rc<BreakStmt>),
    Block(Rc<BlockStmt>),
    Expression(Rc<ExpressionStmt>),
    Function(Rc<FunctionStmt>),
    If(Rc<IfStmt>),
    Print(Rc<PrintStmt>),
    Return(Rc<ReturnStmt>),
    Var(Rc<VarStmt>),
    While(Rc<WhileStmt>),
}

pub struct BreakStmt {
    pub line: usize,
}

pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

pub struct ExpressionStmt {
    pub expression: Expr,
}

pub struct FunctionStmt {
    pub name: Token,
    pub parameters: Rc<Vec<Token>>,
    pub body: Rc<Vec<Stmt>>,
}

pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

pub struct PrintStmt {
    pub expression: Expr,
}

pub struct ReturnStmt {
    pub keyword: Token,
    pub value: Expr,
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

impl Stmt {
    pub fn accept<U>(&self, visitor: &mut impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        match self {
            Stmt::Break(breakstmt) => breakstmt.accept(visitor),
            Stmt::Block(blockstmt) => blockstmt.accept(visitor),
            Stmt::Expression(expressionstmt) => expressionstmt.accept(visitor),
            Stmt::Function(functionstmt) => functionstmt.accept(visitor),
            Stmt::If(ifstmt) => ifstmt.accept(visitor),
            Stmt::Print(printstmt) => printstmt.accept(visitor),
            Stmt::Return(returnstmt) => returnstmt.accept(visitor),
            Stmt::Var(varstmt) => varstmt.accept(visitor),
            Stmt::While(whilestmt) => whilestmt.accept(visitor),
        }
    }
}

impl BreakStmt {
    pub fn accept<U>(self: &Rc<BreakStmt>, visitor: &mut impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_break_stmt(Rc::clone(self))
    }

}

impl BlockStmt {
    pub fn accept<U>(self: &Rc<BlockStmt>, visitor: &mut impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_block_stmt(Rc::clone(self))
    }

}

impl ExpressionStmt {
    pub fn accept<U>(self: &Rc<ExpressionStmt>, visitor: &mut impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_expression_stmt(Rc::clone(self))
    }

}

impl FunctionStmt {
    pub fn accept<U>(self: &Rc<FunctionStmt>, visitor: &mut impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_function_stmt(Rc::clone(self))
    }

}

impl IfStmt {
    pub fn accept<U>(self: &Rc<IfStmt>, visitor: &mut impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_if_stmt(Rc::clone(self))
    }

}

impl PrintStmt {
    pub fn accept<U>(self: &Rc<PrintStmt>, visitor: &mut impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_print_stmt(Rc::clone(self))
    }

}

impl ReturnStmt {
    pub fn accept<U>(self: &Rc<ReturnStmt>, visitor: &mut impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_return_stmt(Rc::clone(self))
    }

}

impl VarStmt {
    pub fn accept<U>(self: &Rc<VarStmt>, visitor: &mut impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_var_stmt(Rc::clone(self))
    }

}

impl WhileStmt {
    pub fn accept<U>(self: &Rc<WhileStmt>, visitor: &mut impl StmtVisitor<Output = U>) -> Result<U, LoxError> {
        visitor.visit_while_stmt(Rc::clone(self))
    }

}

