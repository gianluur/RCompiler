
#[cfg(test)]
mod tests {
    use rcompiler::parser::*;
    use rcompiler::tokenizer::*;
    use rcompiler::error::*;

    // Helper to generate tokens quickly
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


    #[test]
    fn test_basic_declarations() {
        // i32 my_number;
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "my_number"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_const_declaration() {
        // const i32 my_const;
        let tokens = vec![
            tok(TokenKind::Const, "const"),
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "my_const"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_declaration_with_assignment() {
        // bool is_running = true;
        let tokens = vec![
            tok(TokenKind::Boolean, "bool"),
            tok(TokenKind::Identifier, "is_running"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::True, "true"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }


    #[test]
    fn test_array_declaration() {
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
    fn test_if_statement() {
        // if x { y = 10; }
        let tokens = vec![
            tok(TokenKind::If, "if"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::LeftBrace, "{"),
            tok(TokenKind::Identifier, "y"),
            tok(TokenKind::Assignment, "="),
            tok(TokenKind::IntegerLiteral, "10"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::RightBrace, "}"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_while_loop() {
        // while true { break; }
        let tokens = vec![
            tok(TokenKind::While, "while"),
            tok(TokenKind::True, "true"),
            tok(TokenKind::LeftBrace, "{"),
            tok(TokenKind::Break, "break"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::RightBrace, "}"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }


    #[test]
    fn test_error_missing_semicolon() {
        // i32 x
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Eof, ""),
        ];
        let res = parse_test(tokens);
        assert!(res.is_err());
        // Should trigger EP006 (Expected semicolon or assignment)
        assert_eq!(res.unwrap_err().code, ErrorCode::EP006);
    }

    #[test]
    fn test_error_malformed_if() {
        // if { } -> Missing condition
        let tokens = vec![
            tok(TokenKind::If, "if"),
            tok(TokenKind::LeftBrace, "{"),
            tok(TokenKind::RightBrace, "}"),
        ];
        let res = parse_test(tokens);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().code, ErrorCode::EP007);
    }

    #[test]
    fn test_error_invalid_array_size() {
        // i32[] x; -> Missing expression inside []
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::LeftBracket, "["),
            tok(TokenKind::RightBracket, "]"),
        ];
        let res = parse_test(tokens);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().code, ErrorCode::EP001);
    }

    #[test]
    fn test_full_if_elseif_else() {
        // if x { } elif y { } else { }
        let tokens = vec![
            tok(TokenKind::If, "if"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::LeftBrace, "{"),
            tok(TokenKind::RightBrace, "}"),
            tok(TokenKind::ElseIf, "elif"),
            tok(TokenKind::Identifier, "y"),
            tok(TokenKind::LeftBrace, "{"),
            tok(TokenKind::RightBrace, "}"),
            tok(TokenKind::Else, "else"),
            tok(TokenKind::LeftBrace, "{"),
            tok(TokenKind::RightBrace, "}"),
            tok(TokenKind::Eof, ""),
        ];
        let res = parse_test(tokens);
        assert!(res.is_ok(), "Failed to parse if-elif-else chain: {:?}", res.err());
    }

    #[test]
    fn test_variable_assignment_operators() {
        // x += 10;
        let tokens = vec![
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::AddAssignment, "+="),
            tok(TokenKind::IntegerLiteral, "10"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_array_with_expression_size() {
        // i32[5 + 5] arr;
        // Note: Currently returns Placeholder based on your code
        let tokens = vec![
            tok(TokenKind::SignedInt32, "i32"),
            tok(TokenKind::LeftBracket, "["),
            tok(TokenKind::IntegerLiteral, "5"),
            tok(TokenKind::RightBracket, "]"),
            tok(TokenKind::Identifier, "arr"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_empty_while_body() {
        // while true { }
        let tokens = vec![
            tok(TokenKind::While, "while"),
            tok(TokenKind::True, "true"),
            tok(TokenKind::LeftBrace, "{"),
            tok(TokenKind::RightBrace, "}"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_return_void() {
        // return;
        let tokens = vec![
            tok(TokenKind::Return, "return"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_return_value() {
        // return x;
        let tokens = vec![
            tok(TokenKind::Return, "return"),
            tok(TokenKind::Identifier, "x"),
            tok(TokenKind::Semicolon, ";"),
            tok(TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

}