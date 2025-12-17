use rcompiler::tokenizer::*;
use rcompiler::error::ErrorCode;

#[cfg(test)]
mod keywords_and_operators {
    use super::*;

    #[test]
    fn test_all_keywords() {
        let mut tokenizer = Tokenizer::new("if elif else while break continue fn return true false");
        let tokens = tokenizer.tokenize().unwrap();

        let expected_kinds = vec![
            TokenKind::If,
            TokenKind::ElseIf,
            TokenKind::Else,
            TokenKind::While,
            TokenKind::Break,
            TokenKind::Continue,
            TokenKind::Function,
            TokenKind::Return,
            TokenKind::True,
            TokenKind::False,
            TokenKind::Eof,
        ];

        assert_eq!(tokens.len(), expected_kinds.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.kind, expected_kinds[i], "Token {} kind mismatch", i);
        }
    }

    #[test]
    fn test_all_type_keywords() {
        let mut tokenizer = Tokenizer::new("i8 i16 i32 i64 u8 u16 u32 u64 f32 f64");
        let tokens = tokenizer.tokenize().unwrap();

        let expected_kinds = vec![
            TokenKind::SignedInt8,
            TokenKind::SignedInt16,
            TokenKind::SignedInt32,
            TokenKind::SignedInt64,
            TokenKind::UnsignedInt8,
            TokenKind::UnsignedInt16,
            TokenKind::UnsignedInt32,
            TokenKind::UnsignedInt64,
            TokenKind::Float32,
            TokenKind::Float64,
            TokenKind::Eof,
        ];

        assert_eq!(tokens.len(), expected_kinds.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.kind, expected_kinds[i], "Token {} kind mismatch", i);
        }
    }

    #[test]
    fn test_keyword_prefix_as_identifier() {
        let mut tokenizer = Tokenizer::new("if_block i8var");
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].span.literal, "if_block");

        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].span.literal, "i8var");

        assert_eq!(tokens.len(), 3); // 2 Identifiers + EOF
    }

    #[test]
    fn test_relational_and_logical_operators() {
        let mut tokenizer = Tokenizer::new("== != > < >= <= && || !");
        let tokens = tokenizer.tokenize().unwrap();

        let expected_kinds = vec![
            TokenKind::Equal,
            TokenKind::NotEqual,
            TokenKind::GreaterThan,
            TokenKind::LessThan,
            TokenKind::GreaterThanOrEqual,
            TokenKind::LessThanOrEqual,
            TokenKind::And,
            TokenKind::Or,
            TokenKind::Not,
            TokenKind::Eof,
        ];

        let expected_literals = vec![
            "==", "!=", ">", "<", ">=", "<=", "&&", "||", "!", ""
        ];

        assert_eq!(tokens.len(), expected_kinds.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.kind, expected_kinds[i], "Token {} kind mismatch", i);
            assert_eq!(token.span.literal, expected_literals[i], "Token {} literal mismatch", i);
        }
    }

    #[test]
    fn test_bitwise_operators() {
        let mut tokenizer = Tokenizer::new("& | ^ << >>");
        let tokens = tokenizer.tokenize().unwrap();

        let expected_kinds = vec![
            TokenKind::BitwiseAnd,
            TokenKind::BitwiseOr,
            TokenKind::BitwiseXor,
            TokenKind::BitwiseLShift,
            TokenKind::BitwiseRShift,
            TokenKind::Eof,
        ];

        let expected_literals = vec![
            "&", "|", "^", "<<", ">>", ""
        ];

        assert_eq!(tokens.len(), expected_kinds.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.kind, expected_kinds[i], "Token {} kind mismatch", i);
            assert_eq!(token.span.literal, expected_literals[i], "Token {} literal mismatch", i);
        }
    }

    #[test]
    fn test_delimiters() {
        let mut tokenizer = Tokenizer::new("()[];,.");
        let tokens = tokenizer.tokenize().unwrap();

        let expected_kinds = vec![
            TokenKind::LeftParen,
            TokenKind::RightParen,
            TokenKind::LeftBracket,
            TokenKind::RightBracket,
            TokenKind::Semicolon,
            TokenKind::Comma,
            TokenKind::Dot,
            TokenKind::Eof,
        ];

        assert_eq!(tokens.len(), expected_kinds.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.kind, expected_kinds[i], "Token {} kind mismatch", i);
        }
    }

    #[test]
    fn test_all_compound_assignment_operators() {
        let mut tokenizer = Tokenizer::new(
            "a = b += c -= d *= e /= f %= g &= h |= i ^= j <<= k >>= l"
        );
        let tokens = tokenizer.tokenize().unwrap();

        let expected_kinds = vec![
            TokenKind::Identifier,
            TokenKind::Assignment,
            TokenKind::Identifier,
            TokenKind::AddAssignment,
            TokenKind::Identifier,
            TokenKind::SubtractAssignment,
            TokenKind::Identifier,
            TokenKind::MultiplyAssignment,
            TokenKind::Identifier,
            TokenKind::DivideAssignment,
            TokenKind::Identifier,
            TokenKind::ModulusAssignment,
            TokenKind::Identifier,
            TokenKind::BitwiseAndAssignment,
            TokenKind::Identifier,
            TokenKind::BitwiseOrAssignment,
            TokenKind::Identifier,
            TokenKind::BitwiseXorAssignment,
            TokenKind::Identifier,
            TokenKind::BitwiseLShiftAssignment,
            TokenKind::Identifier,
            TokenKind::BitwiseRShiftAssignment,
            TokenKind::Identifier,
            TokenKind::Eof,
        ];

        assert_eq!(tokens.len(), expected_kinds.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.kind, expected_kinds[i], "Token {} kind mismatch", i);
        }
    }

    #[test]
    fn test_bitwise_shift_and_shift_assignment_ambiguity() {
        let mut tokenizer = Tokenizer::new("a << b >>= c >> d <<= e");
        let tokens = tokenizer.tokenize().unwrap();

        let expected_kinds = vec![
            TokenKind::Identifier,
            TokenKind::BitwiseLShift,      // <<
            TokenKind::Identifier,
            TokenKind::BitwiseRShiftAssignment, // >>=
            TokenKind::Identifier,
            TokenKind::BitwiseRShift,       // >>
            TokenKind::Identifier,
            TokenKind::BitwiseLShiftAssignment, // <<=
            TokenKind::Identifier,
            TokenKind::Eof,
        ];

        let expected_literals = vec![
            "a", "<<", "b", ">>=", "c", ">>", "d", "<<=", "e", ""
        ];

        assert_eq!(tokens.len(), expected_kinds.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.kind, expected_kinds[i], "Token {} kind mismatch", i);
            assert_eq!(token.span.literal, expected_literals[i], "Token {} literal mismatch", i);
        }
    }

    #[test]
    fn test_identifiers_with_underscores() {
        let mut tokenizer = Tokenizer::new("_my_var_name WHILE_LOOP");
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].span.literal, "_my_var_name");
        
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].span.literal, "WHILE_LOOP");
    }

    #[test]
    fn test_keyword_as_substring_of_identifier() {
        let mut tokenizer = Tokenizer::new("if32_else i32_var u64_max");
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].span.literal, "if32_else");

        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].span.literal, "i32_var");
        
        assert_eq!(tokens[2].kind, TokenKind::Identifier);
        assert_eq!(tokens[2].span.literal, "u64_max");
    }

    #[test]
    fn test_float_no_leading_digit_as_dot_and_integer() {
        let mut tokenizer = Tokenizer::new(".123");
        let tokens = tokenizer.tokenize().unwrap();
        
        // As noted: parse_number requires a leading digit. 
        // This parses as Dot -> IntegerLiteral.
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::Dot);
        assert_eq!(tokens[1].kind, TokenKind::IntegerLiteral);
        assert_eq!(tokens[1].span.literal, "123");
    }

    #[test]
    fn test_float_with_trailing_zeros() {
        let mut tokenizer = Tokenizer::new("123.00");
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::FloatLiteral);
        assert_eq!(tokens[0].span.literal, "123.00");
    }

    #[test]
    fn test_invalid_character_after_valid_token() {
        let mut tokenizer = Tokenizer::new("myVar@");
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();

        // ET004: Unrecognized character
        assert_eq!(err.code, ErrorCode::ET004);
        assert_eq!(err.span.literal, "@");
    }

    #[test]
    fn test_compound_operator_followed_by_assignment() {
        let mut tokenizer = Tokenizer::new("a += = b");
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[1].kind, TokenKind::AddAssignment); // +=
        assert_eq!(tokens[2].kind, TokenKind::Assignment);    // =
    }

    #[test]
    fn test_bitwise_assignment_separation() {
        let mut tokenizer = Tokenizer::new("a & = b | = c ^ = d");
        let tokens = tokenizer.tokenize().unwrap();
        
        // Expected: Identifier, BitwiseAnd, Assignment, Identifier...
        // because of the space between & and =
        let expected_kinds = vec![
            TokenKind::Identifier, TokenKind::BitwiseAnd, TokenKind::Assignment,
            TokenKind::Identifier, TokenKind::BitwiseOr, TokenKind::Assignment,
            TokenKind::Identifier, TokenKind::BitwiseXor, TokenKind::Assignment,
            TokenKind::Identifier, TokenKind::Eof
        ];

        assert_eq!(tokens.len(), expected_kinds.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.kind, expected_kinds[i], "Token {} kind mismatch", i);
        }
    }
}