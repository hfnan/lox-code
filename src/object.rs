use crate::error::LoxError;

use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg, Not};

#[derive(Debug, Clone, PartialEq, PartialOrd)] 
pub enum Object {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl Object {
    pub fn greater(&self, rhs: Self) -> Result<Self, LoxError> {
        match (self, &rhs) {
            (Object::Num(_), Object::Num(_)) => Ok(Object::Bool(self > &rhs)),
            _ => Err(LoxError::runtime_error(None, None))
        }
    }

    pub fn greaterequal(&self, rhs: Self) -> Result<Self, LoxError> {
        match (self, &rhs) {
            (Object::Num(_), Object::Num(_)) => Ok(Object::Bool(self >= &rhs)),
            _ => Err(LoxError::runtime_error(None, None))
        }
    }

    pub fn less(&self, rhs: Self) -> Result<Self, LoxError> {
        match (self, &rhs) {
            (Object::Num(_), Object::Num(_)) => Ok(Object::Bool(self < &rhs)),
            _ => Err(LoxError::runtime_error(None, None))
        }
    }

    pub fn lessequal(&self, rhs: Self) -> Result<Self, LoxError> {
        match (self, &rhs) {
            (Object::Num(_), Object::Num(_)) => Ok(Object::Bool(self >= &rhs)),
            _ => Err(LoxError::runtime_error(None, None))
        }
    }

    pub fn equal(&self, rhs: Self) -> Result<Self, LoxError> {
        match (self, &rhs) {
            (Object::Nil, Object::Nil) => Ok(Object::Bool(true)),
            (Object::Nil, _) | (_, Object::Nil) => Ok(Object::Bool(false)),
            _ => Ok(Object::Bool(self == &rhs))
        }
    }

    pub fn bangequal(&self, rhs: Self) -> Result<Self, LoxError> {
        match self.equal(rhs) {
            Ok(obj) => !obj,
            _ => Err(LoxError::runtime_error(None, None))
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(x) => write!(f, "{x}"),
            Self::Str(x) => write!(f, "{x}"),
            Self::Bool(x) => if *x {write!(f, "true")} else {write!(f, "false")},
            Self::Nil => write!(f, "nil"),
        }
    }
}

impl Neg for Object {
    type Output = Result<Object, LoxError>; 

    fn neg(self) -> Self::Output {
        match self {
            Object::Num(val) => Ok(Object::Num(-val)),
            _ => Err(LoxError::runtime_error(None, None)),
        }
    }
}

impl Not for Object {
    type Output = Result<Object, LoxError>; 

    fn not(self) -> Self::Output {
        match self {
            Object::Nil | Object::Bool(false) => Ok(Object::Bool(true)),
            _ => Ok(Object::Bool(false))
        }
    }
}



impl Add for Object {
    type Output = Result<Object, LoxError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Num(left), Object::Num(right)) => Ok(Object::Num(left + right)),
            (Object::Str(left), Object::Str(right)) => Ok(Object::Str(format!("{left}{right}"))),
            (Object::Num(left), Object::Str(right)) => Ok(Object::Str(format!("{left}{right}"))),
            (Object::Str(left), Object::Num(right)) => Ok(Object::Str(format!("{left}{right}"))),
            _ => Err(LoxError::runtime_error(None, None))
        } 
    }
}

impl Sub for Object {
    type Output = Result<Object, LoxError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Num(left), Object::Num(right)) => Ok(Object::Num(left - right)),
            _ => Err(LoxError::runtime_error(None, None))
        } 
    }
}

impl Mul for Object {
    type Output = Result<Object, LoxError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
             (Object::Num(left), Object::Num(right)) => Ok(Object::Num(left * right)),
            _ => Err(LoxError::runtime_error(None, None))
        }
    }
}

impl Div for Object {
    type Output = Result<Object, LoxError>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
             (Object::Num(left), Object::Num(right)) => Ok(Object::Num(left / right)),
            _ => Err(LoxError::runtime_error(None, None))
        }
    }
}
