use crate::callable::Callable;
use crate::error::LoxError;

use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg, Not};

#[derive(Debug, Clone, PartialEq, PartialOrd)] 
pub enum Object {
    Num(f64),
    Str(String),
    Bool(bool),
    Func(Callable),
    Nil,
}


impl Object {
    pub fn greater(&self, rhs: Self) -> Result<Self, LoxError> {
        match (self, &rhs) {
            (Object::Num(_), Object::Num(_)) => Ok(Object::Bool(self > &rhs)),
            _ => Err(LoxError::object_error("Operator '>' need two Num operands."))
        }
    }

    pub fn greaterequal(&self, rhs: Self) -> Result<Self, LoxError> {
        match (self, &rhs) {
            (Object::Num(_), Object::Num(_)) => Ok(Object::Bool(self >= &rhs)),
            _ => Err(LoxError::object_error("Operator '>=' need two Num operands."))
        }
    }

    pub fn less(&self, rhs: Self) -> Result<Self, LoxError> {
        match (self, &rhs) {
            (Object::Num(_), Object::Num(_)) => Ok(Object::Bool(self < &rhs)),
            _ => Err(LoxError::object_error("Operator '<' need two Num operands."))
        }
    }

    pub fn lessequal(&self, rhs: Self) -> Result<Self, LoxError> {
        match (self, &rhs) {
            (Object::Num(_), Object::Num(_)) => Ok(Object::Bool(self >= &rhs)),
            _ => Err(LoxError::object_error("Operator '<=' need two Num operands."))
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
            _ => unreachable!()
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
            Self::Func(_) => Ok(())
        }
    }
}

impl Neg for Object {
    type Output = Result<Object, LoxError>; 

    fn neg(self) -> Self::Output {
        match self {
            Object::Num(val) => Ok(Object::Num(-val)),
            _ => Err(LoxError::object_error("Prefix operator '-' need Num operand."))
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
            _ => Err(LoxError::object_error("Unexpected Type of operands for operator '+'."))
        } 
    }
}

impl Sub for Object {
    type Output = Result<Object, LoxError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Num(left), Object::Num(right)) => Ok(Object::Num(left - right)),
            _ => Err(LoxError::object_error("Operator '-' need two Num operands."))
        } 
    }
}

impl Mul for Object {
    type Output = Result<Object, LoxError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
             (Object::Num(left), Object::Num(right)) => Ok(Object::Num(left * right)),
            _ => Err(LoxError::object_error("Operator '*' need two Num operands."))
        }
    }
}

impl Div for Object {
    type Output = Result<Object, LoxError>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
             (Object::Num(left), Object::Num(right)) => Ok(Object::Num(left / right)),
            _ => Err(LoxError::object_error("Operator '/' need two Num operands."))
        }
    }
}
