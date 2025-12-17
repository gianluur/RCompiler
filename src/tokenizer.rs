use core::fmt;
use std::{char, collections::HashMap, iter::Peekable, str::Chars};

use crate::error::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Eof,
    Semicolon,
    Comma,
    Dot,
    IntegerLiteral,
    FloatLiteral,
    CharLiteral,
    StringLiteral,
    Plus,
    Minus,
    Multiplication,
    Division,
    Modulus,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseLShift,
    BitwiseRShift,
    And,
    Or,
    Not,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Assignment,
    AddAssignment,
    SubtractAssignment,
    MultiplyAssignment,
    DivideAssignment,
    ModulusAssignment,
    BitwiseAndAssignment,
    BitwiseOrAssignment,
    BitwiseXorAssignment,
    BitwiseLShiftAssignment,
    BitwiseRShiftAssignment,
    Identifier,
    SignedInt8,
    SignedInt16,
    SignedInt32,
    SignedInt64,
    UnsignedInt8,
    UnsignedInt16,
    UnsignedInt32,
    UnsignedInt64,
    Float32,
    Float64,
    If,
    ElseIf,
    Else,
    While,
    Break,
    Continue,
    Function,
    Return,
    True,
    False,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eof => write!(f, "EOF"),
            Self::Comma => write!(f, "Comma"),
            Self::Dot => write!(f, "Dot"),
            Self::Semicolon => write!(f, "Semicolon"),
            Self::IntegerLiteral => write!(f, "IntegerLiteral"),
            Self::FloatLiteral => write!(f, "FloatLiteral"),
            Self::CharLiteral => write!(f, "CharLiteral"),
            Self::StringLiteral => write!(f, "StringLiteral"),
            Self::Plus => write!(f, "Plus"),
            Self::Minus => write!(f, "Minus"),
            Self::Multiplication => write!(f, "Multiplication"),
            Self::Division => write!(f, "Division"),
            Self::Modulus => write!(f, "Modulus"),
            Self::LeftParen => write!(f, "LeftParen"),
            Self::RightParen => write!(f, "RightParen"),
            Self::LeftBracket => write!(f, "LeftBracket"),
            Self::RightBracket => write!(f, "RightBracket"),
            Self::LeftBrace => write!(f, "LeftBrace"),
            Self::RightBrace => write!(f, "RightBrace"),
            Self::BitwiseAnd => write!(f, "BitwiseAnd"),
            Self::BitwiseOr => write!(f, "BitwiseOr"),
            Self::BitwiseXor => write!(f, "BitwiseXor"),
            Self::BitwiseLShift => write!(f, "BitwiseLShift"),
            Self::BitwiseRShift => write!(f, "BitwiseRShift"),
            Self::And => write!(f, "And"),
            Self::Or => write!(f, "Or"),
            Self::Not => write!(f, "Not"),
            Self::Equal => write!(f, "Equal"),
            Self::NotEqual => write!(f, "NotEqual"),
            Self::GreaterThan => write!(f, "GreaterThan"),
            Self::LessThan => write!(f, "LessThan"),
            Self::GreaterThanOrEqual => write!(f, "GreaterThanOrEqual"),
            Self::LessThanOrEqual => write!(f, "LessThanOrEqual"),
            Self::Assignment => write!(f, "Assignment"),
            Self::AddAssignment => write!(f, "AddAssignment"),
            Self::SubtractAssignment => write!(f, "SubtractAssignment"),
            Self::MultiplyAssignment => write!(f, "MultiplyAssignment"),
            Self::DivideAssignment => write!(f, "DivideAssignment"),
            Self::ModulusAssignment => write!(f, "ModulusAssignment"),
            Self::BitwiseAndAssignment => write!(f, "BitwiseAndAssignment"),
            Self::BitwiseOrAssignment => write!(f, "BitwiseOrAssignment"),
            Self::BitwiseXorAssignment => write!(f, "BitwiseXorAssignment"),
            Self::BitwiseLShiftAssignment => write!(f, "BitwiseLShiftAssignment"),
            Self::BitwiseRShiftAssignment => write!(f, "BitwiseRShiftAssignment"),
            Self::Identifier => write!(f, "Identifier"),
            Self::SignedInt8 => write!(f, "SignedInt8"),
            Self::SignedInt16 => write!(f, "SignedInt16"),
            Self::SignedInt32 => write!(f, "SignedInt32"),
            Self::SignedInt64 => write!(f, "SignedInt64"),
            Self::UnsignedInt8 => write!(f, "UnsignedInt8"),
            Self::UnsignedInt16 => write!(f, "UnsignedInt16"),
            Self::UnsignedInt32 => write!(f, "UnsignedInt32"),
            Self::UnsignedInt64 => write!(f, "UnsignedInt64"),
            Self::Float32 => write!(f, "Float32"),
            Self::Float64 => write!(f, "Float64"),
            Self::If => write!(f, "If"),
            Self::ElseIf => write!(f, "ElseIf"),
            Self::Else => write!(f, "Else"),
            Self::While => write!(f, "While"),
            Self::Break => write!(f, "Break"),
            Self::Continue => write!(f, "Continue"),
            Self::Function => write!(f, "Function"),
            Self::Return => write!(f, "Return"),
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
        }
    }
}

