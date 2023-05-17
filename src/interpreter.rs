use crate::{object::Object, expr::*, error::LoxError, token::*, stmt::*, callable::*, environment::Environment};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Interpreter {
    globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl ExprVisitor for Interpreter {
    type Output = Object;

    fn visit_call_expr(&mut self, expr: &CallExpr) -> Result<Self::Output, LoxError> {
        let callee = self.evaluate(&expr.callee)?;
    
        let mut arguments = Vec::new();

        for argument in &expr.arguments {
            arguments.push(self.evaluate(argument)?);
        }

        if let Object::Func(function) = callee {
            if arguments.len() != function.arity() {
                Err(LoxError::runtime_error(&expr.paren,
                    &format!("Expected {} arguments but got {}.",
                    function.arity(), arguments.len())))
            } else {
                function.call(self, &arguments).map_or_else(|e| {
                    if let LoxError::ObjectError(message) = &e {
                        LoxError::report(expr.paren.line, "", &message);
                    }
                    Err(e)
                }, |obj| Ok(obj))
            }
        } else {
            Err(LoxError::runtime_error(&expr.paren, "Can only call functions and classes."))
        }
    }

    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> Result<Self::Output, LoxError> {
        let left = self.evaluate(&expr.left)?;

        match expr.operator.ttype {
            TokenType::Or if Self::is_truthy(&left) => Ok(left),
            TokenType::And if !Self::is_truthy(&left) => Ok(left),
            _ => self.evaluate(&expr.right)
        }
    }

    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<Self::Output, LoxError> {
        let value = self.evaluate(&expr.value)?;
        self.environment.borrow_mut().assign(expr.name.clone(), value.clone())?;
        Ok(value)
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<Self::Output, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        let result = match expr.operator.ttype {
            TokenType::Plus => left + right,
            TokenType::Minus => left - right,
            TokenType::Star => left * right,
            TokenType::Slash => left / right,
            TokenType::Greater => left.greater(right),
            TokenType::GreaterEqual => left.greaterequal(right),
            TokenType::Less => left.less(right),
            TokenType::LessEqual => left.lessequal(right),
            TokenType::BangEqual => left.bangequal(right),
            TokenType::Equal => left.equal(right),
            TokenType::Comma => Ok(right),
            _ => Err(LoxError::runtime_error(&expr.operator, &format!("Unexpected operator '{}' in binary expression.", expr.operator.lexeme)))
        };
        
        if let Err(LoxError::ObjectError(message)) = &result {
            LoxError::report(expr.operator.line, "", message);
        }
        result
    }   

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<Self::Output, LoxError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<Self::Output, LoxError> {
        expr.value.clone().ok_or_else(|| LoxError::object_error("There is no valid literal!"))
    }
    

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<Self::Output, LoxError> {
        // a kind of awkward
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Minus => - right,
            TokenType::Bang => ! right,
            _ => Err(LoxError::runtime_error(&expr.operator, "Cannot use operator like unary."))
        }
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<Self::Output, LoxError> {
        self.environment.borrow().get(expr.name.clone())
    }
}

impl StmtVisitor for Interpreter {
    type Output = ();

    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Result<Self::Output, LoxError> {
        unimplemented!()
    }

    fn visit_break_stmt(&mut self, stmt: &BreakStmt) -> Result<Self::Output, LoxError> {
        Err(LoxError::Break(stmt.line)) 
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Result<Self::Output, LoxError> {
        while Self::is_truthy(&self.evaluate(&stmt.condition)?) {
            match self.execute(&stmt.body) {
                Err(LoxError::Break(_)) => break,
                Err(e) => return Err(e),
                _ => (),
            }
        } 
        Ok(())
    }
    
    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Result<Self::Output, LoxError> {
        if !matches!(self.evaluate(&stmt.condition)?, Object::Nil | Object::Bool(false)) {
            self.execute(&stmt.then_branch)
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch)
        } else {
            Ok(())
        }
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Result<Self::Output, LoxError> {
        self.execute_block(&stmt.statements, Environment::from(Rc::clone(&self.environment)))
    }

    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Result<Self::Output, LoxError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Result<Self::Output, LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<Self::Output, LoxError> {
        let value = if stmt.initializer.is_some() {
            self.evaluate(stmt.initializer.as_ref().unwrap())?
        } else {
            Object::Nil
        };
        
        self.environment.borrow_mut().define(stmt.name.lexeme.clone(), value);
        Ok(())
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        globals.borrow_mut().define("clock".to_owned(), Object::Func(Rc::new(NativeClock)));

        Self {
            globals: Rc::clone(&globals), 
            environment: Rc::clone(&globals)
        }
    }

    fn is_truthy(object: &Object) -> bool {
        !matches!(object, Object::Bool(false) | Object::Nil)
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
        stmt.accept(self) 
    }

    fn execute_block(&mut self, statements: &[Stmt], environment: Environment) -> Result<(), LoxError> {
        let previous = Rc::clone(&self.environment);
        self.environment = Rc::new(RefCell::new(environment));

        let res = statements.iter().try_for_each(|stmt| self.execute(stmt));

        self.environment = previous;        
        res
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            match self.execute(&stmt) {
                Err(LoxError::Break(line)) => {
                    LoxError::report(line, "", "'break' outside loop.");
                    break;
                },
                Err(_) => break,
                _ => {},
            }
        }
    }
}
