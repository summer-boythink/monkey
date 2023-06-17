use crate::{ast, Lexer, Token};

pub struct Parser {
    lexer: *mut Lexer,
    cur_token: Option<*mut Token>,
    peek_token: Option<*mut Token>,
}

impl Parser {
    pub fn new(l: &mut Lexer) -> Parser {
        Parser {
            lexer: l,
            cur_token: None,
            peek_token: None,
        }
    }

    pub fn new_token(&mut self) {
        self.cur_token = self.peek_token;
        unsafe {
            self.peek_token = Some(&mut ((*self.lexer).next_token()));
        }
    }

    pub fn parse_program(&mut self) -> Option<ast::Program> {
        todo!()
    }
}
