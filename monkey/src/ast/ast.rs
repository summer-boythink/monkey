use core::fmt;
use monkey_macros::DefaultExpressionNode;
use std::any::Any;

use crate::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node + fmt::Display {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node + fmt::Display {
    fn expression_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for s in &self.statements {
            write!(f, "{}", s.to_string())?;
        }
        Ok(())
    }
}

#[derive(DefaultExpressionNode, Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}
