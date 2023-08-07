use lazy_static::lazy_static;
use std::collections::HashMap;
// use derive_more::{Add, Sub, From};
use crate::{
    Expression, ExpressionStatement, Identifier, InfixExpression, IntegerLiteral, LetStatement,
    Lexer, PrefixExpression, Program, ReturnStatement, Statement, Token, TokenType,
};

type InfixParseFn = fn(&mut Parser, Box<dyn Expression>) -> Box<dyn Expression>;
type PrefixParseFn = fn(&mut Parser) -> Box<dyn Expression>;

#[derive(PartialOrd, PartialEq, Clone, Copy)]
pub enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}

lazy_static! {
    static ref PRECEDENCES: HashMap<TokenType, Precedence> = {
        let mut m = HashMap::new();
        m.insert(TokenType::EQ, Precedence::Equals);
        m.insert(TokenType::NotEq, Precedence::Equals);
        m.insert(TokenType::LT, Precedence::LessGreater);
        m.insert(TokenType::GT, Precedence::LessGreater);
        m.insert(TokenType::PLUS, Precedence::Sum);
        m.insert(TokenType::MINUS, Precedence::Sum);
        m.insert(TokenType::SLASH, Precedence::Product);
        m.insert(TokenType::ASTERISK, Precedence::Product);
        m.insert(TokenType::LPAREN, Precedence::Call);
        m
    };
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
        p.register_prefix(TokenType::BANG, Parser::parse_prefix_expression);
        p.register_prefix(TokenType::MINUS, Parser::parse_prefix_expression);
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

    fn no_prefix_parse_fn_error(&mut self, t: TokenType) {
        let msg = format!("no prefix parse function for {:?} found", t);
        self.errors.push(msg);
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
            None => {
                self.no_prefix_parse_fn_error(self.cur_token.clone().unwrap().r#type);
                None
            }
        }
    }

    fn parse_prefix_expression(&mut self) -> Box<dyn Expression> {
        let mut expression = PrefixExpression {
            token: self.cur_token.clone().unwrap().clone(),
            operator: self.cur_token.clone().unwrap().literal.clone(),
            right: None,
        };

        self.next_token();

        expression.right = self.parse_expression(Precedence::Prefix);

        Box::new(expression)
    }

    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Box<dyn Expression> {
        let mut expression = InfixExpression {
            token: self.cur_token.clone().unwrap(),
            operator: self.cur_token.clone().unwrap().literal.clone(),
            left: Some(left),
            right: None,
        };

        let precedence = self.cur_precedence();
        self.next_token();

        if expression.operator == "+" {
            let p = (precedence as i8) - 1;
            expression.right = self.parse_expression(unsafe { std::mem::transmute(p) });
        } else {
            expression.right = self.parse_expression(precedence);
        }

        Box::new(expression)
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

    fn peek_precedence(&self) -> Precedence {
        *PRECEDENCES
            .get(&self.peek_token.clone().unwrap().r#type)
            .unwrap_or(&Precedence::Lowest)
    }

    fn cur_precedence(&self) -> Precedence {
        *PRECEDENCES
            .get(&self.cur_token.clone().unwrap().r#type)
            .unwrap_or(&Precedence::Lowest)
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
