use std::{rc::Rc, fmt::Display};

use crate::{callable::*, stmt::*, object::*, error::*, interpreter::*, environment::*};


pub struct LoxFunction {
    declaration: FunctionStmt
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Object]) -> Result<Object, LoxError> {
        let mut environment = Environment::from(Rc::clone(&interpreter.globals));
        for (param, arg) in self.declaration.parameters.iter().zip(arguments.iter()) {
            environment.define(&param.lexeme, arg);
        }

        match interpreter.execute_block(&self.declaration.body, environment) {
            Err(LoxError::Return(value, _)) => Ok(value),
            Err(e) => Err(e),
            Ok(_) => Ok(Object::Nil)
        }
    }

    fn arity(&self) -> usize {
        self.declaration.parameters.len()
    }

    fn name(&self) -> &str {
        ""
    }
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {} >", self.declaration.name.lexeme)
    }
}

impl LoxFunction {
    pub fn new(declaration: FunctionStmt) -> Self {
        Self { declaration }
    }
}

impl Clone for FunctionStmt {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            parameters: Rc::clone(&self.parameters),
            body: Rc::clone(&self.body),
        }
    }
}