// --- TOKEN SPAN ---

#[derive(Debug, PartialEq)]
pub struct TokenSpan {
    pub start: usize,
    pub end: usize,
    pub literal: String
}

impl TokenSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self { start, end, literal }
    }
}

impl fmt::Display for TokenSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.literal)
    }
}

#[derive(Debug, PartialEq)]
pub struct TokenizerError {
    pub code: ErrorCode,
    pub span: TokenSpan,
    pub line: usize,
    pub column: usize
}

impl TokenizerError {
    pub fn to_diagnostic(&self, filename: String) -> Diagnostic {
        Diagnostic {
            kind: DiagnosticKind::Error(self.code),
            info: DiagnosticInfo { 
                filename, 
                line: self.line, 
                column: self.column 
            },
            hint: Some(self.get_hint(self.code))
        }
    }

    fn get_hint(&self, code: ErrorCode) -> String {
        match code {
            // Numeric Literals
            ErrorCode::ET001 => "remove the extra '.'".to_string(),
            ErrorCode::ET002 => "add a space between the number and the identifier".to_string(),
            ErrorCode::ET003 => "add a '0' after the decimal or remove the point".to_string(),
            
            // Lexing / Characters
            ErrorCode::ET004 => "this character is not supported in this position".to_string(),
            ErrorCode::ET005 => "if you intended to use a string, use double quotes \"\" instead".to_string(),
            ErrorCode::ET006 => "character literals cannot be empty; maybe you meant ' '?".to_string(),
            ErrorCode::ET007 => "add a closing single quote (') to end the character literal".to_string(),
            ErrorCode::ET008 => "escaped quotes like \\' do not count as closing quotes unless preceded by a valid escape".to_string(),
            
            // Escape Sequences
            ErrorCode::ET009 => "valid escapes include \\n, \\r, \\t, \\\\, \\0, \\', and \\\"".to_string(),
            ErrorCode::ET010 => "octal escapes must be in the range \\000 to \\377".to_string(),
            ErrorCode::ET011 => "hexadecimal escapes must follow the pattern \\xHH (e.g., \\x1A)".to_string(),
            
            // String Literals
            ErrorCode::ET012 => "add a double quote (\") to the end of this line".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TokenSpan,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Kind: {} | Literal: {}", self.kind, self.span)
    }
}

pub struct Tokenizer<'a> {
    // Input and a list of all keywords
    input: Peekable<Chars<'a>>,
    keywords: HashMap<&'a str, TokenKind>,

