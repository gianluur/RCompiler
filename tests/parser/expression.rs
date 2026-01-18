use rcompiler::parser::*;
use rcompiler::ast::*;
use rcompiler::tokenizer::*;
use rcompiler::interner::*;

#[cfg(test)]
mod tests {
    use super::*;

    // --- Helpers ---

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

    // --- Control Flow ---

    #[test]
    fn test_if_else_chain() {
        let mut i = Interner::new();
        // if x { return 1; } else { return 0; }
        let tokens = vec![
            tok(&mut i, TokenKind::If, "if"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::LeftBrace, "{"),
            tok(&mut i, TokenKind::Return, "return"),
            tok(&mut i, TokenKind::IntegerLiteral, "1"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::RightBrace, "}"),
            tok(&mut i, TokenKind::Else, "else"),
            tok(&mut i, TokenKind::LeftBrace, "{"),
            tok(&mut i, TokenKind::Return, "return"),
            tok(&mut i, TokenKind::IntegerLiteral, "0"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::RightBrace, "}"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_unary_arithmetic_mix() {
        let mut i = Interner::new();
        // i32 x = -5 * -3;
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::Minus, "-"),
            tok(&mut i, TokenKind::IntegerLiteral, "5"),
            tok(&mut i, TokenKind::Multiplication, "*"),
            tok(&mut i, TokenKind::Minus, "-"),
            tok(&mut i, TokenKind::IntegerLiteral, "3"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok(), "Should handle multiple unaries in one expr");
    }

    #[test]
    fn test_array_index_is_expression() {
        let mut i = Interner::new();
        // u16 x = my_array[1 + offset];
        let tokens = vec![
            tok(&mut i, TokenKind::UnsignedInt16, "u16"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::Identifier, "my_array"),
            tok(&mut i, TokenKind::LeftBracket, "["),
            tok(&mut i, TokenKind::IntegerLiteral, "1"),
            tok(&mut i, TokenKind::Plus, "+"),
            tok(&mut i, TokenKind::Identifier, "offset"),
            tok(&mut i, TokenKind::RightBracket, "]"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_function_call_as_binary_operand() {
        let mut i = Interner::new();
        // i32 x = 10 + calculate(a, b);
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::IntegerLiteral, "10"),
            tok(&mut i, TokenKind::Plus, "+"),
            tok(&mut i, TokenKind::Identifier, "calculate"),
            tok(&mut i, TokenKind::LeftParen, "("),
            tok(&mut i, TokenKind::Identifier, "a"),
            tok(&mut i, TokenKind::Comma, ","),
            tok(&mut i, TokenKind::Identifier, "b"),
            tok(&mut i, TokenKind::RightParen, ")"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_nested_while_condition() {
        let mut i = Interner::new();
        // while (status + true) { ... }
        let tokens = vec![
            tok(&mut i, TokenKind::While, "while"),
            tok(&mut i, TokenKind::LeftParen, "("),
            tok(&mut i, TokenKind::Identifier, "status"),
            tok(&mut i, TokenKind::Plus, "+"), 
            tok(&mut i, TokenKind::BooleanLiteral, "true"),
            tok(&mut i, TokenKind::RightParen, ")"),
            tok(&mut i, TokenKind::LeftBrace, "{"),
            tok(&mut i, TokenKind::RightBrace, "}"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_function_definition() {
        let mut i = Interner::new();
        // fn add(i32 a, i32 b) i32 { return a + b; }
        let tokens = vec![
            tok(&mut i, TokenKind::Function, "fn"),
            tok(&mut i, TokenKind::Identifier, "add"),
            tok(&mut i, TokenKind::LeftParen, "("),
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "a"),
            tok(&mut i, TokenKind::Comma, ","),
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "b"),
            tok(&mut i, TokenKind::RightParen, ")"),
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::LeftBrace, "{"),
            tok(&mut i, TokenKind::Return, "return"),
            tok(&mut i, TokenKind::Identifier, "a"),
            tok(&mut i, TokenKind::Plus, "+"),
            tok(&mut i, TokenKind::Identifier, "b"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::RightBrace, "}"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_function_call_with_expressions() {
        let mut i = Interner::new();
        // func(1 + 2, x);
        let tokens = vec![
            tok(&mut i, TokenKind::Identifier, "func"),
            tok(&mut i, TokenKind::LeftParen, "("),
            tok(&mut i, TokenKind::IntegerLiteral, "1"),
            tok(&mut i, TokenKind::Plus, "+"),
            tok(&mut i, TokenKind::IntegerLiteral, "2"),
            tok(&mut i, TokenKind::Comma, ","),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::RightParen, ")"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_nested_array_access() {
        let mut i = Interner::new();
        // i32 x = matrix[0][1];
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::Identifier, "matrix"),
            tok(&mut i, TokenKind::LeftBracket, "["),
            tok(&mut i, TokenKind::IntegerLiteral, "0"),
            tok(&mut i, TokenKind::RightBracket, "]"),
            tok(&mut i, TokenKind::LeftBracket, "["),
            tok(&mut i, TokenKind::IntegerLiteral, "1"),
            tok(&mut i, TokenKind::RightBracket, "]"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_precedence_arithmetic() {
        let mut i = Interner::new();
        // i32 x = 1 + 2 * 3;
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::IntegerLiteral, "1"),
            tok(&mut i, TokenKind::Plus, "+"),
            tok(&mut i, TokenKind::IntegerLiteral, "2"),
            tok(&mut i, TokenKind::Multiplication, "*"),
            tok(&mut i, TokenKind::IntegerLiteral, "3"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        let res = parse_test(tokens);
        assert!(res.is_ok(), "Failed to parse 1 + 2 * 3: {:?}", res.err());
    }

    #[test]
    fn test_unary_precedence() {
        let mut i = Interner::new();
        // i32 x = -5 + 3;
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::Minus, "-"),
            tok(&mut i, TokenKind::IntegerLiteral, "5"),
            tok(&mut i, TokenKind::Plus, "+"),
            tok(&mut i, TokenKind::IntegerLiteral, "3"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_while_loop_complex_condition() {
        let mut i = Interner::new();
        // while x * 10 { x = x + 1; }
        let tokens = vec![
            tok(&mut i, TokenKind::While, "while"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Multiplication, "*"), 
            tok(&mut i, TokenKind::IntegerLiteral, "10"),
            tok(&mut i, TokenKind::LeftBrace, "{"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Plus, "+"),
            tok(&mut i, TokenKind::IntegerLiteral, "1"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::RightBrace, "}"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_array_declaration_with_size() {
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
    fn test_literal_expressions() {
        let mut i = Interner::new();
        // i32 x = 0;
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::IntegerLiteral, "0"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_multiple_unary_chain() {
        let mut i = Interner::new();
        // i32 x = ---5;
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::Minus, "-"),
            tok(&mut i, TokenKind::Minus, "-"),
            tok(&mut i, TokenKind::Minus, "-"),
            tok(&mut i, TokenKind::IntegerLiteral, "5"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_unary_grouping() {
        let mut i = Interner::new();
        // i32 x = -(1 + 2);
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::Minus, "-"),
            tok(&mut i, TokenKind::LeftParen, "("),
            tok(&mut i, TokenKind::IntegerLiteral, "1"),
            tok(&mut i, TokenKind::Plus, "+"),
            tok(&mut i, TokenKind::IntegerLiteral, "2"),
            tok(&mut i, TokenKind::RightParen, ")"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_left_associativity_chain() {
        let mut i = Interner::new();
        // i32 x = 10 - 5 - 2;
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::IntegerLiteral, "10"),
            tok(&mut i, TokenKind::Minus, "-"),
            tok(&mut i, TokenKind::IntegerLiteral, "5"),
            tok(&mut i, TokenKind::Minus, "-"),
            tok(&mut i, TokenKind::IntegerLiteral, "2"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_parentheses_override_precedence() {
        let mut i = Interner::new();
        // i32 x = (1 + 2) * 3;
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::LeftParen, "("),
            tok(&mut i, TokenKind::IntegerLiteral, "1"),
            tok(&mut i, TokenKind::Plus, "+"),
            tok(&mut i, TokenKind::IntegerLiteral, "2"),
            tok(&mut i, TokenKind::RightParen, ")"),
            tok(&mut i, TokenKind::Multiplication, "*"),
            tok(&mut i, TokenKind::IntegerLiteral, "3"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_function_call_no_args() {
        let mut i = Interner::new();
        // i32 x = foo();
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::Identifier, "foo"),
            tok(&mut i, TokenKind::LeftParen, "("),
            tok(&mut i, TokenKind::RightParen, ")"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_nested_function_calls() {
        let mut i = Interner::new();
        // i32 x = foo(bar(1), baz(2 + 3));
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::Identifier, "foo"),
            tok(&mut i, TokenKind::LeftParen, "("),
            tok(&mut i, TokenKind::Identifier, "bar"),
            tok(&mut i, TokenKind::LeftParen, "("),
            tok(&mut i, TokenKind::IntegerLiteral, "1"),
            tok(&mut i, TokenKind::RightParen, ")"),
            tok(&mut i, TokenKind::Comma, ","),
            tok(&mut i, TokenKind::Identifier, "baz"),
            tok(&mut i, TokenKind::LeftParen, "("),
            tok(&mut i, TokenKind::IntegerLiteral, "2"),
            tok(&mut i, TokenKind::Plus, "+"),
            tok(&mut i, TokenKind::IntegerLiteral, "3"),
            tok(&mut i, TokenKind::RightParen, ")"),
            tok(&mut i, TokenKind::RightParen, ")"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_array_index_expression() {
        let mut i = Interner::new();
        // i32 x = arr[1 + 2];
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::Identifier, "arr"),
            tok(&mut i, TokenKind::LeftBracket, "["),
            tok(&mut i, TokenKind::IntegerLiteral, "1"),
            tok(&mut i, TokenKind::Plus, "+"),
            tok(&mut i, TokenKind::IntegerLiteral, "2"),
            tok(&mut i, TokenKind::RightBracket, "]"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_multi_dimensional_array_access() {
        let mut i = Interner::new();
        // i32 x = matrix[0][1];
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::Identifier, "matrix"),
            tok(&mut i, TokenKind::LeftBracket, "["),
            tok(&mut i, TokenKind::IntegerLiteral, "0"),
            tok(&mut i, TokenKind::RightBracket, "]"),
            tok(&mut i, TokenKind::LeftBracket, "["),
            tok(&mut i, TokenKind::IntegerLiteral, "1"),
            tok(&mut i, TokenKind::RightBracket, "]"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok());
    }

    #[test]
    fn test_expression_everything_combined() {
        let mut i = Interner::new();
        // i32 x = foo(a + b * c)[bar(2)] - -baz();
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::Identifier, "foo"),
            tok(&mut i, TokenKind::LeftParen, "("),
            tok(&mut i, TokenKind::Identifier, "a"),
            tok(&mut i, TokenKind::Plus, "+"),
            tok(&mut i, TokenKind::Identifier, "b"),
            tok(&mut i, TokenKind::Multiplication, "*"),
            tok(&mut i, TokenKind::Identifier, "c"),
            tok(&mut i, TokenKind::RightParen, ")"),
            tok(&mut i, TokenKind::LeftBracket, "["),
            tok(&mut i, TokenKind::Identifier, "bar"),
            tok(&mut i, TokenKind::LeftParen, "("),
            tok(&mut i, TokenKind::IntegerLiteral, "2"),
            tok(&mut i, TokenKind::RightParen, ")"),
            tok(&mut i, TokenKind::RightBracket, "]"),
            tok(&mut i, TokenKind::Minus, "-"),
            tok(&mut i, TokenKind::Minus, "-"),
            tok(&mut i, TokenKind::Identifier, "baz"),
            tok(&mut i, TokenKind::LeftParen, "("),
            tok(&mut i, TokenKind::RightParen, ")"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_ok(), "Failed complex mixed expression");
    }

    #[test]
    fn test_trailing_comma_in_call_should_fail() {
        let mut i = Interner::new();
        // foo(1,)
        let tokens = vec![
            tok(&mut i, TokenKind::Identifier, "foo"),
            tok(&mut i, TokenKind::LeftParen, "("),
            tok(&mut i, TokenKind::IntegerLiteral, "1"),
            tok(&mut i, TokenKind::Comma, ","),
            tok(&mut i, TokenKind::RightParen, ")"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_err());
    }

    #[test]
    fn test_unclosed_parenthesis_should_fail() {
        let mut i = Interner::new();
        // i32 x = (1 + 2;
        let tokens = vec![
            tok(&mut i, TokenKind::SignedInt32, "i32"),
            tok(&mut i, TokenKind::Identifier, "x"),
            tok(&mut i, TokenKind::Assignment, "="),
            tok(&mut i, TokenKind::LeftParen, "("),
            tok(&mut i, TokenKind::IntegerLiteral, "1"),
            tok(&mut i, TokenKind::Plus, "+"),
            tok(&mut i, TokenKind::IntegerLiteral, "2"),
            tok(&mut i, TokenKind::Semicolon, ";"),
            tok(&mut i, TokenKind::Eof, ""),
        ];
        assert!(parse_test(tokens).is_err());
    }
}