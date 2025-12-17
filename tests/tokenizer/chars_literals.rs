use rcompiler::tokenizer::*;
use rcompiler::error::*;

#[cfg(test)]
mod char_literal_tests {
    use super::*;

    fn tokenize(input: &str) -> Result<Vec<Token>, TokenizerError> {
        let mut tokenizer = Tokenizer::new(input);
        tokenizer.tokenize()
    }

    macro_rules! assert_char_literal_success {
        ($input:expr, $expected_literal:expr) => {{
            let result = tokenize($input);
            assert!(result.is_ok(), "Input: \"{}\" failed with error: {:?}", $input, result.err());

            let tokens = result.unwrap();
            
            assert_eq!(tokens.len(), 2, "Input: \"{}\". Expected 2 tokens, got {}", $input, tokens.len());
            
            let char_token = &tokens[0];
            assert_eq!(char_token.kind, TokenKind::CharLiteral, "Input: \"{}\". Token kind mismatch.", $input);
            assert_eq!(char_token.span.literal, $expected_literal, "Input: \"{}\". Literal value mismatch.", $input);
        }};
    }

    macro_rules! assert_char_literal_error {
        ($input:expr, $expected_error_code:path, $description:expr) => {{
            let result = tokenize($input);
            assert!(result.is_err(), "Input: \"{}\" unexpectedly succeeded", $input);
            
            let err = result.unwrap_err();
            
            assert_eq!(
                err.code, 
                $expected_error_code, 
                "Input: \"{}\". Expected error code {:?}, but got {:?} (Literal: {})", 
                $input, 
                $expected_error_code, 
                err.code, 
                err.span.literal
            );
        }};
    }


    #[test]
    fn test_basic_char() {
        assert_char_literal_success!("'a'", "'a'");
    }

    #[test]
    fn test_space_char() {
        assert_char_literal_success!("' '", "' '");
    }

    #[test]
    fn test_digit_char() {
        assert_char_literal_success!("'1'", "'1'");
    }

    #[test]
    fn test_symbol_char() {
        assert_char_literal_success!("'+'", "'+'");
    }


    #[test]
    fn test_escape_newline() {
        assert_char_literal_success!("'\\n'", "'\\n'");
    }

    #[test]
    fn test_escape_backslash() {
        assert_char_literal_success!("'\\\\'", "'\\\\'");
    }

    #[test]
    fn test_escape_single_quote() {
        assert_char_literal_success!("'\\''", "'\\''");
    }
    
    #[test]
    fn test_escape_double_quote() {
        assert_char_literal_success!("'\"'", "'\"'");
    }

    #[test]
    fn test_octal_1_digit() {
        assert_char_literal_success!("'\\1'", "'\\1'");
    }
    
    #[test]
    fn test_octal_2_digits() {
        assert_char_literal_success!("'\\12'", "'\\12'");
    }

    #[test]
    fn test_octal_3_digits() {
        assert_char_literal_success!("'\\123'", "'\\123'");
    }
    
    #[test]
    fn test_octal_max_value() {
        assert_char_literal_success!("'\\377'", "'\\377'");
    }

    #[test]
    fn test_hex_2_digits_lower() {
        assert_char_literal_success!("'\\xa5'", "'\\xa5'");
    }

    #[test]
    fn test_hex_2_digits_upper() {
        assert_char_literal_success!("'\\xAF'", "'\\xAF'");
    }

    #[test]
    fn test_hex_min_value() {
        assert_char_literal_success!("'\\x00'", "'\\x00'");
    }


    #[test]
    fn test_empty_literal() {
        assert_char_literal_error!("''", ErrorCode::ET006, "Empty char literal");
    }

    #[test]
    fn test_multiple_characters() {
        assert_char_literal_error!("'ab'", ErrorCode::ET005, "Too many characters");
    }

    #[test]
    fn test_multiple_chars_with_space() {
        assert_char_literal_error!("'a b'", ErrorCode::ET005, "Too many characters with space");
    }

    #[test]
    fn test_multiple_chars_with_escape() {
        assert_char_literal_error!("'a\\n'", ErrorCode::ET005, "Too many characters (char + escape)");
    }
    
    #[test]
    fn test_missing_end_quote_eof() {
        assert_char_literal_error!("'a", ErrorCode::ET007, "Missing end quote (EOF)");
    }

    #[test]
    fn test_escaping_missing_quote_eof() {
        assert_char_literal_error!("'\\", ErrorCode::ET007, "Missing end quote (escape)");
    }


    #[test]
    fn test_invalid_simple_escape() {
        assert_char_literal_error!("'\\z'", ErrorCode::ET009, "Invalid simple escape");
    }

    #[test]
    fn test_eof_after_backslash() {
        assert_char_literal_error!("'\\", ErrorCode::ET007, "EOF after backslash");
    }


    #[test]
    fn test_octal_too_many_digits() {
        assert_char_literal_error!("'\\1234'", ErrorCode::ET010, "Octal too many digits");
    }

    #[test]
    fn test_octal_invalid_digit_after_octal() {
        assert_char_literal_error!("'\\1a'", ErrorCode::ET010, "Octal invalid digit 'a'");
    }


    #[test]
    fn test_hex_too_many_digits() {
        assert_char_literal_error!("'\\x123'", ErrorCode::ET011, "Hex too many digits");
    }

    #[test]
    fn test_hex_too_few_digits() {
        assert_char_literal_error!("'\\x'", ErrorCode::ET011, "Hex too few digits");
    }

    #[test]
    fn test_hex_invalid_digit() {
        assert_char_literal_error!("'\\xg'", ErrorCode::ET011, "Hex invalid digit 'g'");
    }

    #[test]
    fn test_hex_invalid_chars_after_hex() {
        assert_char_literal_error!("'\\x12a'", ErrorCode::ET011, "Hex invalid chars after");
    }
}