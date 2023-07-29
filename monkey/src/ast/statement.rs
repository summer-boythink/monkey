use crate::{Expression, Identifier, Node, Statement, Token};
use monkey_macros::DefaultStatementNode;

#[derive(DefaultStatementNode)]
pub struct LetStatement {
    pub token: Token,
    pub name: Option<Identifier>,
    pub value: Option<Box<dyn Expression>>,
}
