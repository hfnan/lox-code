use std::collections::HashMap;

use crate::object::Object;
use crate::token::*;
use crate::error::LoxError;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Object, LoxError> {
        if let Some(object) = self.values.get(&name.lexeme).cloned() {
            Ok(object)
        } else {
            Err(LoxError::runtime_error(Some(&name), Some(&format!("Undifined variable '{}'.", name.lexeme))))
        }
    } 
}