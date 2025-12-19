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
    Boolean,
    Character,
    String,
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
            Self::Comma => write!(f, ","),
            Self::Dot => write!(f, "."),
            Self::Semicolon => write!(f, ";"),
            Self::IntegerLiteral => write!(f, "IntegerLiteral"),
            Self::FloatLiteral => write!(f, "FloatLiteral"),
            Self::CharLiteral => write!(f, "CharLiteral"),
            Self::StringLiteral => write!(f, "StringLiteral"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Multiplication => write!(f, "*"),
            Self::Division => write!(f, "/"),
            Self::Modulus => write!(f, "%"),
            Self::LeftParen => write!(f, "("),
            Self::RightParen => write!(f, ")"),
            Self::LeftBracket => write!(f, "["),
            Self::RightBracket => write!(f, "]"),
            Self::LeftBrace => write!(f, "{{"),
            Self::RightBrace => write!(f, "}}"),
            Self::BitwiseAnd => write!(f, "&"),
            Self::BitwiseOr => write!(f, "|"),
            Self::BitwiseXor => write!(f, "^"),
            Self::BitwiseLShift => write!(f, ">>"),
            Self::BitwiseRShift => write!(f, "<<"),
            Self::And => write!(f, "&&"),
            Self::Or => write!(f, "||"),
            Self::Not => write!(f, "!"),
            Self::Equal => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
            Self::GreaterThan => write!(f, ">"),
            Self::LessThan => write!(f, "<LessThan>"),
            Self::GreaterThanOrEqual => write!(f, ">="),
            Self::LessThanOrEqual => write!(f, "<="),
            Self::Assignment => write!(f, "="),
            Self::AddAssignment => write!(f, "+="),
            Self::SubtractAssignment => write!(f, "-="),
            Self::MultiplyAssignment => write!(f, "*="),
            Self::DivideAssignment => write!(f, "/="),
            Self::ModulusAssignment => write!(f, "%="),
            Self::BitwiseAndAssignment => write!(f, "&="),
            Self::BitwiseOrAssignment => write!(f, "|="),
            Self::BitwiseXorAssignment => write!(f, "^="),
            Self::BitwiseLShiftAssignment => write!(f, ">>="),
            Self::BitwiseRShiftAssignment => write!(f, "<<="),
            Self::Identifier => write!(f, "Identifier"),
            Self::SignedInt8 => write!(f, "i8"),
            Self::SignedInt16 => write!(f, "i16"),
            Self::SignedInt32 => write!(f, "i32"),
            Self::SignedInt64 => write!(f, "i64"),
            Self::UnsignedInt8 => write!(f, "u8"),
            Self::UnsignedInt16 => write!(f, "u16"),
            Self::UnsignedInt32 => write!(f, "u32"),
            Self::UnsignedInt64 => write!(f, "u64"),
            Self::Float32 => write!(f, "f32"),
            Self::Float64 => write!(f, "f64"),
            Self::Boolean => write!(f, "bool"),
            Self::Character => write!(f, "char"),
            Self::String => write!(f, "str"),
            Self::If => write!(f, "if"),
            Self::ElseIf => write!(f, "elif"),
            Self::Else => write!(f, "else"),
            Self::While => write!(f, "while"),
            Self::Break => write!(f, "break"),
            Self::Continue => write!(f, "continue"),
            Self::Function => write!(f, "fn"),
            Self::Return => write!(f, "return"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TokenSpan<'a> {
    pub start: usize,
    pub end: usize,
    pub literal: &'a str
}

impl<'a> TokenSpan<'a> {
    pub fn new(start: usize, end: usize, literal: &'a str) -> Self {
        Self { start, end, literal }
    }
}

impl<'a> fmt::Display for TokenSpan<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.literal)
    }
}

#[derive(Debug, PartialEq)]
pub struct TokenizerError<'a> {
    pub code: ErrorCode,
    pub span: TokenSpan<'a> ,
    pub line: usize,
    pub column: usize
}

impl<'a> TokenizerError<'a>  {
    pub fn to_diagnostic(&'a self, filename: &'a str) -> Diagnostic<'a> {
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

