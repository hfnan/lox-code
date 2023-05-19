use crate::loxfunction::LoxFunction;
use crate::{object::Object, expr::*, error::LoxError, token::*, stmt::*, callable::*, environment::Environment};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
    locals: HashMap<Expr, usize>,
}

impl ExprVisitor for Interpreter {
    type Output = Object;

    fn visit_call_expr(&mut self, expr: Rc<CallExpr>) -> Result<Self::Output, LoxError> {
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
                        LoxError::report(expr.paren.line, "", message);
                    }
                    Err(e)
                }, Ok)
            }
        } else {
            Err(LoxError::runtime_error(&expr.paren, "Can only call functions and classes."))
        }
    }

    fn visit_logical_expr(&mut self, expr: Rc<LogicalExpr>) -> Result<Self::Output, LoxError> {
        let left = self.evaluate(&expr.left)?;

        match expr.operator.ttype {
            TokenType::Or if Self::is_truthy(&left) => Ok(left),
            TokenType::And if !Self::is_truthy(&left) => Ok(left),
            _ => self.evaluate(&expr.right)
        }
    }

    fn visit_assign_expr(&mut self, expr: Rc<AssignExpr>) -> Result<Self::Output, LoxError> {
        let value = self.evaluate(&expr.value)?;
        if let Some(distance) = self.locals.get(&Expr::Assign(Rc::clone(&expr))) {
            self.environment.borrow_mut().assign_at(*distance, &expr.name, value.clone())?;
        } else {
            self.globals.borrow_mut().assign(&expr.name, value.clone())?;
        }
        Ok(value)
    }

    fn visit_binary_expr(&mut self, expr: Rc<BinaryExpr>) -> Result<Self::Output, LoxError> {
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

    fn visit_grouping_expr(&mut self, expr: Rc<GroupingExpr>) -> Result<Self::Output, LoxError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&mut self, expr: Rc<LiteralExpr>) -> Result<Self::Output, LoxError> {
        expr.value.clone().ok_or_else(|| LoxError::object_error("There is no valid literal!"))
    }
    

    fn visit_unary_expr(&mut self, expr: Rc<UnaryExpr>) -> Result<Self::Output, LoxError> {
        // a kind of awkward
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Minus => - right,
            TokenType::Bang => ! right,
            _ => Err(LoxError::runtime_error(&expr.operator, "Cannot use operator like unary."))
        }
    }

    fn visit_variable_expr(&mut self, expr: Rc<VariableExpr>) -> Result<Self::Output, LoxError> {
        self.look_up_variable(&expr.name, Expr::Variable(Rc::clone(&expr)))
    }
}

impl StmtVisitor for Interpreter {
    type Output = ();

    fn visit_return_stmt(&mut self, stmt: Rc<ReturnStmt>) -> Result<Self::Output, LoxError> {
        let value = self.evaluate(&stmt.value)?;
        Err(LoxError::Return(value, stmt.keyword.line))
    }

    fn visit_function_stmt(&mut self, stmt: Rc<FunctionStmt>) -> Result<Self::Output, LoxError> {
        let function = LoxFunction::new(Rc::clone(&stmt), Rc::clone(&self.environment));
        self.environment.borrow_mut().define(&stmt.name.lexeme, &Object::Func(Rc::new(function)));
        Ok(())
    }

    fn visit_break_stmt(&mut self, stmt: Rc<BreakStmt>) -> Result<Self::Output, LoxError> {
        Err(LoxError::Break(stmt.line)) 
    }

    fn visit_while_stmt(&mut self, stmt: Rc<WhileStmt>) -> Result<Self::Output, LoxError> {
        while Self::is_truthy(&self.evaluate(&stmt.condition)?) {
            match self.execute(&stmt.body) {
                Err(LoxError::Break(_)) => break,
                Err(e) => return Err(e),
                _ => (),
            }
        } 
        Ok(())
    }
    
    fn visit_if_stmt(&mut self, stmt: Rc<IfStmt>) -> Result<Self::Output, LoxError> {
        if !matches!(self.evaluate(&stmt.condition)?, Object::Nil | Object::Bool(false)) {
            self.execute(&stmt.then_branch)
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch)
        } else {
            Ok(())
        }
    }

    fn visit_block_stmt(&mut self, stmt: Rc<BlockStmt>) -> Result<Self::Output, LoxError> {
        self.execute_block(&stmt.statements, Environment::from(Rc::clone(&self.environment)))
    }

    fn visit_expression_stmt(&mut self, stmt: Rc<ExpressionStmt>) -> Result<Self::Output, LoxError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: Rc<PrintStmt>) -> Result<Self::Output, LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: Rc<VarStmt>) -> Result<Self::Output, LoxError> {
        let value = if stmt.initializer.is_some() {
            self.evaluate(stmt.initializer.as_ref().unwrap())?
        } else {
            Object::Nil
        };
        
        self.environment.borrow_mut().define(&stmt.name.lexeme, &value);
        Ok(())
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        globals.borrow_mut().define("clock", &Object::Func(Rc::new(NativeClock)));

        Self {
            globals: Rc::clone(&globals), 
            environment: Rc::clone(&globals),
            locals: HashMap::new()
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

    pub fn execute_block(&mut self, statements: &[Stmt], environment: Environment) -> Result<(), LoxError> {
        let previous = Rc::clone(&self.environment);
        self.environment = Rc::new(RefCell::new(environment));

        let res = statements.iter().try_for_each(|stmt| self.execute(stmt));

        self.environment = previous;        
        res
    }

    pub fn resolve(&mut self, expr: Expr, depth: usize) -> Result<(), LoxError> {
        self.locals.insert(expr, depth);
        Ok(())
    }

    fn look_up_variable(&mut self, name: &Token, expr: Expr) -> Result<Object, LoxError> {
        if let Some(distance) = self.locals.get(&expr) {
            self.environment.borrow().get_at(*distance, &name.lexeme)
        } else {
            self.globals.borrow().get(name)
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            match self.execute(&stmt) {
                Err(LoxError::Break(line)) => {
                    LoxError::report(line, "", "'break' outside loop.");
                    break;
                },
                Err(LoxError::Return(_, line)) => {
                    LoxError::report(line, "", "'return' outside a funcion.");
                    break;
                } 
                Err(_) => break,
                _ => {},
            }
        }
    }
}
