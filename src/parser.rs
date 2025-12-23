use crate::tokenizer::{Token, TokenKind, TokenSpan};
use crate::error::*;

use core::panic;

#[derive(Debug, Clone, Copy)]
pub struct StatementSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: StatementSpan,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: StatementSpan) -> Self {
        Self { node, span }
    }
}

impl<T> std::fmt::Display for Spanned<T>
where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.node)
    }
}

// Type aliases now carry the lifetime 'a
type Expression<'a> = Box<Spanned<RawExpression<'a>>>;
pub type Statement<'a> = Box<Spanned<RawStatement<'a>>>;
#[derive(Debug, Clone)]
pub struct Type<'a> {
    pub kind: TokenKind,
    pub is_array: bool,
    pub array_length: Option<Expression<'a>>,
}

impl<'a> Type<'a> {
    pub fn new(kind: TokenKind, is_array: bool, array_length: Option<Expression<'a>>) -> Self {
        Self { kind, is_array, array_length }
    }

    pub fn is(kind: TokenKind) -> bool {
        use TokenKind::*;
        matches!(kind, 
            SignedInt8 | SignedInt16 | SignedInt32 | SignedInt64 |
            UnsignedInt8 | UnsignedInt16 | UnsignedInt32 | UnsignedInt64 |
            Float32 | Float64 | Character | String | Boolean
        )
    }
}

impl<'a> std::fmt::Display for Type<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_array {
            write!(f, 
                "Type {{\n    kind: {:?},\n    is_array: {},\n    array_length: {}\n}}", 
                self.kind, self.is_array, self.array_length.as_ref().unwrap()
            )
        } 
        else {
            // Concise single-line output for non-arrays
            write!(f, "Type {{\n    kind: {}\n}}", self.kind)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parameter<'a> {
    pub name: &'a str,
    pub ty: Type<'a>,
}

#[derive(Debug, Clone)]
pub struct Block<'a> {
    pub statements: Vec<Statement<'a>>,
    pub span: StatementSpan,
}

#[derive(Debug, Clone)]
pub enum RawExpression<'a> {
    Placeholder,
    Variable(&'a str),
    Literal {
        kind: TokenKind,
        value: &'a str,
    },
    Binary {
        left: Expression<'a>,
        operator: TokenKind,
        right: Expression<'a>,
    },
    Unary {
        operator: TokenKind,
        operand: Expression<'a>,
    },
    FunctionCall {
        name: &'a str,
        arguments: Vec<Expression<'a>>,
    },
    ArrayAccess {
        array: Expression<'a>,
        index: Expression<'a>,
    },
}

impl<'a> RawExpression<'a> {
    pub fn is(kind: TokenKind) -> bool {
        use TokenKind::*;
        matches!(kind,
            IntegerLiteral | FloatLiteral | CharLiteral | StringLiteral |
            Plus | Minus | Multiplication | Division | Modulus | 
            BitwiseAnd | BitwiseOr | BitwiseXor | BitwiseLShift | BitwiseRShift |
            LeftParen | RightParen | True | False | Identifier
        )
    }
    
    pub fn is_start(kind: TokenKind) -> bool {
        use TokenKind::*;
        matches!(kind,
            IntegerLiteral | FloatLiteral | CharLiteral | StringLiteral |
            LeftParen | RightParen | True | False | Identifier
        )
    }
}

impl<'a>  std::fmt::Display for RawExpression<'a>  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Placeholder => write!(f, "Expression"),
            _ => write!(f, "Expression are not yet implemented"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RawStatement<'a> {
    Placeholder,
    VariableDeclaration {
        is_const: bool,
        type_: Type<'a>,
        name: &'a str,
        value: Option<Expression<'a>>,
    },
    VariableAssignment {
        name: &'a str,
        operator: TokenKind,
        value: Expression<'a>,
    },
    If {
        condition: Expression<'a>,
        then_branch: Block<'a>,
        else_branch: Option<ElseBranch<'a>>,
    },
    While {
        condition: Expression<'a>,
        body: Block<'a>,
    },
    Break,
    Continue,
    FunctionCall(Expression<'a>),
    Return(Option<Expression<'a>>),
}

impl<'a> std::fmt::Display for RawStatement<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawStatement::Placeholder => write!(f, "Placeholder"),
            RawStatement::VariableDeclaration { is_const, type_, name, value } => {
                write!(f, 
                    "VariableDeclaration {{\n    is_const: {},\n    type: {},\n    identifier: {},\n    value: {:?}\n}}", 
                    is_const, type_, name, if let Some(value) = value { format!("{}", value) } else { "None".to_string() }
                )
            }
            // Add other variants as you implement them...
            _ => write!(f, "{:?}", self), // Fallback to Debug for now
        }
    }
}

