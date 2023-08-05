#[cfg(test)]
mod tests {

    use monkey::{
        parser::parser::Parser, Expression, ExpressionStatement, Identifier, IntegerLiteral,
        LetStatement, Lexer, Node, PrefixExpression, ReturnStatement, Statement,
    };

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

    #[test]
    fn test_indent_expression() {
        let input = "foobar;".to_string();

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        check_parser_error(p);

        if let Some(prog) = program {
            assert_eq!(
                prog.statements.len(),
                1,
                "prog.Statements does not contain 1 statements. got={}",
                prog.statements.len()
            );
            for stmt in &prog.statements {
                let ident_stmt = stmt
                    .as_any()
                    .downcast_ref::<ExpressionStatement>()
                    .expect("stmt not ExpressionStatement");

                let ident = ident_stmt
                    .expression
                    .as_ref()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<Identifier>()
                    .expect("ident_stmt not Identifier");

                assert_eq!(ident.token_literal(), "foobar");
                assert_eq!(ident.value, "foobar");
            }
        }
    }

    #[test]
    fn test_integet_expression() {
        let input = "5;".to_string();

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let program = p.parse_program();
        check_parser_error(p);

        if let Some(prog) = program {
            assert_eq!(
                prog.statements.len(),
                1,
                "prog.Statements does not contain 1 statements. got={}",
                prog.statements.len()
            );
            for stmt in &prog.statements {
                let int_stmt = stmt
                    .as_any()
                    .downcast_ref::<ExpressionStatement>()
                    .unwrap()
                    .expression
                    .as_ref()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<IntegerLiteral>()
                    .expect("int_stmt not IntegerLiteral");

                assert_eq!(int_stmt.token_literal(), "5");
                assert_eq!(int_stmt.value, 5);
            }
        }
    }

    #[test]
    fn test_prefix_expressions() {
        let prefix_tests = vec![("!5;", "!", 5), ("-15;", "-", 15)];

        for tt in prefix_tests {
            let mut l = Lexer::new(tt.0.to_string());
            let mut p = Parser::new(&mut l);

            let program = p.parse_program();
            check_parser_error(p);

            if let Some(prog) = program {
                assert_eq!(
                    prog.statements.len(),
                    1,
                    "prog.Statements does not contain 1 statements. got={}",
                    prog.statements.len()
                );
                for stmt in &prog.statements {
                    let prefix_stmt = stmt
                        .as_any()
                        .downcast_ref::<ExpressionStatement>()
                        .unwrap()
                        .expression
                        .as_ref()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<PrefixExpression>()
                        .expect("prefix_stmt not PrefixExpression");

                    assert_eq!(prefix_stmt.operator, tt.1);
                    test_indent_literal(
                        prefix_stmt.right.as_ref().map(|x| x.as_ref()).unwrap(),
                        tt.2,
                    );
                }
            }
        }
    }

    fn test_indent_literal(il: &dyn Expression, value: i64) {
        let integ = il
            .as_any()
            .downcast_ref::<IntegerLiteral>()
            .expect("il not *ast.IntegerLiteral");

        assert_eq!(
            integ.value, value,
            "integ.Value not {}. got={}",
            value, integ.value
        );
        assert_eq!(
            integ.token_literal(),
            value.to_string(),
            "integ.TokenLiteral not {}. got={}",
            value,
            integ.token_literal()
        );
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
