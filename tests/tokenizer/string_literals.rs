use rcompiler::tokenizer::*;
use rcompiler::error::ErrorCode;

#[cfg(test)]
mod string_literal_tests {
    use super::*;

    fn get_string_literals<'a>(input: &'a str) -> Result<Vec<&'a str>, TokenizerError<'a>> {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize()?;
        let mut out = Vec::new();
        for t in tokens {
            if matches!(t.kind, TokenKind::StringLiteral) {
                // No clone! We just store the reference (&str)
                out.push(t.span.literal);
            }
        }
        Ok(out)
    }

    macro_rules! assert_string_literal_error {
        ($input:expr, $expected_error_code:path) => {{
            let result = get_string_literals($input);
            assert!(result.is_err(), "Input: \"{}\" unexpectedly succeeded", $input);
            
            let err = result.unwrap_err();
            
            assert_eq!(
                err.code, 
                $expected_error_code, 
                "Input: \"{}\". Expected error code {:?}, but got {:?}", 
                $input, 
                $expected_error_code, 
                err.code
            );
        }};
    }

    #[test]
    fn test_valid_strings() {
        let valid_cases = vec![
            (r#""hello""#, r#""hello""#),
            (r#""he said \"hi\"""#, r#""he said \"hi\"""#),
            (r#""foo\\bar""#, r#""foo\\bar""#),
            (r#""\n\t\r\a\b\f\v\\\'\"?""#, r#""\n\t\r\a\b\f\v\\\'\"?""#),
            (r#""\123""#, r#""\123""#),
            (r#""\xFF""#, r#""\xFF""#),
            (r#""abc\x12\077xyz""#, r#""abc\x12\077xyz""#),
            (r#""abc\"""#, r#""abc\"""#),
        ];

        for (input, expected) in valid_cases {
            match get_string_literals(input) {
                Ok(strings) => {
                    assert_eq!(strings.len(), 1, "Expected exactly 1 literal for input: {input}");
                    assert_eq!(strings[0], expected, "Literal mismatch for input: {input}");
                }
                Err(e) => panic!("Unexpected tokenizer error for input `{input}`: {:?}", e),
            }
        }
    }

    #[test]
    fn test_invalid_strings() {
        assert_string_literal_error!(r#"""#, ErrorCode::ET012);
        assert_string_literal_error!(r#""hello"#, ErrorCode::ET012);
        
        assert_string_literal_error!(r#""abc\"#, ErrorCode::ET012);

        assert_string_literal_error!(r#""\z""#, ErrorCode::ET009);

        assert_string_literal_error!(r#""\x""#, ErrorCode::ET011);
        
        assert_string_literal_error!(r#""\xFFF""#, ErrorCode::ET011);
        
        assert_string_literal_error!(r#""\7777""#, ErrorCode::ET010);
        
        assert_string_literal_error!("\"hello \\\nworld\"", ErrorCode::ET009);
    }

    #[test]
    fn test_bug_trigger_cases() {
        
        match get_string_literals(r#""a\b""#) {
            Ok(v) => assert_eq!(v[0], r#""a\b""#),
            Err(e) => panic!("`\"a\\b\"` unexpectedly errored: {:?}", e),
        }
        
        match get_string_literals(r#""\\""#) {
            Ok(v) => assert_eq!(v[0], r#""\\""#),
            Err(e) => panic!("`\"\\\\\"` unexpectedly errored: {:?}", e),
        }

        match get_string_literals(r#""\nX""#) {
            Ok(v) => assert_eq!(v[0], r#""\nX""#),
            Err(e) => panic!("`\"\\nX\"` unexpectedly errored: {:?}", e),
        }
    }
}