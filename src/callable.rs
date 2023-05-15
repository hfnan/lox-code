use crate::{interpreter::Interpreter, object::Object, error::LoxError};


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Callable {
}

impl Callable {
    pub fn call(&self, interpreter: &mut Interpreter, arguments: &[Object]) -> Result<Object, LoxError> {
        todo!()
    }
}