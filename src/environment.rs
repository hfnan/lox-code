use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::env;
use std::rc::Rc;

use crate::object::Object;
use crate::token::*;
use crate::error::LoxError;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new(), enclosing: None }
    }

    pub fn from(enclosing: Rc<RefCell<Self>>) -> Self {
        Self { values: HashMap::new(), enclosing: Some(enclosing)}
    }

    pub fn define(&mut self, name: &str, value: &Object) {
        self.values.insert(name.to_owned(), value.clone());
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxError> {
        if let Some(object) = self.values.get(&name.lexeme).cloned() {
            Ok(object)
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(name)
        } else {
            Err(LoxError::runtime_error(&name, &format!("Undifined variable '{}'.", name.lexeme)))
        }
    } 

    pub fn get_at(&self, distance: usize, name: &str) -> Result<Object, LoxError> {
        if distance == 0 {
            Ok(self.values.get(name).unwrap().clone())
        }
        else {
            Ok(self.ancestor(distance).borrow().values.get(name).unwrap().clone())
        }
    }

    pub fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>> {
        let mut environ = Rc::clone(&self.enclosing.as_ref().unwrap());
        for _ in 1..distance {
            let tmp = Rc::clone(&environ.borrow().enclosing.as_ref().unwrap());
            environ = tmp;
        }
        Rc::clone(&environ)
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), LoxError> {
        if let Entry::Occupied(mut e) = self.values.entry(name.lexeme.clone()) {
            e.insert(value);
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(LoxError::runtime_error(&name, &format!("Undefined variable '{}'", name.lexeme)))
        }
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: Object) -> Result<(), LoxError>{
        self.ancestor(distance).borrow_mut().values.insert(name.lexeme.clone(), value);
        Ok(())
    }
}