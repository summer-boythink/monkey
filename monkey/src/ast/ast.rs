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

#[derive(DefaultExpressionNode, Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

#[derive(DefaultExpressionNode, Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
}

impl std::fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}{})", self.operator, self.right.as_ref().unwrap())
    }
}

pub struct InfixExpression {
    pub token: Token, // The operator token, e.g. +
    pub left: Option<Box<dyn Expression>>,
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
}

impl fmt::Display for InfixExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({} {} {})",
            self.left.as_ref().unwrap(),
            self.operator,
            self.right.as_ref().unwrap()
        )
    }
}
