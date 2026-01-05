#[cfg(test)]
mod tests {
    use rcompiler::parser::*;
    use rcompiler::tokenizer::*;
    // --- Helpers ---

    fn tok(kind: TokenKind, lit: &'static str) -> Token<'static> {
         Token {
            kind,
            span: TokenSpan { start: 0, end: 0, literal: lit, line: 0, column: 0 },
        }
    }

    fn parse_test(tokens: Vec<Token<'static>>) -> Result<Vec<Statement<'static>>, ParserError> {
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    // --- Precedence & Math Tests ---

    #[test]
    fn test_precedence_arithmetic() {
        // i32 x = 1 + 2 * 3;
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::IntegerLiteral, "1"),
            tok(TokenKind::Plus, "+"),
            tok(TokenKind::IntegerLiteral, "2"),
            tok(TokenKind::Multiplication, "*"),
            tok(TokenKind::IntegerLiteral, "3"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];
        let res = parse_test(tokens);
        assert!(res.is_ok(), "Failed to parse 1 + 2 * 3: {:?}", res.err());
    }

    #[test]
    fn test_unary_precedence() {
        // i32 x = -5 + 3;
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::Minus, "-"),
            tok(TokenKind::IntegerLiteral, "5"),
            tok(TokenKind::Plus, "+"),
            tok(TokenKind::IntegerLiteral, "3"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    // --- Array & Function Tests (The Pratt Special) ---

    #[test]
    fn test_nested_array_access() {
        // i32 x = matrix[0][1];
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::Identifier, "matrix"),
            tok(TokenKind::LeftBracket, "["),
            tok(TokenKind::IntegerLiteral, "0"),
            tok(TokenKind::RightBracket, "]"),
            tok(TokenKind::LeftBracket, "["),
            tok(TokenKind::IntegerLiteral, "1"),
            tok(TokenKind::RightBracket, "]"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_function_call_with_expressions() {
        // func(1 + 2, x);
        let tokens = vec![
            tok(TokenKind::Identifier, "func"),
            tok(TokenKind::LeftParen, "("),
            tok(TokenKind::IntegerLiteral, "1"),
            tok(TokenKind::Plus, "+"),
            tok(TokenKind::IntegerLiteral, "2"),
            tok(TokenKind::Comma, ","),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::RightParen, ")"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }
    // --- Control Flow ---

    #[test]
    fn test_while_loop_complex_condition() {
        // while x < 10 + 2 { x = x + 1; }
        let tokens = vec![
            tok(TokenKind::While, "while"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Multiplication, "*"), // Using * as a stand-in for < if you haven't added < yet
            tok(TokenKind::IntegerLiteral, "10"),
            tok(TokenKind::LeftBrace, "{"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Plus, "+"),
            tok(TokenKind::IntegerLiteral, "1"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::RightBrace, "}"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_if_else_chain() {
        // if x { return 1; } else { return 0; }
        let tokens = vec![
            tok(TokenKind::If, "if"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::LeftBrace, "{"),
            tok(TokenKind::Return, "return"),
            tok(TokenKind::IntegerLiteral, "1"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::RightBrace, "}"),
            tok(TokenKind::Else, "else"),
            tok(TokenKind::LeftBrace, "{"),
            tok(TokenKind::Return, "return"),
            tok(TokenKind::IntegerLiteral, "0"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::RightBrace, "}"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_function_definition() {
        // fn add(i32 a, i32 b) i32 { return a + b; }
        let tokens = vec![
            tok(TokenKind::Function, "fn"),
            tok(TokenKind::Identifier, "add"),
            tok(TokenKind::LeftParen, "("),
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "a"),
            tok(TokenKind::Comma, ","),
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "b"),
            tok(TokenKind::RightParen, ")"),
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::LeftBrace, "{"),
            tok(TokenKind::Return, "return"),
            tok(TokenKind::Identifier, "a"),
            tok(TokenKind::Plus, "+"),
            tok(TokenKind::Identifier, "b"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::RightBrace, "}"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_unary_arithmetic_mix() {
        // i32 x = -5 * -3;
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::Minus, "-"),
            tok(TokenKind::IntegerLiteral, "5"),
            tok(TokenKind::Multiplication, "*"),
            tok(TokenKind::Minus, "-"),
            tok(TokenKind::IntegerLiteral, "3"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok(), "Should handle multiple unaries in one expr");
    }

    #[test]
    fn test_array_index_is_expression() {
        // u16 x = my_array[1 + offset];
        let tokens = vec![
            tok(TokenKind::UnsignedInt16, "u16"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::Identifier, "my_array"),
            tok(TokenKind::LeftBracket, "["),
            tok(TokenKind::IntegerLiteral, "1"),
            tok(TokenKind::Plus, "+"),
            tok(TokenKind::Identifier, "offset"),
            tok(TokenKind::RightBracket, "]"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_function_call_as_binary_operand() {
        // i32 x = 10 + calculate(a, b);
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::IntegerLiteral, "10"),
            tok(TokenKind::Plus, "+"),
            tok(TokenKind::Identifier, "calculate"),
            tok(TokenKind::LeftParen, "("),
            tok(TokenKind::Identifier, "a"),
            tok(TokenKind::Comma, ","),
            tok(TokenKind::Identifier, "b"),
            tok(TokenKind::RightParen, ")"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_nested_while_condition() {
        // while (status == true) { ... }
        // Testing parens inside while condition
        let tokens = vec![
            tok(TokenKind::While, "while"),
            tok(TokenKind::LeftParen, "("),
            tok(TokenKind::Identifier, "status"),
            tok(TokenKind::Plus, "+"), // Using + as stand-in for == for now
            tok(TokenKind::True, "true"),
            tok(TokenKind::RightParen, ")"),
            tok(TokenKind::LeftBrace, "{"),
            tok(TokenKind::RightBrace, "}"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_array_declaration_with_size() {
        // u16[5] my_array;
        let tokens = vec![
            tok(TokenKind::UnsignedInt16, "u16"),
            tok(TokenKind::LeftBracket, "["),
            tok(TokenKind::IntegerLiteral, "5"),
            tok(TokenKind::RightBracket, "]"),
            tok(TokenKind::Identifier, "my_array"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

        #[test]
    fn test_literal_expressions() {
        let cases = vec![
            tok(TokenKind::IntegerLiteral, "0"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];

        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),
            cases[0].clone(),
            cases[1].clone(),
            cases[2].clone(),
        ];

        assert!(parse_test(tokens).is_ok());
    }

    // --- Unary Stress ---

    #[test]
    fn test_multiple_unary_chain() {
        // i32 x = ---5;
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::Minus, "-"),
            tok(TokenKind::Minus, "-"),
            tok(TokenKind::Minus, "-"),
            tok(TokenKind::IntegerLiteral, "5"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];

        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_unary_grouping() {
        // i32 x = -(1 + 2);
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::Minus, "-"),
            tok(TokenKind::LeftParen, "("),
            tok(TokenKind::IntegerLiteral, "1"),
            tok(TokenKind::Plus, "+"),
            tok(TokenKind::IntegerLiteral, "2"),
            tok(TokenKind::RightParen, ")"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];

        assert!(parse_test(tokens).is_ok());
    }

    // --- Precedence & Associativity ---

    #[test]
    fn test_left_associativity_chain() {
        // i32 x = 10 - 5 - 2;
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::IntegerLiteral, "10"),
            tok(TokenKind::Minus, "-"),
            tok(TokenKind::IntegerLiteral, "5"),
            tok(TokenKind::Minus, "-"),
            tok(TokenKind::IntegerLiteral, "2"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];

        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_parentheses_override_precedence() {
        // i32 x = (1 + 2) * 3;
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::LeftParen, "("),
            tok(TokenKind::IntegerLiteral, "1"),
            tok(TokenKind::Plus, "+"),
            tok(TokenKind::IntegerLiteral, "2"),
            tok(TokenKind::RightParen, ")"),
            tok(TokenKind::Multiplication, "*"),
            tok(TokenKind::IntegerLiteral, "3"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];

        assert!(parse_test(tokens).is_ok());
    }

    // --- Function Calls ---

    #[test]
    fn test_function_call_no_args() {
        // i32 x = foo();
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::Identifier, "foo"),
            tok(TokenKind::LeftParen, "("),
            tok(TokenKind::RightParen, ")"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];

        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_nested_function_calls() {
        // i32 x = foo(bar(1), baz(2 + 3));
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::Identifier, "foo"),
            tok(TokenKind::LeftParen, "("),

            tok(TokenKind::Identifier, "bar"),
            tok(TokenKind::LeftParen, "("),
            tok(TokenKind::IntegerLiteral, "1"),
            tok(TokenKind::RightParen, ")"),
            tok(TokenKind::Comma, ","),

            tok(TokenKind::Identifier, "baz"),
            tok(TokenKind::LeftParen, "("),
            tok(TokenKind::IntegerLiteral, "2"),
            tok(TokenKind::Plus, "+"),
            tok(TokenKind::IntegerLiteral, "3"),
            tok(TokenKind::RightParen, ")"),

            tok(TokenKind::RightParen, ")"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];

        assert!(parse_test(tokens).is_ok());
    }

    // --- Array Access ---

    #[test]
    fn test_array_index_expression() {
        // i32 x = arr[1 + 2];
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::Identifier, "arr"),
            tok(TokenKind::LeftBracket, "["),
            tok(TokenKind::IntegerLiteral, "1"),
            tok(TokenKind::Plus, "+"),
            tok(TokenKind::IntegerLiteral, "2"),
            tok(TokenKind::RightBracket, "]"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];

        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_multi_dimensional_array_access() {
        // i32 x = matrix[0][1];
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::Identifier, "matrix"),
            tok(TokenKind::LeftBracket, "["),
            tok(TokenKind::IntegerLiteral, "0"),
            tok(TokenKind::RightBracket, "]"),
            tok(TokenKind::LeftBracket, "["),
            tok(TokenKind::IntegerLiteral, "1"),
            tok(TokenKind::RightBracket, "]"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];

        assert!(parse_test(tokens).is_ok());
    }

    // --- Mixed High-Complexity Expressions ---

    #[test]
    fn test_expression_everything_combined() {
        // i32 x = foo(a + b * c)[bar(2)] - -baz();
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),

            tok(TokenKind::Identifier, "foo"),
            tok(TokenKind::LeftParen, "("),
            tok(TokenKind::Identifier, "a"),
            tok(TokenKind::Plus, "+"),
            tok(TokenKind::Identifier, "b"),
            tok(TokenKind::Multiplication, "*"),
            tok(TokenKind::Identifier, "c"),
            tok(TokenKind::RightParen, ")"),

            tok(TokenKind::LeftBracket, "["),
            tok(TokenKind::Identifier, "bar"),
            tok(TokenKind::LeftParen, "("),
            tok(TokenKind::IntegerLiteral, "2"),
            tok(TokenKind::RightParen, ")"),
            tok(TokenKind::RightBracket, "]"),

            tok(TokenKind::Minus, "-"),
            tok(TokenKind::Minus, "-"),
            tok(TokenKind::Identifier, "baz"),
            tok(TokenKind::LeftParen, "("),
            tok(TokenKind::RightParen, ")"),

            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];

        assert!(
            parse_test(tokens).is_ok(),
            "Failed complex mixed expression"
        );
    }

    // --- Expected Failures (Expression Errors) ---

    #[test]
    fn test_trailing_comma_in_call_should_fail() {
        // foo(1,)
        let tokens = vec![
            tok(TokenKind::Identifier, "foo"),
            tok(TokenKind::LeftParen, "("),
            tok(TokenKind::IntegerLiteral, "1"),
            tok(TokenKind::Comma, ","),
            tok(TokenKind::RightParen, ")"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];

        assert!(parse_test(tokens).is_err());
    }

    #[test]
    fn test_unclosed_parenthesis_should_fail() {
        // i32 x = (1 + 2;
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::LeftParen, "("),
            tok(TokenKind::IntegerLiteral, "1"),
            tok(TokenKind::Plus, "+"),
            tok(TokenKind::IntegerLiteral, "2"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];

        assert!(parse_test(tokens).is_err());
    }
}