    // Some general context
    character: char,
    position: usize,
    is_parsing_string_literal: bool,

    // Needed for diagnostics
    line: usize,
    column: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut keywords: HashMap<&str, TokenKind> = HashMap::new();
        keywords.insert("if", TokenKind::If);
        keywords.insert("elif", TokenKind::ElseIf);
        keywords.insert("else", TokenKind::Else);
        keywords.insert("while", TokenKind::While);
        keywords.insert("break", TokenKind::Break);
        keywords.insert("continue", TokenKind::Continue);
        keywords.insert("fn", TokenKind::Function);
        keywords.insert("return", TokenKind::Return);
        keywords.insert("true", TokenKind::True);
        keywords.insert("false", TokenKind::False);
        keywords.insert("i8", TokenKind::SignedInt8);
        keywords.insert("i16", TokenKind::SignedInt16);
        keywords.insert("i32", TokenKind::SignedInt32);
        keywords.insert("i64", TokenKind::SignedInt64);
        keywords.insert("u8", TokenKind::UnsignedInt8);
        keywords.insert("u16", TokenKind::UnsignedInt16);
        keywords.insert("u32", TokenKind::UnsignedInt32);
        keywords.insert("u64", TokenKind::UnsignedInt64);
        keywords.insert("f32", TokenKind::Float32);
        keywords.insert("f64", TokenKind::Float64);

        Self { 
            input: input.chars().peekable(), 
            keywords,

            character: ' ', 
            position: 0,
            is_parsing_string_literal: false,

            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerError> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(_) = self.next() {
            if self.character.is_whitespace() {
                continue;
            }
            if let Some(token) = self.get_token(self.position - 1)? {
                tokens.push(token);
            }
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            span: TokenSpan::new(self.position, self.position, String::from("")),
        });

