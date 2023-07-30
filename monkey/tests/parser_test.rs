#[cfg(test)]
mod tests {

    use monkey::{parser::parser::Parser, LetStatement, Lexer, Node, ReturnStatement, Statement};

    #[test]
    fn test_let_statements() {
        let input = "let x = 5;\
        let y = 10;\
        let foo = 88;"
            .to_string();

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        check_parser_error(p);

        if let Some(prog) = program {
            let statelen = prog.statements.len();
            if statelen != 3 {
                panic!("hope 3 statements ,get {}", statelen)
            }

            let tests = vec!["x", "y", "foo"];
            for (key, test) in tests.iter().enumerate() {
                let stmt = &prog.statements[key];
                assert!(test_let_statement(stmt, test.to_string()));
            }
        } else {
            panic!("parse_program None");
        }
    }

    #[test]
    fn test_return_statements() {
        let input = "return 5;\
        return 10;\
        return 993322;"
            .to_string();

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        check_parser_error(p);

        if let Some(prog) = program {
            assert_eq!(
                prog.statements.len(),
                3,
                "prog.Statements does not contain 3 statements. got={}",
                prog.statements.len()
            );

            for stmt in &prog.statements {
                let return_stmt = stmt
                    .as_any()
                    .downcast_ref::<ReturnStatement>()
                    .expect("stmt not ReturnStatement");
                assert_eq!(
                    return_stmt.token_literal(),
                    "return",
                    "returnStmt.TokenLiteral not 'return', got {:?}",
                    return_stmt.token_literal()
                );
            }
        }
    }

    fn check_parser_error(p: Parser) {
        let errors = p.errors();

        if errors.is_empty() {
            return;
        }
        for msg in errors {
            panic!("parser error: {:?}", msg);
        }
        panic!("parser has {} errors", errors.len());
    }

    fn test_let_statement<S>(s: &Box<S>, name: String) -> bool
    where
        S: Statement + ?Sized,
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

        if let Some(ident) = let_stmt.name.as_ref().map(|ident| ident) {
            if ident.value != name {
                eprintln!("letStmt.Name.Value not '{}'. got={}", name, ident.value);
                return false;
            }

            if ident.token_literal() != name {
                eprintln!(
                    "letStmt.Name.TokenLiteral() not '{}'. got={}",
                    name,
                    ident.token_literal()
                );
                return false;
            }
        }

        true
    }
}
