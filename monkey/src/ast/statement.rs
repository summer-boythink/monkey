use monkey_macros::DefaultStatementNode;

use crate::{Expression, Identifier, Token};

#[derive(DefaultStatementNode)]
pub struct LetStatement {
    pub token: Token,
    pub name: *mut Identifier,
    pub value: dyn Expression,
}
