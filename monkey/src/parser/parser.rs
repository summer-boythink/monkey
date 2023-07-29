use crate::{Identifier, LetStatement, Lexer, Program, Statement, Token, TokenType};

pub struct Parser {
    pub lexer: *mut Lexer,
    pub cur_token: Option<Token>,
    pub peek_token: Option<Token>,
}

impl Parser {
    pub fn new(l: &mut Lexer) -> Parser {
        let mut p = Parser {
            lexer: l,
            cur_token: None,
            peek_token: None,
        };
        p.next_token();
        p.next_token();
        return p;
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
            _ => None,
        }
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

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }
}