    fn get_hint(&self, code: ErrorCode) -> &str {
        match code {
            ErrorCode::ET001 => "remove the extra '.'",
            ErrorCode::ET002 => "add a space between the number and the identifier",
            ErrorCode::ET003 => "add a '0' after the decimal or remove the point",
            ErrorCode::ET004 => "this character is not supported in this position",
            ErrorCode::ET005 => "if you intended to use a string, use double quotes \"\" instead",
            ErrorCode::ET006 => "character literals cannot be empty; maybe you meant ' '?",
            ErrorCode::ET007 => "add a closing single quote (') to end the character literal",
            ErrorCode::ET008 => "escaped quotes like \\' do not count as closing quotes unless preceded by a valid escape",
            ErrorCode::ET009 => "valid escapes include \\n, \\r, \\t, \\\\, \\0, \\', and \\\"",
            ErrorCode::ET010 => "octal escapes must be in the range \\000 to \\377",
            ErrorCode::ET011 => "hexadecimal escapes must follow the pattern \\xHH (e.g., \\x1A)",
            ErrorCode::ET012 => "add a double quote (\") to the end of this line",
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token<'a>  {
    pub kind: TokenKind,
    pub span: TokenSpan<'a> ,
}

impl<'a>  fmt::Display for Token<'a>  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Kind: {} | Literal: {}", self.kind, self.span)
    }
}

