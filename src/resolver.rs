use std::collections::HashMap;
use std::rc::Rc;

use crate::{interpreter::Interpreter, expr::*, stmt::*, error::*, token::Token};

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    had_error: bool
}

impl<'a> ExprVisitor for Resolver<'a> {
    type Output = ();
    
    fn visit_variable_expr(&mut self, expr: Rc<VariableExpr>) -> Result<Self::Output, LoxError> {
        if matches!(self.scopes.last_mut(), Some(scope) if matches!(scope.get(&expr.name.lexeme), Some(false))) {
            LoxError::parse_error(&expr.name, "Can't read local variable in its own initializer.");
            self.had_error = true;
        } 
        self.resolve_local(Expr::Variable(Rc::clone(&expr)), &expr.name)?;
        Ok(())
    }
    
    fn visit_assign_expr(&mut self, expr: Rc<AssignExpr>) -> Result<Self::Output, LoxError> {
        self.resolve_expr(&expr.value)?;
        self.resolve_local(Expr::Assign(Rc::clone(&expr)), &expr.name)?;
        Ok(())
    }

    fn visit_binary_expr(&mut self, expr: Rc<BinaryExpr>) -> Result<Self::Output, LoxError> {
        self.resolve_expr(&expr.left)?;
        self.resolve_expr(&expr.right)
    }

    fn visit_call_expr(&mut self, expr: Rc<CallExpr>) -> Result<Self::Output, LoxError> {
        self.resolve_expr(&expr.callee)?;
        for arg in &expr.arguments {
            self.resolve_expr(arg)?;
        }
        Ok(())
    }

    fn visit_grouping_expr(&mut self, expr: Rc<GroupingExpr>) -> Result<Self::Output, LoxError> {
        self.resolve_expr(&expr.expression)
    }

    fn visit_literal_expr(&mut self, _expr: Rc<LiteralExpr>) -> Result<Self::Output, LoxError> {
        Ok(())
    }

    fn visit_logical_expr(&mut self, expr: Rc<LogicalExpr>) -> Result<Self::Output, LoxError> {
        self.resolve_expr(&expr.left)?;
        self.resolve_expr(&expr.right)
    }

    fn visit_unary_expr(&mut self, expr: Rc<UnaryExpr>) -> Result<Self::Output, LoxError> {
        self.resolve_expr(&expr.right)
    }

    
}

impl<'a> StmtVisitor for Resolver<'a> {
    type Output = ();
    fn visit_block_stmt(&mut self, stmt: Rc<BlockStmt>) -> Result<Self::Output,LoxError> {
        self.begin_scope();
        self.resolve(&stmt.statements)?;
        self.end_scope();
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: Rc<VarStmt>) -> Result<Self::Output, LoxError> {
        self.declare(&stmt.name);
        if let Some(init) = &stmt.initializer {
            self.resolve_expr(init)?;
        }
        self.define(&stmt.name);
        Ok(())
    }
    
    fn visit_function_stmt(&mut self, stmt: Rc<FunctionStmt>) -> Result<Self::Output, LoxError> {
        self.declare(&stmt.name);
        self.define(&stmt.name);    // This lets a function recursively refer to itself inside its own body.

        self.resolve_function(stmt)?;
        Ok(())
    }

    fn visit_break_stmt(&mut self, _stmt: Rc<BreakStmt>) -> Result<Self::Output, LoxError> {
        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: Rc<ExpressionStmt>) -> Result<Self::Output, LoxError> {
        self.resolve_expr(&stmt.expression)
    }

    fn visit_if_stmt(&mut self, stmt: Rc<IfStmt>) -> Result<Self::Output, LoxError> {
        self.resolve_expr(&stmt.condition)?;
        self.resolve_stmt(&stmt.then_branch)?;
        if let Some(else_branch) = &stmt.else_branch {
            self.resolve_stmt(else_branch)?;
        }
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: Rc<PrintStmt>) -> Result<Self::Output, LoxError> {
        self.resolve_expr(&stmt.expression)
    }

    fn visit_return_stmt(&mut self, stmt: Rc<ReturnStmt>) -> Result<Self::Output, LoxError> {
        self.resolve_expr(&stmt.value)
    }

    fn visit_while_stmt(&mut self, stmt: Rc<WhileStmt>) -> Result<Self::Output, LoxError> {
        self.resolve_expr(&stmt.condition)?;
        self.resolve_stmt(&stmt.body)
    }
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self { interpreter, scopes: Vec::new() ,had_error: false }
    }

    pub fn success(&self)-> bool {
        !self.had_error
    }

    pub fn resolve(&mut self, statements: &[Stmt]) -> Result<(), LoxError> {
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

    fn resolve_local(&mut self, expr: Expr, name: &Token) -> Result<(), LoxError> {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if let Some(_) = scope.get(&name.lexeme) {
                return self.interpreter.resolve(expr, i)
            } 
        }
        Ok(())
    }

    fn resolve_function(&mut self, function: Rc<FunctionStmt>) -> Result<(), LoxError>{
        self.begin_scope();
        for param in function.parameters.iter() {
            self.declare(param);
            self.define(param);
        }
        self.resolve(&function.body)?;
        self.end_scope();
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
            if scope.contains_key(&name.lexeme) {
                LoxError::parse_error(name, "Already variable with this name in this scope.");
                self.had_error = true;
            }
            scope.insert(name.lexeme.to_owned(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.to_owned(), true);
        }
    }
}