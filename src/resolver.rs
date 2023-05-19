use std::collections::HashMap;

use crate::{interpreter::Interpreter, expr::*, stmt::*, object::*, error::*, token::Token};

pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    had_error: bool
}

impl ExprVisitor for Resolver {
    type Output = ();
    
    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<Self::Output, LoxError> {
        if matches!(self.scopes.last_mut(), Some(scope) if matches!(scope.get(&expr.name.lexeme), Some(false))) {
            LoxError::parse_error(&expr.name, "Can't read local variable in its own initializer.");
            self.had_error = true;
        } 
        self.resolve_local(expr, expr.name)?;
        Ok(())
    }
    
    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<Self::Output, LoxError> {
        todo!()
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<Self::Output, LoxError> {
        todo!()
    }

    fn visit_call_expr(&mut self, expr: &CallExpr) -> Result<Self::Output, LoxError> {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<Self::Output, LoxError> {
        todo!()
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<Self::Output, LoxError> {
        todo!()
    }

    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> Result<Self::Output, LoxError> {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<Self::Output, LoxError> {
        todo!()
    }

    
}

impl StmtVisitor for Resolver {
    type Output = ();
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Result<Self::Output,LoxError> {
        self.begin_scope();
        self.resolve(&stmt.statements)?;
        self.end_scope();
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<Self::Output, LoxError> {
        self.declare(&stmt.name);
        if let Some(init) = &stmt.initializer {
            self.resolve_expr(init)?;
        }
        self.define(&stmt.name);
        Ok(())
    }

    fn visit_break_stmt(&mut self, stmt: &BreakStmt) -> Result<Self::Output, LoxError> {
        todo!()
    }

    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Result<Self::Output, LoxError> {
        todo!()
    }

    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Result<Self::Output, LoxError> {
        todo!()
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Result<Self::Output, LoxError> {
        todo!()
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Result<Self::Output, LoxError> {
        todo!()
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Result<Self::Output, LoxError> {
        todo!()
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Result<Self::Output, LoxError> {
        todo!()
    }
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Self { interpreter, scopes: Vec::new() ,had_error: false }
    }

    pub fn success(&self)-> bool {
        !self.had_error
    }

    fn resolve(&mut self, statements: &[Stmt]) -> Result<(), LoxError> {
        for statement in statements {
            self.resolve_stmt(statement)?
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
        stmt.accept(self)
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), LoxError> {
        expr.accept(self)
    }

    fn resolve_local(&mut self, expr: &Expr, name: Token) -> Result<(), LoxError> {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if let Some(_) = scope.get(&name.lexeme) {
                return self.interpreter.resolve(expr, i)
            } 
        }
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop().unwrap();
    }

    fn declare(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.to_owned(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.to_owned(), true);
        }
    }
}