use crate::{Expression, Identifier, Node, Statement, Token};
use monkey_macros::DefaultStatementNode;

#[derive(DefaultStatementNode)]
pub struct LetStatement {
    pub token: Token,
    pub name: Option<Identifier>,
    pub value: Option<Box<dyn Expression>>,
}

#[derive(DefaultStatementNode)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<dyn Expression>>,
}

#[derive(DefaultStatementNode)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Box<dyn Expression>>,
}

use std::fmt;

impl fmt::Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.token_literal())?;
        if let Some(ident) = &self.name {
            write!(f, "{}", ident.value)?;
        }
        write!(f, " = ")?;
        if let Some(value) = &self.value {
            write!(f, "{}", value.to_string())?;
        }
        write!(f, ";")
    }
}

impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.token_literal())?;
        if let Some(return_value) = &self.return_value {
            write!(f, "{}", return_value.to_string())?;
        }
        write!(f, ";")
    }
}

impl fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(expression) = &self.expression {
            write!(f, "{}", expression.to_string())
        } else {
            Ok(())
        }
    }
}
