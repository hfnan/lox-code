use std::{time::SystemTime, fmt::Display};

use crate::{interpreter::Interpreter, object::Object, error::LoxError};

pub trait LoxCallable: Display {
    fn arity(&self) -> usize;
    fn name(&self) -> &str;
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Object]) -> Result<Object, LoxError>; 
}

// For native clock function
pub struct NativeClock;
impl LoxCallable for NativeClock {
    fn arity(&self) -> usize {
        0
    }

    fn name(&self) -> &str {
        "clock"
    }

    fn call(&self, _interpreter: &mut Interpreter, _arguments: &[Object]) -> Result<Object, LoxError> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => Ok(Object::Num(n.as_secs_f64())),
            Err(_) => Err(LoxError::object_error("SystemTime before UNIX EPOCH!"))
        }
    }
}

impl Display for NativeClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}