        Ok(tokens)
    }

    fn get_token(&mut self, start: usize) -> Result<Option<Token>, TokenizerError> {
        if self.character.is_ascii_digit() {
            self.parse_number(start)
        } 
        else if self.character.is_ascii_alphanumeric() || self.character == '_' {
            self.parse_text(start)
        }
        else {
            self.parse_symbol(start)
        }
    }

    fn parse_number(&mut self, start: usize) -> Result<Option<Token>, TokenizerError> {
        let mut literal: String = String::from(self.character); 
        let mut dots: i8 = 0; 

        while let Some(next) = self.peek() { 
            if !(next.is_ascii_digit()) && next != '.' {
                break;
            }
            if next == '.' {
                dots += 1;
            }
            literal.push(self.next().unwrap());
        }

        if dots > 1 {
            return Err(
                TokenizerError {
                    code: ErrorCode::ET001,
                    span: TokenSpan::new(start, self.position, literal),
                    column: self.column,
                    line: self.line
                }
            );
        }

        let mut invalid: String = String::from("");
        while let Some(next) = self.peek() {
            if next.is_ascii_alphanumeric() || next == '_' {
                invalid.push(self.next().unwrap());
            }
            else {
                break;
            }
        }

        if invalid.len() > 0 {
            return Err(TokenizerError {
                code: ErrorCode::ET002,
                span: TokenSpan::new(start, self.position, format!("{}{}", literal, invalid)),
                line: self.line,
                column: self.column,
            });
        }

        if dots == 0 {
            return Ok(Some(Token { 
                kind: TokenKind::IntegerLiteral, 
                span: TokenSpan::new(start, self.position, literal) 
            }));
        }
        else {
            if literal.ends_with('.') {
                return Err(
                    TokenizerError {
                        code: ErrorCode::ET003,
                        span: TokenSpan::new(start, self.position, literal),
                        column: self.column,
                        line: self.line
                    }
                );
            }

            return Ok(Some(Token {
                kind: TokenKind::FloatLiteral,
                span: TokenSpan::new(start, self.position, literal)
            }))
        }
    }

    fn parse_text(&mut self, start: usize) -> Result<Option<Token>, TokenizerError> {
        let mut literal: String = String::from(self.character);
        while let Some(next) = self.peek() {
            if next.is_ascii_alphanumeric() || next == '_' {
                literal.push(self.next().unwrap());
            }
            else {
                break;
            }
        }

        if let Some(&keyword) = self.keywords.get(&literal.as_str()) {
            return Ok(Some(Token {
                kind: keyword,
                span: TokenSpan::new(start, self.position, literal)
            }));
        }
        else {
            return Ok(Some(Token {
                kind: TokenKind::Identifier,
                span: TokenSpan::new(start, self.position, literal)
            }));
        }
    }
    
    fn parse_symbol(&mut self, start: usize) -> Result<Option<Token>, TokenizerError> {
        match self.character {
            '+' => {
                if self.peek() == Some('=') {
                    self.next();
                    Ok(Some(Token { 
                        kind: TokenKind::AddAssignment, 
                        span: TokenSpan::new(start, self.position, String::from("+="))
                    }))
                }
                else {
                    Ok(Some(Token {
                        kind: TokenKind::Plus,
                        span: TokenSpan::new(start, self.position, String::from("+"))
                    }))
                }
            },

            '-' => {
                if self.peek() == Some('=') {
                    self.next();
                    Ok(Some(Token { 
                        kind: TokenKind::SubtractAssignment, 
                        span: TokenSpan::new(start, self.position, String::from("-="))
                    }))
                }
                else {
                    Ok(Some(Token {
                        kind: TokenKind::Minus,
                        span: TokenSpan::new(start, self.position, String::from("-"))
                    }))
                }
            },
            
            '*' => {
                if self.peek() == Some('=') {
                    self.next();
                    Ok(Some(Token { 
                        kind: TokenKind::MultiplyAssignment, 
                        span: TokenSpan::new(start, self.position, String::from("*="))
                    }))
                }
                else {
                    Ok(Some(Token {
                        kind: TokenKind::Multiplication,
                        span: TokenSpan::new(start, self.position, String::from("*"))
                    }))
                }
            },

            '/' => {
                if self.peek() == Some('=') {
                    self.next();
                    Ok(Some(Token { 
                        kind: TokenKind::DivideAssignment, 
                        span: TokenSpan::new(start, self.position, String::from("/="))
                    }))
                }
                else {
                    Ok(Some(Token {
                        kind: TokenKind::Division,
                        span: TokenSpan::new(start, self.position, String::from("/"))
                    }))
                }
            },

            '%' => {
                if self.peek() == Some('=') {
                    self.next();
                    Ok(Some(Token { 
                        kind: TokenKind::ModulusAssignment, 
                        span: TokenSpan::new(start, self.position, String::from("%="))
                    }))
                }
                else {
                    Ok(Some(Token {
                        kind: TokenKind::Modulus,
                        span: TokenSpan::new(start, self.position, String::from("%"))
                    }))
                }
            },

            '&' => {
                if self.peek() == Some('=') {
                    self.next();
                    Ok(Some(Token { 
                        kind: TokenKind::BitwiseAndAssignment, 
                        span: TokenSpan::new(start, self.position, String::from("&="))
                    }))
                }

                else if self.peek() == Some('&') {
                    self.next();
                    Ok(Some(Token {
                        kind: TokenKind::And,
                        span: TokenSpan::new(start, self.position, String::from("&&"))
                    }))
                }

                else {
                    Ok(Some(Token {
                        kind: TokenKind::BitwiseAnd,
                        span: TokenSpan::new(start, self.position, String::from("&"))
                    }))
                }
            },

            '|' => {
                if self.peek() == Some('=') {
                    self.next();
                    Ok(Some(Token { 
                        kind: TokenKind::BitwiseOrAssignment, 
                        span: TokenSpan::new(start, self.position, String::from("|="))
                    }))
                }

                else if self.peek() == Some('|') {
                    self.next();
                    Ok(Some(Token {
                        kind: TokenKind::Or,
                        span: TokenSpan::new(start, self.position, String::from("||"))
                    }))
                }

                else {
                    Ok(Some(Token {
                        kind: TokenKind::BitwiseOr,
                        span: TokenSpan::new(start, self.position, String::from("|"))
                    }))
                }
            },

            '^' => {
                if self.peek() == Some('=') {
                    self.next();
                    Ok(Some(Token { 
                        kind: TokenKind::BitwiseXorAssignment, 
                        span: TokenSpan::new(start, self.position, String::from("^="))
                    }))
                }
                else {
                    Ok(Some(Token {
                        kind: TokenKind::BitwiseXor,
                        span: TokenSpan::new(start, self.position, String::from("^"))
                    }))
                }
            },

            '=' => {
                if self.peek() == Some('=') {
                    self.next();
                    Ok(Some(Token { 
                        kind: TokenKind::Equal, 
                        span: TokenSpan::new(start, self.position, String::from("=="))
                    }))
                }
                else {
                    Ok(Some(Token {
                        kind: TokenKind::Assignment,
                        span: TokenSpan::new(start, self.position, String::from("="))
                    }))
                }
            },

            '!' => {
                if self.peek() == Some('=') {
                    self.next();
                    Ok(Some(Token { 
                        kind: TokenKind::NotEqual, 
                        span: TokenSpan::new(start, self.position, String::from("!="))
                    }))
                }
                else {
                    Ok(Some(Token {
                        kind: TokenKind::Not,
                        span: TokenSpan::new(start, self.position, String::from("!"))
                    }))
                }
            },

            '<' => {
                if self.peek() == Some('<') {
                    self.next();

                    if self.peek() == Some('=') {
                        self.next();
                        Ok(Some(Token { 
                            kind: TokenKind::BitwiseLShiftAssignment, 
                            span: TokenSpan::new(start, self.position, String::from("<<="))
                        }))
                    }
                    else {
                        Ok(Some(Token { 
                            kind: TokenKind::BitwiseLShift, 
                            span: TokenSpan::new(start, self.position, String::from("<<"))
                        }))
                    }
                }

                else if self.peek() == Some('=') {
                    self.next();
                    Ok(Some(Token { 
                        kind: TokenKind::LessThanOrEqual, 
                        span: TokenSpan::new(start, self.position, String::from("<="))
                    }))
                }

                else {
                    Ok(Some(Token {
                        kind: TokenKind::LessThan,
                        span: TokenSpan::new(start, self.position, String::from("<"))
                    }))
                }
            },

            '>' => {
                if self.peek() == Some('>') {
                    self.next();

                    if self.peek() == Some('=') {
                        self.next();
                        Ok(Some(Token { 
                            kind: TokenKind::BitwiseRShiftAssignment, 
                            span: TokenSpan::new(start, self.position, String::from(">>="))
                        }))
                    }
                    else {
                        Ok(Some(Token { 
                            kind: TokenKind::BitwiseRShift, 
                            span: TokenSpan::new(start, self.position, String::from(">>"))
                        }))
                    }
                }

                else if self.peek() == Some('=') {
                    self.next();
                    Ok(Some(Token { 
                        kind: TokenKind::GreaterThanOrEqual, 
                        span: TokenSpan::new(start, self.position, String::from(">="))
                    }))
                }
                
                else {
                    Ok(Some(Token {
                        kind: TokenKind::GreaterThan,
                        span: TokenSpan::new(start, self.position, String::from(">"))
                    }))
                }
            },

            '(' => Ok(Some(Token { 
                kind: TokenKind::LeftParen, 
                span: TokenSpan::new(start, self.position, String::from("("))
            })),

            ')' => Ok(Some(Token { 
                kind: TokenKind::RightParen, 
                span: TokenSpan::new(start, self.position, String::from(")"))
            })),
            
            '[' => Ok(Some(Token { 
                kind: TokenKind::LeftBracket, 
                span: TokenSpan::new(start, self.position, String::from("["))
            })),

            ']' => Ok(Some(Token {
                kind: TokenKind::RightBracket, 
                span: TokenSpan::new(start, self.position, String::from("]"))
            })),
            
            '{' => Ok(Some(Token { 
                kind: TokenKind::LeftBrace, 
                span: TokenSpan::new(start, self.position, String::from("{"))
            })),

            '}' => Ok(Some(Token { 
                kind: TokenKind::RightBrace, 
                span: TokenSpan::new(start, self.position, String::from("}"))
            })),

            ',' => Ok(Some(Token { 
                kind: TokenKind::Comma, 
                span: TokenSpan::new(start, self.position, String::from(","))
            })),

            '.' => Ok(Some(Token { 
                kind: TokenKind::Dot, 
                span: TokenSpan::new(start, self.position, String::from("."))
            })),

            ';' => Ok(Some(Token { 
                kind: TokenKind::Semicolon, 
                span: TokenSpan::new(start, self.position, String::from(";"))
            })),

            '\'' => self.parse_char(start),
 
            '\"' => self.parse_string(start),

            '#' => self.parse_comment(),

            _ => Err(TokenizerError {
                code: ErrorCode::ET004,
                span: TokenSpan::new(start, self.position, String::from(self.character)),
                line: self.line,
                column: self.column
            })
        }
    }

    fn parse_char(&mut self, start: usize) -> Result<Option<Token>, TokenizerError> {
        let mut literal: String = String::from(self.character);
        let mut chars_count: u8 = 0;
        let mut is_escaping_single_quote: bool = false;

        while let Some(next) = self.peek() {
            if next == '\''{
                literal.push(self.next().unwrap());

                if chars_count > 1 {
                    return Err(TokenizerError {
                        code: ErrorCode::ET005,
                        span: TokenSpan::new(start, self.position, literal),
                        line: self.line,
                        column: self.column
                    });
                }
                else if chars_count == 0 {
                    return Err(TokenizerError {
                        code: ErrorCode::ET006,
                        span: TokenSpan::new(start, self.position, literal),
                        line: self.line,
                        column: self.column
                    });
                }

                is_escaping_single_quote = false;
                break;
            }

            literal.push(self.next().unwrap());

            if next == '\\' {
                is_escaping_single_quote = self.parse_inner_char(&mut literal)?;
            } 

            chars_count += 1;
        }

        if !literal.ends_with('\'') || literal.len() == 1  {
            return Err(TokenizerError {
                code: ErrorCode::ET007,
                span: TokenSpan::new(start, self.position, literal),
                line: self.line,
                column: self.column
            })
        }
        else if is_escaping_single_quote {
            return Err(TokenizerError {
                code: ErrorCode::ET008,
                span: TokenSpan::new(start, self.position, literal),
                line: self.line,
                column: self.column
            })
        }

        Ok(Some(Token {
            kind: TokenKind::CharLiteral,
            span: TokenSpan::new(start, self.position, literal)
        }))
    }

    fn parse_inner_char(&mut self, literal: &mut String) -> Result<bool, TokenizerError> {
        if self.character == '\\' {
            return self.parse_escape_sequence(literal);
        }

        literal.push(self.next().unwrap());
        return Ok(false)
    }

    fn parse_escape_sequence(&mut self, literal: &mut String) -> Result<bool, TokenizerError> {
        if let Some(next) = self.peek() {
            literal.push(self.next().unwrap());

            match next {
                'a' | 'b' | 'f' | 'n' | 
                'r' | 't' | 'v' | '\\' | 
                '\'' | '\"' | '?' | '0' => {
                    if next == '\'' {
                        return Ok(true);
                    }
                },
                
                '1'..='7' => self.parse_octal(literal)?,

                'x' => self.parse_hex(literal)?,

                _ => return Err(TokenizerError {
                    code: ErrorCode::ET009,
                    span: TokenSpan::new(self.position - literal.len(), self.position, literal.clone()),
                    line: self.line,
                    column: self.column
                }),
            };
        }

        Ok(false)
    } 

    fn is_ascii_octal(&mut self, character: char) -> Option<char> {
        match character {
            '0'..='7' => Some(character),
            _ => None
        }
    }

    fn parse_octal(&mut self, literal: &mut String) -> Result<(), TokenizerError> {
        let mut is_invalid: bool = false;
        let mut digit_count: u8 = 1; // Starts at 1 because we consumed the first octal digit in parse_escape_sequence

        let start_position = self.position - literal.len(); // Approximate start of the escape sequence

        while let Some(next) = self.peek() {
            if next == '\'' || (self.is_parsing_string_literal && next == '\"') {
                if is_invalid || digit_count > 3 {
                    return Err(TokenizerError {
                        code: ErrorCode::ET010,
                        span: TokenSpan::new(start_position, self.position, literal.clone()),
                        line: self.line,
                        column: self.column
                    });
                }
                break;
            }

            if !self.is_ascii_octal(next).is_some() {
                is_invalid = true;
            }

            literal.push(self.next().unwrap());
            digit_count += 1;
        }
        
        Ok(())
    }

    fn parse_hex(&mut self, literal: &mut String) -> Result<(), TokenizerError> {
        let mut is_invalid: bool = false;
        let mut digit_count: u8 = 0;
        
        let start_position = self.position - literal.len(); // Approximate start of the escape sequence

        while let Some(next) = self.peek() {
            if next == '\'' || (self.is_parsing_string_literal && next == '\"') {
                if digit_count > 2 || digit_count == 0 || is_invalid {
                    return Err(TokenizerError {
                        code: ErrorCode::ET011,
                        span: TokenSpan::new(start_position, self.position, literal.clone()),
                        line: self.line,
                        column: self.column
                    });
                }
                break;
            }

            if !next.is_ascii_hexdigit() {
                if next == '\\' && digit_count == 2 {
                    break;
                }
                is_invalid = true;
            }

            literal.push(self.next().unwrap());
            digit_count += 1;
        }

        Ok(())
    }

    fn parse_string(&mut self, start: usize) -> Result<Option<Token>, TokenizerError> {
        let mut literal: String = String::from(self.character);
        self.is_parsing_string_literal = true;

        while let Some(next) = self.peek() {
            if next == '\"' {
                literal.push(self.next().unwrap());
                break;
            }

            literal.push(self.next().unwrap());

            if next == '\\' {
                self.parse_inner_char(&mut literal)?; 
            }
        }

        if !literal.ends_with('\"') || literal.len() == 1 {
            return Err(TokenizerError {
                code: ErrorCode::ET012,
                span: TokenSpan::new(start, self.position, literal),
                line: self.line,
                column: self.column
            })
        }

        self.is_parsing_string_literal = false;

        Ok(Some(Token {
            kind: TokenKind::StringLiteral,
            span: TokenSpan::new(start, self.position, literal)
        }))
    }

    fn parse_comment(&mut self) -> Result<Option<Token>, TokenizerError> {
        let mut literal: String = String::from(self.character);
        while let Some(next) = self.peek() {
            if next == '\n' {
                break;
            }
            literal.push(self.next().unwrap());
        }
        Ok(None)
    }

    fn next(&mut self) -> Option<char> {
        match self.input.next() {
            Some(c) => {
                if c == '\n' {
                    self.line += 1;
                    self.column = 1;
                }
                else {
                    self.column += 1;
                }

                self.character = c;
                self.position += 1;
                Some(c)
            }
            None => None
        }
    }

    fn peek(&mut self) -> Option<char> {
        match self.input.peek() {
            Some(c) => Some(*c),
            None => None
        }
    }
}