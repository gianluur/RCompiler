use rcompiler::tokenizer::*;
use rcompiler::error::ErrorCode; // Ensure ErrorCode is imported

#[cfg(test)]
mod simple_expressions {
    use super::*;

    #[test]
    fn test_simple_integer() {
        let mut tokenizer = Tokenizer::new("123");
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2); // 123 + EOF
        assert_eq!(tokens[0].kind, TokenKind::IntegerLiteral);
        assert_eq!(tokens[0].span.literal, "123");
    }

    #[test]
    fn test_float_literal() {
        let mut tokenizer = Tokenizer::new("3.14");
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::FloatLiteral);
        assert_eq!(tokens[0].span.literal, "3.14");
    }

    #[test]
    fn test_operators() {
        let mut tokenizer = Tokenizer::new("1+2-3");
        let tokens = tokenizer.tokenize().unwrap();

        // 1 + 2 - 3
        // [0] is 1, [1] is +, [2] is 2, [3] is -, [4] is 3
        assert_eq!(tokens[1].kind, TokenKind::Plus);
        assert_eq!(tokens[3].kind, TokenKind::Minus);
    }

    #[test]
    fn test_parentheses() {
        let mut tokenizer = Tokenizer::new("(1+2)");
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::LeftParen);
        assert_eq!(tokens[2].kind, TokenKind::Plus);
        assert_eq!(tokens[4].kind, TokenKind::RightParen);
    }

    #[test]
    fn test_invalid_float_two_dots() {
        let mut tokenizer = Tokenizer::new("1.2.3");
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();
        
        // ET001: More than one dot in number
        assert_eq!(err.code, ErrorCode::ET001); 
        assert_eq!(err.span.literal, "1.2.3");
    }

    #[test]
    fn test_complex_expression_no_whitespace() {
        let mut tokenizer = Tokenizer::new("(123+4.5)*6/7.8");
        let tokens = tokenizer.tokenize().unwrap();

        let expected_kinds = vec![
            TokenKind::LeftParen,
            TokenKind::IntegerLiteral,
            TokenKind::Plus,
            TokenKind::FloatLiteral,
            TokenKind::RightParen,
            TokenKind::Multiplication,
            TokenKind::IntegerLiteral,
            TokenKind::Division,
            TokenKind::FloatLiteral,
            TokenKind::Eof,
        ];

        let expected_literals = vec![
            "(", "123", "+", "4.5", ")", "*", "6", "/", "7.8", ""
        ];

        assert_eq!(tokens.len(), expected_kinds.len());

        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.kind, expected_kinds[i], "Token {} kind mismatch", i);
            assert_eq!(token.span.literal, expected_literals[i], "Token {} literal mismatch", i);
        }
    }

    #[test]
    fn test_mixed_expression_with_whitespace() {
        let mut tokenizer = Tokenizer::new(" ( 1 + 2 ) * 3 ");
        let tokens = tokenizer.tokenize().unwrap();

        let expected_kinds = vec![
            TokenKind::LeftParen,
            TokenKind::IntegerLiteral,
            TokenKind::Plus,
            TokenKind::IntegerLiteral,
            TokenKind::RightParen,
            TokenKind::Multiplication,
            TokenKind::IntegerLiteral,
            TokenKind::Eof,
        ];
        
        let expected_literals = vec![
            "(", "1", "+", "2", ")", "*", "3", ""
        ];

        assert_eq!(tokens.len(), expected_kinds.len());
        
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.kind, expected_kinds[i], "Token {} kind mismatch", i);
            assert_eq!(token.span.literal, expected_literals[i], "Token {} literal mismatch", i);
        }
    }

    #[test]
    fn test_single_characters() {
        let mut tokenizer = Tokenizer::new("+-*/()");
        let tokens = tokenizer.tokenize().unwrap();

        let expected_kinds = vec![
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Multiplication,
            TokenKind::Division,
            TokenKind::LeftParen,
            TokenKind::RightParen,
            TokenKind::Eof,
        ];

        assert_eq!(tokens.len(), expected_kinds.len());
        for i in 0..6 {
            assert_eq!(tokens[i].kind, expected_kinds[i]);
        }
    }

    #[test]
    fn test_single_digits() {
        let mut tokenizer = Tokenizer::new("1 2 3 4 5");
        let tokens = tokenizer.tokenize().unwrap();
        
        let expected_literals = vec!["1", "2", "3", "4", "5", ""];

        assert_eq!(tokens.len(), 6); // Five integers + EOF
        for i in 0..5 {
            assert_eq!(tokens[i].kind, TokenKind::IntegerLiteral);
            assert_eq!(tokens[i].span.literal, expected_literals[i]);
        }
    }

    #[test]
    fn test_float_trailing_dot() {
        let mut tokenizer = Tokenizer::new("123.");
        let result = tokenizer.tokenize();
        
        assert!(result.is_err());
        let err = result.unwrap_err();

        // ET003: Trailing dot found in the number
        assert_eq!(err.code, ErrorCode::ET003);
        assert_eq!(err.span.literal, "123.");
    }

    #[test]
    fn test_invalid_character_error() {
        let mut tokenizer = Tokenizer::new("123$456");
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();

        // The tokenizer processes 123, then hits $.
        // ET004: Unrecognized character
        assert_eq!(err.code, ErrorCode::ET004);
        assert_eq!(err.span.literal, "$");
    }

    #[test]
    fn test_empty_input() {
        let mut tokenizer = Tokenizer::new("");
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 1); // Only EOF
        assert_eq!(tokens[0].kind, TokenKind::Eof);
        assert_eq!(tokens[0].span.start, 0);
        assert_eq!(tokens[0].span.end, 0);
    }

    #[test]
    fn test_input_with_only_whitespace() {
        let mut tokenizer = Tokenizer::new("  \t\n  ");
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 1); // Only EOF
        assert_eq!(tokens[0].kind, TokenKind::Eof);
    }

    #[test]
    fn test_span_accuracy() {
        let mut tokenizer = Tokenizer::new("12 + 3.14");
        let tokens = tokenizer.tokenize().unwrap();
        
        // 12 -> start 0, end 2
        assert_eq!(tokens[0].span.literal, "12");
        assert_eq!(tokens[0].span.start, 0);
        assert_eq!(tokens[0].span.end, 2); 
        
        // + -> start 3, end 4 (space at 2, + at 3)
        assert_eq!(tokens[1].span.literal, "+");
        assert_eq!(tokens[1].span.start, 3);
        assert_eq!(tokens[1].span.end, 4);
        
        // 3.14 -> start 5, end 9 (space at 4, 3.14 at 5..9)
        assert_eq!(tokens[2].span.literal, "3.14");
        assert_eq!(tokens[2].span.start, 5);
        assert_eq!(tokens[2].span.end, 9);
    }

    #[test]
    fn test_zero_integer() {
        let mut tokenizer = Tokenizer::new("0");
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::IntegerLiteral);
        assert_eq!(tokens[0].span.literal, "0");
    }

    #[test]
    fn test_integer_leading_zeros() {
        let mut tokenizer = Tokenizer::new("007");
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::IntegerLiteral);
        assert_eq!(tokens[0].span.literal, "007");
    }

    #[test]
    fn test_invalid_char_after_number() {
        let mut tokenizer = Tokenizer::new("123a");
        let result = tokenizer.tokenize();

        assert!(result.is_err()); 
        let err = result.unwrap_err();

        // ET002: Numbers can't be immediately followed by letters
        assert_eq!(err.code, ErrorCode::ET002);
        // Your logic combines the number and the invalid part
        assert_eq!(err.span.literal, "123a"); 
    }

    #[test]
    fn test_invalid_char_with_whitespace() {
        let mut tokenizer = Tokenizer::new("1 + $");
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();

        // Should tokenize '1', '+', then fail on '$'
        // ET004: Unrecognized character
        assert_eq!(err.code, ErrorCode::ET004);
        assert_eq!(err.span.literal, "$");
    }

    #[test]
    fn test_sequential_operators() {
        let mut tokenizer = Tokenizer::new("++--");
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].kind, TokenKind::Plus);
        assert_eq!(tokens[1].kind, TokenKind::Plus);
        assert_eq!(tokens[2].kind, TokenKind::Minus);
        assert_eq!(tokens[3].kind, TokenKind::Minus);
        assert_eq!(tokens[4].kind, TokenKind::Eof);
    }

    #[test]
    fn test_eof_span_with_trailing_whitespace() {
        let input = "123  \n";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[1].kind, TokenKind::Eof);
        // EOF start/end should be the length of input
        assert_eq!(tokens[1].span.start, input.len());
        assert_eq!(tokens[1].span.end, input.len());
    }
}