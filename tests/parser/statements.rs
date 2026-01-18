use rcompiler::parser::*;
use rcompiler::ast::*;
use rcompiler::tokenizer::*;
use rcompiler::error::*;
use rcompiler::interner::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to generate tokens while maintaining the interner state.
    fn tok(interner: &mut Interner, kind: TokenKind, lit: &str) -> Token {
        Token {
            kind,
            span: TokenSpan { 
                start: 0, 
                end: 0, 
                literal: interner.intern(lit), 
                line: 0, 
                column: 0 
            },
        }
    }

    fn parse_test(tokens: Vec<Token>) -> Result<Vec<Statement>, ParserError> {
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_basic_declarations() {
        let mut i = Interner::new();
        // i32 my_number;
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "my_number"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_const_declaration() {
        let mut i = Interner::new();
        // const i32 my_const;
        let tokens = vec![
            tok(&mut i, TokenKind::Const, "const"),
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "my_const"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_declaration_with_assignment() {
        let mut i = Interner::new();
        // bool is_running = true;
        let tokens = vec![
            tok(&mut i, TokenKind::Boolean, "bool"),
            tok(&mut i, TokenKind::Identifier, "is_running"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::BooleanLiteral, "true"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_array_declaration() {
        let mut i = Interner::new();
        // u16[5] my_array;
        let tokens = vec![
            tok(&mut i, TokenKind::UnsignedInt16, "u16"),
            tok(&mut i, TokenKind::LeftBracket, "["),
            tok(&mut i, TokenKind::IntegerLiteral, "5"),
            tok(&mut i, TokenKind::RightBracket, "]"),
            tok(&mut i, TokenKind::Identifier, "my_array"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_if_statement() {
        let mut i = Interner::new();
        // if x { y = 10; }
        let tokens = vec![
            tok(&mut i, TokenKind::If, "if"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::LeftBrace, "{"),
            tok(&mut i, TokenKind::Identifier, "y"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::IntegerLiteral, "10"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::RightBrace, "}"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_while_loop() {
        let mut i = Interner::new();
        // while true { break; }
        let tokens = vec![
            tok(&mut i, TokenKind::While, "while"),
            tok(&mut i, TokenKind::BooleanLiteral, "true"),
            tok(&mut i, TokenKind::LeftBrace, "{"),
            tok(&mut i, TokenKind::Break, "break"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::RightBrace, "}"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_error_missing_semicolon() {
        let mut i = Interner::new();
        // i32 x
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        let res = parse_test(tokens);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().code, ErrorCode::EP006);
    }

    #[test]
    fn test_error_malformed_if() {
        let mut i = Interner::new();
        // if { }
        let tokens = vec![
            tok(&mut i, TokenKind::If, "if"),
            tok(&mut i, TokenKind::LeftBrace, "{"),
            tok(&mut i, TokenKind::RightBrace, "}"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        let res = parse_test(tokens);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().code, ErrorCode::EP007);
    }

    #[test]
    fn test_error_invalid_array_size() {
        let mut i = Interner::new();
        // i32[] x;
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::LeftBracket, "["),
            tok(&mut i, TokenKind::RightBracket, "]"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        let res = parse_test(tokens);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().code, ErrorCode::EP001);
    }

    #[test]
    fn test_full_if_elseif_else() {
        let mut i = Interner::new();
        // if x { } elif y { } else { }
        let tokens = vec![
            tok(&mut i, TokenKind::If, "if"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::LeftBrace, "{"),
            tok(&mut i, TokenKind::RightBrace, "}"),
            tok(&mut i, TokenKind::ElseIf, "elif"),
            tok(&mut i, TokenKind::Identifier, "y"),
            tok(&mut i, TokenKind::LeftBrace, "{"),
            tok(&mut i, TokenKind::RightBrace, "}"),
            tok(&mut i, TokenKind::Else, "else"),
            tok(&mut i, TokenKind::LeftBrace, "{"),
            tok(&mut i, TokenKind::RightBrace, "}"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_variable_assignment_operators() {
        let mut i = Interner::new();
        // x += 10;
        let tokens = vec![
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::AddAssignment, "+="),
            tok(&mut i, TokenKind::IntegerLiteral, "10"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_array_with_expression_size() {
        let mut i = Interner::new();
        // i32[5 + 5] arr;
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::LeftBracket, "["),
            tok(&mut i, TokenKind::IntegerLiteral, "5"),
            tok(&mut i, TokenKind::Plus, "+"),
            tok(&mut i, TokenKind::IntegerLiteral, "5"),
            tok(&mut i, TokenKind::RightBracket, "]"),
            tok(&mut i, TokenKind::Identifier, "arr"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_empty_while_body() {
        let mut i = Interner::new();
        let tokens = vec![
            tok(&mut i, TokenKind::While, "while"),
            tok(&mut i, TokenKind::BooleanLiteral, "true"),
            tok(&mut i, TokenKind::LeftBrace, "{"),
            tok(&mut i, TokenKind::RightBrace, "}"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_return_void() {
        let mut i = Interner::new();
        // return;
        let tokens = vec![
            tok(&mut i, TokenKind::Return, "return"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_return_value() {
        let mut i = Interner::new();
        // return x;
        let tokens = vec![
            tok(&mut i, TokenKind::Return, "return"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }
}