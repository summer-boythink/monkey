use std::collections::HashMap;

use crate::{
    Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, Lexer, Program,
    ReturnStatement, Statement, Token, TokenType,
};

type PrefixParseFn = fn(&mut Parser) -> Box<dyn Expression>;
type InfixParseFn = fn(&mut Parser, dyn Expression) -> Box<dyn Expression>;

#[derive(PartialOrd, PartialEq)]
pub enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}

pub struct Parser {
    pub lexer: *mut Lexer,
    pub cur_token: Option<Token>,
    pub peek_token: Option<Token>,
    errors: Vec<String>,
    pub prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    pub infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
    pub fn new(l: &mut Lexer) -> Parser {
        let mut p = Parser {
            lexer: l,
            cur_token: None,
            peek_token: None,
            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        p.register_prefix(TokenType::IDENT, Parser::parse_identifier);
        p.register_prefix(TokenType::INT, Parser::parse_integer_literal);
        p.next_token();
        p.next_token();
        p
    }

    fn parse_identifier(&mut self) -> Box<dyn Expression> {
        Box::new(Identifier {
            token: self.cur_token.clone().unwrap(),
            value: self.cur_token.clone().unwrap().literal,
        })
    }

    fn register_prefix(&mut self, token_type: TokenType, f: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, f);
    }

    fn register_infix(&mut self, token_type: TokenType, f: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, f);
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        unsafe {
            self.peek_token = Some((*self.lexer).next_token());
        }
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program {
            statements: Vec::new(),
        };

        if self.cur_token.is_none() {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        while let Some(cur_token) = self.cur_token.clone() {
            if cur_token.r#type != TokenType::EOF {
                if let Some(stmt) = self.parse_statement() {
                    program.statements.push(stmt);
                }
                self.next_token();
            } else {
                break;
            }
        }

        Some(program)
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.as_ref().map(|t| t.r#type) {
            Some(TokenType::LET) => self
                .parse_let_statement()
                .map(|stmt| Box::new(stmt) as Box<dyn Statement>),

            Some(TokenType::RETURN) => self
                .parse_return_statement()
                .map(|stmt| Box::new(stmt) as Box<dyn Statement>),
            _ => self
                .parser_expression_statement()
                .map(|stmt| Box::new(stmt) as Box<dyn Statement>),
        }
    }

    fn parser_expression_statement(&mut self) -> Option<ExpressionStatement> {
        let stmt = ExpressionStatement {
            token: self.cur_token.clone().unwrap(),
            expression: self.parse_expression(Precedence::Lowest),
        };
        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token()
        }
        Some(stmt)
    }

    fn parse_expression(&mut self, _precedence: Precedence) -> Option<Box<dyn Expression>> {
        let prefix = self
            .prefix_parse_fns
            .get(&self.cur_token.clone().unwrap().r#type);
        match prefix {
            Some(prefix) => {
                let m = prefix(self);
                Some(m)
            }
            None => None,
        }
    }

    fn parse_integer_literal(&mut self) -> Box<dyn Expression> {
        let mut lit = IntegerLiteral {
            token: self.cur_token.clone().unwrap(),
            value: 0,
        };

        match i64::from_str_radix(&self.cur_token.clone().unwrap().literal, 10) {
            Ok(value) => {
                lit.value = value;
                Box::new(lit.clone())
            }
            Err(_) => {
                eprintln!(
                    "could not parse {} as integer",
                    self.cur_token.clone().unwrap().literal
                );
                Box::new(lit.clone())
            }
        }
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        let stmt = ReturnStatement {
            token: self.cur_token.clone().unwrap(),
            return_value: None,
        };
        self.next_token();

        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(stmt)
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let mut stmt = LetStatement {
            token: self.cur_token.clone().unwrap(),
            name: None,
            value: None,
        };

        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }
        let token = self.cur_token.clone().unwrap();
        stmt.name = Some(Identifier {
            token: token.clone(),
            value: token.literal,
        });

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        // TODO:value赋值
        // self.next_token();
        // stmt.value =

        // TODO: 跳过对表达式的处理，直到遇见分号
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(stmt)
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        match self.cur_token.as_ref().map(|tok| tok.r#type) {
            Some(token_type) => token_type == t,
            None => false,
        }
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        match self.peek_token.as_ref().map(|tok| tok.r#type) {
            Some(token_type) => token_type == t,
            None => false,
        }
    }

    fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            t,
            self.peek_token.clone().unwrap().r#type
        );
        self.errors.push(msg);
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }
}
