use rcompiler::tokenizer::*;
use rcompiler::error::ErrorCode;

#[cfg(test)]
mod tokenizer_error_tests {
    use super::*;

    // --- 1. NUMERIC LITERAL ERRORS ---

    #[test]
    fn test_err_et001_multiple_decimals() {
        let mut tokenizer = Tokenizer::new("1.2.3");
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::ET001); // Numeric literal contains multiple decimal points
        assert_eq!(err.span.literal, "1.2.3");
    }

    #[test]
    fn test_err_et002_identifier_after_literal() {
        let mut tokenizer = Tokenizer::new("42units");
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::ET002); // Identifiers cannot immediately follow a numeric literal
        assert_eq!(err.span.literal, "42units");
    }

    #[test]
    fn test_err_et003_trailing_decimal() {
        let mut tokenizer = Tokenizer::new("123.");
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::ET003); // Numeric literal cannot end with a trailing decimal point
        assert_eq!(err.span.literal, "123.");
    }

    // --- 2. LEXING / CHARACTER ERRORS ---

    #[test]
    fn test_err_et004_unexpected_character() {
        let mut tokenizer = Tokenizer::new("@");
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::ET004); // Unexpected or unrecognized character
        assert_eq!(err.span.literal, "@");
    }

    #[test]
    fn test_err_et005_too_many_chars_in_literal() {
        let mut tokenizer = Tokenizer::new("'abc'");
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::ET005); // Character literal must contain exactly one character
        assert_eq!(err.span.literal, "'abc'");
    }

    #[test]
    fn test_err_et006_empty_char_literal() {
        let mut tokenizer = Tokenizer::new("''");
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::ET006); // Character literal cannot be empty
        assert_eq!(err.span.literal, "''");
    }

    #[test]
    fn test_err_et007_unterminated_char() {
        let mut tokenizer = Tokenizer::new("'a");
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::ET007); // Unterminated character literal
        assert_eq!(err.span.literal, "'a");
    }

    #[test]
    fn test_err_et008_escaped_quote_as_terminator() {
        // Here the quote is escaped \', so it doesn't close the literal
        let mut tokenizer = Tokenizer::new("'\\'");
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::ET008); // Closing quote is being escaped
    }

    // --- 3. ESCAPE SEQUENCES ---

    #[test]
    fn test_err_et009_invalid_escape() {
        let mut tokenizer = Tokenizer::new("'\\z'");
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::ET009); // Unknown or invalid escape sequence
    }

    #[test]
    fn test_err_et010_invalid_octal() {
        // Octal escapes max out at \377 (255)
        let mut tokenizer = Tokenizer::new("'\\4000'"); 
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::ET010); // Invalid octal character escape
    }

    #[test]
    fn test_err_et011_invalid_hex() {
        // Hex escapes must be \xHH (exactly two hex digits)
        let mut tokenizer = Tokenizer::new("'\\xG1'"); 
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::ET011); // Invalid hexadecimal character escape
    }

    // --- 4. STRING LITERAL ERRORS ---

    #[test]
    fn test_err_et012_unterminated_string() {
        let mut tokenizer = Tokenizer::new("\"this string never ends");
        let result = tokenizer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, ErrorCode::ET012); // Missing closing double quote
    }
}