#[derive(Debug, Clone)]
pub enum ElseBranch<'a> {
    ElseIf(Statement<'a>),
    Else(Block<'a>),
}

pub trait TokenMatcher {
    fn matches(&self, kind: TokenKind) -> bool;
}

impl TokenMatcher for TokenKind {
    fn matches(&self, kind: TokenKind) -> bool {
        *self == kind
    }
}

impl<F> TokenMatcher for F 
where F: Fn(TokenKind) -> bool {
    fn matches(&self, kind: TokenKind) -> bool {
        self(kind)
    }
}

pub struct ParserError {
    pub code: ErrorCode,
    pub span: StatementSpan,
}

impl ParserError {
    pub fn to_diagnostic<'a>(&'a self, filename: &'a str) -> Diagnostic<'a> {
        Diagnostic { 
            kind: DiagnosticKind::Error(self.code), 
            info: DiagnosticInfo { filename, line: 0, column: 0 }, 
            hint: None 
        }
    }
}

pub struct Parser<'a> {
    tokens: std::iter::Peekable<std::vec::IntoIter<Token<'a>>>,
    next: Token<'a>,
    peeked: Token<'a>,
    // Tracking current span state
    start: usize,
    end: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {        
        Self {
            tokens: tokens.into_iter().peekable(),
            next: Token{ 
                kind: TokenKind::Eof, 
                span: TokenSpan { start: 0, end: 0, literal: "" } 
            },
            peeked: Token{ 
                kind: TokenKind::Eof, 
                span: TokenSpan { start: 0, end: 0, literal: "" } 
            },
            start: 0,
            end: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement<'a>>, ParserError> {
        let mut statements: Vec<Statement<'a>> = Vec::new();

        while let Some(_) = self.peek() {
            if self.peeked.kind == TokenKind::Eof {
                break;
            }

            statements.push(self.get_statement()?);
        }

        Ok(statements)
    }

    fn get_statement(&mut self) -> Result<Statement<'a>, ParserError> {
        if self.is_variable() {
            self.parse_variable()
        }
        else {
            panic!("Not implemented");  
        }
    }

    fn is_variable(&mut self) -> bool {
        self.match_peek(Type::is) || self.match_peek(TokenKind::Const)
    }

    fn parse_variable(&mut self) -> Result<Statement<'a>, ParserError> {
        let mut is_const: bool = false;
        if self.peeked.kind == TokenKind::Const {
            self.next();
            is_const = true;
        }

        let type_: Type = self.parse_type()?;
        let name: &str = self.expect(TokenKind::Identifier, ErrorCode::EP003)?.span.literal;
        let mut value: Option<Expression<'a>> = None;

        if self.match_peek(TokenKind::Semicolon) {
            self.next();
            Ok(self.statement(RawStatement::VariableDeclaration { is_const, type_, name, value}))
        }
        else if self.match_peek(TokenKind::Assignment)  {
            self.next();

            self.expect(RawExpression::is_start, ErrorCode::EP004)?;
            value = Some(self.parse_expression()?);

            self.expect(TokenKind::Semicolon, ErrorCode::EP005)?;
            Ok(self.statement(RawStatement::VariableDeclaration { is_const, type_, name, value}))
        }
        else {
            Err(self.error(ErrorCode::EP006))
        }
    }

    fn parse_type(&mut self) -> Result<Type<'a>, ParserError> {
        let kind: TokenKind  = self.next().kind;
        let mut array_length: Option<Expression<'a>> = None;
        let mut is_array: bool = false;

        // Check for a left bracket '['
        if self.match_next(TokenKind::LeftBracket) {
            // Check if next character is the start of an expression
            self.expect(RawExpression::is_start, ErrorCode::EP001)?;
            array_length = Some(self.parse_expression()?);
            
            // Chech for a right bracket ']'
            self.expect(TokenKind::RightBracket, ErrorCode::EP002)?;
            is_array = true;
        }

        Ok(Type::new(kind, is_array, array_length))
    }

    // Note:
    // I will create expression later on because they are the hardest part,
    // Doing it later allows me to get more stuff done quicker
    fn parse_expression(&mut self) -> Result<Expression<'a>, ParserError> {
        Ok(self.expression(RawExpression::Placeholder))
    }


    fn statement(&mut self, node: RawStatement<'a>) -> Statement<'a> {
        Box::new(Spanned { 
            node, 
            span: StatementSpan { 
                start: self.start, 
                end: self.end 
            } 
        })
    }

    fn expression(&mut self, node: RawExpression<'a>) -> Expression<'a> {
        Box::new(Spanned { 
            node, 
            span: StatementSpan { 
                start: self.start, 
                end: self.end 
            } 
        })
    }

    fn error(&self, code: ErrorCode) -> ParserError {
        ParserError { 
            code, 
            span: StatementSpan { 
                start: self.start, 
                end: self.end 
            }
        }
    }


    fn next(&mut self) -> Token<'a> {
        match self.tokens.next() {
            Some(token) => {
                self.end += 1;
                token
            }
            None => {
                panic!("Compiler Error! Before each call to next(), call peek()");
            }
        }
    }

    fn peek(&mut self) -> Option<Token<'a>> { 
        match self.tokens.peek() {
            Some(&token) => {
                self.peeked = token;
                Some(self.peeked)
            },
            None => None
        }
    }

    pub fn match_next<M: TokenMatcher>(&mut self, matcher: M) -> bool {
        if let Some(token) = self.peek() {
            if matcher.matches(token.kind) {
                self.next();
                return true;
            }
        }
        return false;
    }

    pub fn match_peek<M: TokenMatcher>(&mut self, matcher: M) -> bool {
        if let Some(token) = self.peek() {
            if matcher.matches(token.kind) {
                return true;
            }
        }
        return false;
    }

    fn expect<M: TokenMatcher>(&mut self, matcher: M, error: ErrorCode) -> Result<Token<'a>, ParserError> {
        if let Some(token) = self.peek() {
            if matcher.matches(token.kind) {
                return Ok(self.next());
            }
        }
        Err(self.error(error))
    }

}