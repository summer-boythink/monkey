#[cfg(test)]
mod tests {
    use monkey::{parser::parser::Parser, Lexer, Statement, LetStatement};

    #[test]
    fn test_let_statements() {
        let input = "let x = 5;\
        let y = 10;\
        let foo = 88;"
            .to_string();

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        if let Some(prog) = program {
            let statelen = prog.statements.len();
            if statelen != 3 {
                eprintln!("hope 3 statements ,get {}", statelen)
            }

            let tests = vec!["x", "y", "foo"];
            for (key, test) in tests.iter().enumerate() {
                let stmt = &prog.statements[key];
                assert!(test_let_statement(stmt, test.to_string()));
            }
        } else {
            eprintln!("parse_program None");
        }
    }

    fn test_let_statement<S>(s:&Box<S>, name: String) -> bool
        where
        S:Statement+ ?Sized
     {
        if s.token_literal() != "let" {
            eprintln!("s.TokenLiteral not 'let'. got={}", s.token_literal());
            return false;
        }
    
        let let_stmt = match s.as_any().downcast_ref::<LetStatement>() {
            Some(stmt) => stmt,
            None => {
                eprintln!("s not LetStatement. got={}", std::any::type_name::<S>());
                return false;
            }
        };

        unsafe {
            if (*let_stmt.name).value != name {
                eprintln!(
                    "letStmt.Name.Value not '{}'. got={}",
                    name, (*let_stmt.name).value
                );
                return false;
            }
        
            if (*let_stmt.name).token_literal() != name {
                eprintln!(
                    "letStmt.Name.TokenLiteral() not '{}'. got={}",
                    name,
                    (*let_stmt.name).token_literal()
                );
                return false;
            }
        
            true
        }
    }
}