pub struct Tokenizer<'a> {
    source: &'a str,
    input: Peekable<Chars<'a>>,
    keywords: HashMap<&'a str, TokenKind>,

    character: char,
    start: usize,
    end: usize,
    is_parsing_string_literal: bool,

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
        keywords.insert("char", TokenKind::Character);
        keywords.insert("str", TokenKind::String);
        keywords.insert("bool", TokenKind::Boolean);

        Self { 
            source: input,
            input: input.chars().peekable(), 
            keywords,

            character: ' ', 
            start: 0,
            end: 0,
            is_parsing_string_literal: false,

            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token<'a>>, TokenizerError<'a>> {
        let mut tokens: Vec<Token<'a>> = Vec::new();

        while let Some(_) = self.next() {
            if self.character.is_whitespace() {
                self.start = self.end;
                continue;
            }
            if let Some(token) = self.get_token()? {
                tokens.push(token);
                self.start = self.end;
            }
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            span: TokenSpan::new(self.end, self.end, ""),
        });

        Ok(tokens)
    }

    fn get_token(&mut self) -> Result<Option<Token<'a>>, TokenizerError<'a>> {
        if self.character.is_ascii_digit() {
            self.parse_number()
        } 
        else if self.character.is_ascii_alphanumeric() || self.character == '_' {
            self.parse_text()
        }
        else {
            self.parse_symbol()
        }
    }

    fn parse_number(&mut self) -> Result<Option<Token<'a>>, TokenizerError<'a>> {
        let mut dots: i8 = 0; 

        while let Some(next) = self.peek() { 
            if !(next.is_ascii_digit()) && next != '.' {
                break;
            }
            if next == '.' {
                dots += 1;
            }
            self.next().unwrap();
        }

        if dots > 1 {
            return self.error(ErrorCode::ET001);
        }

        let mut invalid_found = false;
        while let Some(next) = self.peek() {
            if next.is_ascii_alphanumeric() || next == '_' {
                self.next().unwrap();
                invalid_found = true;
            }
            else {
                break;
            }
        }

        if invalid_found {
            return self.error(ErrorCode::ET002);
        }

        let literal: &str = &self.source[self.start..self.end];
        if dots == 0 {
            return self.token(TokenKind::IntegerLiteral);
        }
        else {
            if literal.ends_with('.') {
                return self.error(ErrorCode::ET003);
            }

            return self.token(TokenKind::FloatLiteral);
        }
    }

    fn parse_text(&mut self) -> Result<Option<Token<'a>>, TokenizerError<'a>> {
        while let Some(next) = self.peek() {
            if next.is_ascii_alphanumeric() || next == '_' {
                self.next().unwrap();
            }
            else {
                break;
            }
        }

        let literal: &str = &self.source[self.start..self.end];

        if let Some(&keyword) = self.keywords.get(literal) {
            return self.token(keyword);
        }
        else {
            return self.token(TokenKind::Identifier);
        }
    }
    
    fn parse_symbol(&mut self) -> Result<Option<Token<'a>>, TokenizerError<'a>> {
        match self.character {
            '+' => {
                if self.match_next('=') {
                    self.token(TokenKind::AddAssignment)
                }
                else {
                    self.token(TokenKind::Plus)
                }
            },

            '-' => {
                if self.match_next('=') {
                    self.token(TokenKind::SubtractAssignment)
                }
                else {
                    self.token(TokenKind::Minus)
                }
            },
            
            '*' => {
                if self.match_next('=') {
                    self.token(TokenKind::MultiplyAssignment)
                }
                else {
                    self.token(TokenKind::Multiplication)
                }
            },

            '/' => {
                if self.match_next('=') {
                    self.token(TokenKind::DivideAssignment)
                }
                else {
                    self.token(TokenKind::Division)
                }
            },

            '%' => {
                if self.match_next('=') {
                    self.token(TokenKind::ModulusAssignment)
                }
                else {
                    self.token(TokenKind::Modulus)
                }
            },

            '&' => {
                if self.match_next('=') {
                    self.token(TokenKind::BitwiseAndAssignment)
                }
                else if self.match_next('&') {
                    self.token(TokenKind::And)
                }
                else {
                    self.token(TokenKind::BitwiseAnd)
                }
            },

            '|' => {
                if self.match_next('=') {
                    self.token(TokenKind::BitwiseOrAssignment)
                }
                else if self.match_next('|') {
                    self.token(TokenKind::Or)
                }
                else {
                    self.token(TokenKind::BitwiseOr)
                }
            },

            '^' => {
                if self.match_next('=') {
                    self.token(TokenKind::BitwiseXorAssignment)
                }
                else {
                    self.token(TokenKind::BitwiseXor)
                }
            },

            '=' => {
                if self.match_next('=') {
                    self.token(TokenKind::Equal)
                }
                else {
                    self.token(TokenKind::Assignment)
                }
            },

            '!' => {
                if self.match_next('=') {
                    self.token(TokenKind::NotEqual)
                }
                else {
                    self.token(TokenKind::Not)
                }
            },

            '<' => {
                if self.match_next('<') {
                    if self.match_next('=') {
                        self.token(TokenKind::BitwiseLShiftAssignment)
                    }
                    else {
                        self.token(TokenKind::BitwiseLShift)
                    }
                }
                else if self.match_next('=') {
                    self.token(TokenKind::LessThanOrEqual)
                }
                else {
                    self.token(TokenKind::LessThan)
                }
            },

            '>' => {
                if self.match_next('>') {
                    if self.match_next('=') {
                        self.token(TokenKind::BitwiseRShiftAssignment)
                    }
                    else {
                        self.token(TokenKind::BitwiseRShift)
                    }
                }
                else if self.match_next('=') {
                    self.token(TokenKind::GreaterThanOrEqual)
                }
                else {
                    self.token(TokenKind::GreaterThan)
                }
            },

            '(' => self.token(TokenKind::LeftParen),
            ')' => self.token(TokenKind::RightParen),
            '[' => self.token(TokenKind::LeftBracket),
            ']' => self.token(TokenKind::RightBracket),
            '{' => self.token(TokenKind::LeftBrace),
            '}' => self.token(TokenKind::RightBrace),
            ',' => self.token(TokenKind::Comma),
            '.' => self.token(TokenKind::Dot),
            ';' => self.token(TokenKind::Semicolon),

            '\'' => self.parse_char(),
            '\"' => self.parse_string(),
            '#' => self.parse_comment(),

            _ => self.error(ErrorCode::ET004)
        }
    }

    fn parse_char(&mut self) -> Result<Option<Token<'a>>, TokenizerError<'a>> {
        let mut chars_count: u8 = 0;
        let mut is_escaping_single_quote: bool = false;

        while let Some(next) = self.peek() {
            if next == '\''{
                self.next().unwrap();

                if chars_count > 1 {
                    return self.error(ErrorCode::ET005);
                }
                else if chars_count == 0 {
                    return self.error(ErrorCode::ET006);
                }

                is_escaping_single_quote = false;
                break;
            }

            self.next().unwrap();

            if next == '\\' {
                is_escaping_single_quote = self.parse_inner_char()?;
            } 

            chars_count += 1;
        }

        let literal: &str = &self.source[self.start..self.end];

        if !literal.ends_with('\'') || literal.len() == 1  {
            return self.error(ErrorCode::ET007);
        }
        else if is_escaping_single_quote {
            return self.error(ErrorCode::ET008);
        }

        self.token(TokenKind::CharLiteral)
    }

    fn parse_inner_char(&mut self) -> Result<bool, TokenizerError<'a>> {
        if self.character == '\\' {
            return self.parse_escape_sequence();
        }

        self.next().unwrap();
        return Ok(false)
    }

    fn parse_escape_sequence(&mut self) -> Result<bool, TokenizerError<'a>> {
        if let Some(next) = self.peek() {
            self.next().unwrap();

            match next {
                'a' | 'b' | 'f' | 'n' | 
                'r' | 't' | 'v' | '\\' | 
                '\'' | '\"' | '?' | '0' => {
                    if next == '\'' {
                        return Ok(true);
                    }
                },
                
                '1'..='7' => self.parse_octal()?,

                'x' => self.parse_hex()?,

                _ => return self.error(ErrorCode::ET009),
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

    fn parse_octal(&mut self) -> Result<(), TokenizerError<'a>> {
        let mut is_invalid: bool = false;
        let mut digit_count: u8 = 1; 

        while let Some(next) = self.peek() {
            if next == '\'' || (self.is_parsing_string_literal && next == '\"') {
                if is_invalid || digit_count > 3 {
                    return self.error(ErrorCode::ET010);
                }
                break;
            }

            if !self.is_ascii_octal(next).is_some() {
                is_invalid = true;
            }

            self.next().unwrap();
            digit_count += 1;
        }
        
        Ok(())
    }

    fn parse_hex(&mut self) -> Result<(), TokenizerError<'a>> {
        let mut is_invalid: bool = false;
        let mut digit_count: u8 = 0;
        
        while let Some(next) = self.peek() {
            if next == '\'' || (self.is_parsing_string_literal && next == '\"') {
                if digit_count > 2 || digit_count == 0 || is_invalid {
                    return self.error(ErrorCode::ET011);
                }
                break;
            }

            if !next.is_ascii_hexdigit() {
                if next == '\\' && digit_count == 2 {
                    break;
                }
                is_invalid = true;
            }

            self.next().unwrap();
            digit_count += 1;
        }

        Ok(())
    }

    fn parse_string(&mut self) -> Result<Option<Token<'a>>, TokenizerError<'a>> {
        self.is_parsing_string_literal = true;

        while let Some(next) = self.peek() {
            if next == '\"' {
                self.next().unwrap();
                break;
            }

            self.next().unwrap();

            if next == '\\' {
                self.parse_inner_char()?; 
            }
        }

        let literal: &str = &self.source[self.start..self.end];

        if !literal.ends_with('\"') || literal.len() == 1 {
            return self.error(ErrorCode::ET012);
        }

        self.is_parsing_string_literal = false;

        self.token(TokenKind::StringLiteral)
    }

    fn parse_comment(&mut self) -> Result<Option<Token<'a>>, TokenizerError<'a>> {
        while let Some(next) = self.peek() {
            if next == '\n' {
                break;
            }
            self.next().unwrap();
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
                self.end += 1;
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

    fn token(&self, kind: TokenKind) -> Result<Option<Token<'a>>, TokenizerError<'a>> {
        Ok(Some(Token {
            kind,
            span: TokenSpan::new(self.start, self.end, &self.source[self.start..self.end]),
        }))
    }

    fn error<T>(&self, code: ErrorCode) -> Result<T, TokenizerError<'a>> {
        Err(TokenizerError {
            code,
            span: TokenSpan::new(self.start, self.end, &self.source[self.start..self.end]),
            line: self.line,
            column: self.column,
        })
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.next();
            return true
        } 
        else {
            return false;
        }
    }
}