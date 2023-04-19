use crate::{expr::*, error::LoxError, token::*};

use std::fmt;

#[derive(Debug, Clone)] 
pub enum Object {
    Num(f64),
    Str(String),
    False,
    True,
    Nil,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(x) => write!(f, "{x}"),
            Self::Str(x) => write!(f, "\"{x}\""),
            Self::False => write!(f, "false"),
            Self::True => write!(f, "true"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

pub struct Interpreter {

}

impl ExprVisitor for Interpreter {
    type Output = Object;
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Self::Output, LoxError> {
        Ok(Object::Nil)
    }   

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Self::Output, LoxError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Self::Output, LoxError> {
        expr.value.clone().ok_or(LoxError::evalerror())
    }
    

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Self::Output, LoxError> {
        // a kind of awkward
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Minus => match right {
                Object::Num(val) => Ok(Object::Num(-val)),
                _ => Err(LoxError::evalerror())
            },
            TokenType::Bang => self.reverse(self.is_truthy(&right)),

            _ => Err(LoxError::evalerror())
        }
    }
}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    fn reverse(&self, object: Object) -> Result<Object, LoxError> {
        match object {
            Object::True => Ok(Object::False),
            Object::False => Ok(Object::True),
            _ => Err(LoxError::evalerror())
        }
    }

    fn is_truthy(&self, object: &Object) -> Object {
        match object {
            Object::Nil => Object::False,
            Object::False | Object::True => object.clone(), 
            _ => Object::True,
        }
    } 
}