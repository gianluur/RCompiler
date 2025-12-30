use crate::tokenizer::{Token, TokenKind, TokenSpan};
use crate::error::*;

use core::panic;

#[derive(Debug, Clone, Copy)]
pub struct StatementSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: StatementSpan,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.node.fmt(f)
    }
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: StatementSpan) -> Self {
        Self { node, span }
    }
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
        body: Body<'a>,
        elses: Vec<ElseBranch<'a>>,
    },
    While {
        condition: Expression<'a>,
        body: Body<'a>,
    },
    Function {
      name: &'a str,
      parameters: Vec<Parameter<'a>>,
      type_: TokenKind,
      body: Body<'a>
    },
    LoopControl(&'a str),
    FunctionCall(Expression<'a>),
    Return(Option<Expression<'a>>),
}

#[derive(Debug, Clone)]
pub enum ElseBranch<'a> {
    ElseIf(Statement<'a>),
    Else(Body<'a>),
}

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
            Float32 | Float64 | Character | String | Boolean | Const
        )
    }
}

#[derive(Debug, Clone)]
pub struct Parameter<'a> {
    pub name: &'a str,
    pub type_: Type<'a>,
}

#[derive(Debug, Clone)]
pub struct Body<'a> {
    pub statements: Vec<Statement<'a>>,
}

pub type Expression<'a> = Box<Spanned<RawExpression<'a>>>;
pub type Statement<'a> = Box<Spanned<RawStatement<'a>>>;

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

#[derive(Debug)]
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

    // context related stuff
    inside_if_statement: bool,
    inside_elif_statement: bool,
    inside_while_loop: bool,
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

            inside_if_statement: false,
            inside_elif_statement: false,
            inside_while_loop: false,
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

    // TODO: I can also add after i've done the major of parsing
    // in the else branch i can add checks for like else if, or elses 
    // that aren't followed by the if staements or staements like that
    // So i can get better errors
    fn get_statement(&mut self) -> Result<Statement<'a>, ParserError> {
        if self.is_variable() {
            self.parse_variable()
        }

        else if self.match_peek(TokenKind::Identifier) {
            self.parse_identifier()
        }

        else if self.match_peek(TokenKind::If) {
            self.parse_if_statement()
        }

        else if self.match_peek(TokenKind::While) {
            self.parse_while_statement()
        }

        else if self.is_loop_control() {
            self.parse_loop_control()
        }

        else if self.match_peek(TokenKind::Function) {
            self.parse_function()
        }

        else if self.match_peek(TokenKind::Return) {
            self.parse_return_statement()
        }

        else {
            panic!("Not implemented {}", self.peeked.kind);  
        }
    }

    fn is_variable(&mut self) -> bool {
        self.match_peek(Type::is) || self.match_peek(TokenKind::Const)
    }

    fn is_assignment() -> impl Fn(TokenKind) -> bool {
        |kind: TokenKind| matches!(kind, 
            TokenKind::Assignment | 
            TokenKind::AddAssignment | TokenKind::SubtractAssignment |
            TokenKind::MultiplyAssignment | TokenKind::DivideAssignment | 
            TokenKind::ModulusAssignment | TokenKind::BitwiseAndAssignment |
            TokenKind::BitwiseOrAssignment | TokenKind::BitwiseXorAssignment |
            TokenKind::BitwiseRShiftAssignment | TokenKind::BitwiseLShiftAssignment
        )
    }

    fn is_loop_control(&mut self) -> bool {
        return self.match_peek(TokenKind::Break) ||
               self.match_peek(TokenKind::Continue);
    }

    fn parse_type(&mut self) -> Result<Type<'a>, ParserError> {
        let kind: TokenKind  = self.next().kind;
        let mut array_length: Option<Expression<'a>> = None;
        let mut is_array: bool = false;

        // Check for a left bracket '['
        if self.match_peek(TokenKind::LeftBracket) {
            self.next();

            // Check if next character is the start of an expression
            self.expect_peek(RawExpression::is_start, ErrorCode::EP001)?;
            array_length = Some(self.parse_expression()?);
            
            // Chech for a right bracket ']'
            self.expect_next(TokenKind::RightBracket, ErrorCode::EP002)?;
            is_array = true;
        }

        Ok(Type::new(kind, is_array, array_length))
    }

    fn parse_variable(&mut self) -> Result<Statement<'a>, ParserError> {
        let mut is_const: bool = false;
        if self.peeked.kind == TokenKind::Const {
            self.next();
            is_const = true;
        }

        let type_: Type = self.parse_type()?;
        let name: &str = self.expect_next(TokenKind::Identifier, ErrorCode::EP003)?.span.literal;
        let mut value: Option<Expression<'a>> = None;

        if self.match_peek(TokenKind::Semicolon) {
            self.next();

            Ok(self.statement(RawStatement::VariableDeclaration { is_const, type_, name, value}))
        }
        else if self.match_peek(TokenKind::Assignment)  {
            self.next();

            self.expect_peek(RawExpression::is_start, ErrorCode::EP004)?;
            value = Some(self.parse_expression()?);

            self.expect_next(TokenKind::Semicolon, ErrorCode::EP005)?;
            Ok(self.statement(RawStatement::VariableDeclaration { is_const, type_, name, value}))
        }
        else {
            Err(self.error(ErrorCode::EP006))
        }
    }

    fn parse_if_statement(&mut self) -> Result<Statement<'a>, ParserError> {
        self.inside_if_statement = true;

        let condition: Expression<'a>;
        let body: Body<'a>;
        let mut elses: Vec<ElseBranch> = Vec::new();

        self.next(); // Consumes the 'if' keyword or 'elif' keyword

        self.expect_peek(RawExpression::is_start, ErrorCode::EP007)?;
        condition = self.parse_expression()?;
        
        self.expect_peek(TokenKind::LeftBrace, ErrorCode::EP008)?;
        body = self.parse_body()?;

        if !self.inside_elif_statement {
            while let Some(_) = self.peek() {
                if self.peeked.kind == TokenKind::ElseIf {
                    self.inside_elif_statement = true;
                    elses.push(ElseBranch::ElseIf(self.parse_if_statement()?));
                    self.inside_elif_statement = false;
                }
                else if self.peeked.kind == TokenKind::Else {
                    self.next(); // Parses the 'else' keyword
                    self.expect_peek(TokenKind::LeftBrace, ErrorCode::EP009)?;

                    elses.push(ElseBranch::Else(self.parse_body()?));
                }
                else {
                    break;
                }
            }
            self.inside_if_statement = false;
        }

        Ok(self.statement(RawStatement::If { condition, body, elses }))

    }

    fn parse_body(&mut self) -> Result<Body<'a>, ParserError> {
        self.next(); // Consumes '{'

        let mut statements: Vec<Statement<'a>> = Vec::new();

        while let Some(_) = self.peek() {
            if self.peeked.kind == TokenKind::Eof {
                return Err(self.error(ErrorCode::EP010));
            }

            if self.peeked.kind == TokenKind::RightBrace {
                self.next(); // Consumes the '}'
                // Note:
                // There's no point in updating the is_match variable
                // Becuase it will never be used again.
                // The check is performed inside the if statement that
                // stops the code execution of this function so there's no
                // risk of accidental use
                break;
            }

            statements.push(self.get_statement()?);
        }

        Ok(Body {
            statements,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Statement<'a>, ParserError> {
        self.inside_while_loop = true;
        self.next(); // Consumes the 'while' keyword

        let condition: Expression<'a>;
        let body: Body<'a>;

        self.expect_peek(RawExpression::is_start, ErrorCode::EP011)?;
        condition = self.parse_expression()?;
        
        self.expect_peek(TokenKind::LeftBrace, ErrorCode::EP012)?;
        body = self.parse_body()?;

        self.inside_while_loop = false;

        Ok(self.statement(RawStatement::While { condition, body }))
    }

    fn parse_identifier(&mut self) -> Result<Statement<'a>, ParserError> {
        let identifier: Token<'a> = self.next();

        if self.match_peek(TokenKind::LeftParen){
            self.parse_function_call(identifier)
        }
        else if self.match_peek(Parser::is_assignment()) {
            self.parse_variable_assignment(identifier)
        }
        else {
            Err(self.error(ErrorCode::EP013))
        }
    }

    fn parse_function_call(&mut self, name: Token<'a>) -> Result<Statement<'a>, ParserError> {
        self.next(); // consumes the '('

        let arguments: Vec<Expression> = self.parse_arguments()?;

        self.expect_next(TokenKind::Semicolon, ErrorCode::EP017)?;

        // I have to do this for the borrow checker, i didn't really understand why
        // I can't put it all in one statement
        let fncall: Expression = self.expression(RawExpression::FunctionCall { 
            name: name.span.literal,
            arguments 
        });

        Ok(self.statement(RawStatement::FunctionCall(
            fncall
        )))

    }

    fn parse_arguments(&mut self) -> Result<Vec<Expression<'a>>, ParserError> {
        let mut arguments: Vec<Expression> = Vec::new();
        while let Some(_) = self.peek() {
            if self.peeked.kind == TokenKind::Eof {
                return Err(self.error(ErrorCode::EP014));
            }

            if self.peeked.kind == TokenKind::RightParen {
                self.next();
                break;
            }

            self.expect_peek(RawExpression::is_start, ErrorCode::EP015)?;
            arguments.push(self.parse_expression()?);

            if self.match_peek(TokenKind::Comma){
                self.next();
                if self.match_peek(TokenKind::RightParen) {
                    return Err(self.error(ErrorCode::EP016));
                }
            }            
        }

        Ok(arguments)
    }

    fn parse_variable_assignment(&mut self, name: Token<'a>) -> Result<Statement<'a>, ParserError> {
        let operator: TokenKind = self.next().kind;
        
        self.expect_peek(RawExpression::is_start, ErrorCode::EP018)?;
        let value: Expression<'a> = self.parse_expression()?;
        
        self.expect_next(TokenKind::Semicolon, ErrorCode::EP019)?;
        
        Ok(self.statement(RawStatement::VariableAssignment {
            name: name.span.literal, 
            operator, 
            value 
        }))
    }

    fn parse_loop_control(&mut self) -> Result<Statement<'a>, ParserError> {
        let keyword: Token<'a> = self.next(); // Parses either 'break' or 'continue'
        self.expect_next(TokenKind::Semicolon, ErrorCode::EP020)?;

        Ok(self.statement(RawStatement::LoopControl(keyword.span.literal)))
    }

    fn parse_return_statement(&mut self) -> Result<Statement<'a>, ParserError> {
        self.next(); // Parses the 'return' keyword
        
        let mut expression: Option<Expression<'a>> = None;
        if self.match_peek(TokenKind::Semicolon) {
            self.next();
        }
        else if self.match_peek(RawExpression::is_start) {
            expression = Some(self.parse_expression()?);
            self.expect_next(TokenKind::Semicolon, ErrorCode::EP022)?;
        }
        else {
            return Err(self.error(ErrorCode::EP021));
        }

        Ok(self.statement(RawStatement::Return(expression)))
    }

    fn parse_function(&mut self) -> Result<Statement<'a>, ParserError> {
        self.next(); // Consumes the 'fn' keyword
        
        let name: &'a str = self.expect_next(TokenKind::Identifier, 
                    ErrorCode::EP023)?.span.literal;
        
        self.expect_next(TokenKind::LeftParen, ErrorCode::EP024)?;
        let parameters: Vec<Parameter> = self.parse_parameters()?;
        
        let mut type_: TokenKind = TokenKind::Null;
        // if self.match_peek(Type::is) {
        //     type_ = self.parse_type()?.kind;
        // }

        if !self.match_peek(TokenKind::LeftBrace) && 
            self.match_peek(Type::is) {
            type_ = self.parse_type()?.kind;
        }
        else {
            return Err(self.error(ErrorCode::EP025));
        }

        self.expect_peek(TokenKind::LeftBrace, ErrorCode::EP025)?;
        let body: Body<'a> = self.parse_body()?;
        
        Ok(self.statement(RawStatement::Function { 
            name, 
            parameters, 
            type_, 
            body 
        }))
    }

    fn parse_parameters(&mut self) -> Result<Vec<Parameter<'a>>, ParserError> {
        let mut parameters: Vec<Parameter> = Vec::new();
        while let Some(_) = self.peek() {
            if self.peeked.kind == TokenKind::Eof {
                return Err(self.error(ErrorCode::EP026));
            }

            if self.peeked.kind == TokenKind::RightParen {
                self.next();
                break;
            }

            self.expect_peek(Type::is, ErrorCode::EP027)?;
            let type_: Type<'a> = self.parse_type()?;

            let name: &'a str = self.expect_next(TokenKind::Identifier, 
                        ErrorCode::EP028)?.span.literal;

            parameters.push(Parameter {
                name, 
                type_ 
            });

            if self.match_peek(TokenKind::Comma){
                self.next();
                if !self.match_peek(Type::is) {
                    return Err(self.error(ErrorCode::EP029));
                }
            }            
        }

        Ok(parameters)
    }

    fn parse_expression(&mut self) -> Result<Expression<'a>, ParserError> {
        self.next();
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
                self.start = self.end;
                self.end += token.span.literal.len();
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

    pub fn match_peek<M: TokenMatcher>(&mut self, matcher: M) -> bool {
        if let Some(token) = self.peek() {
            if matcher.matches(token.kind) {
                return true;
            }
        }
        return false;
    }

    fn expect_next<M: TokenMatcher>(&mut self, matcher: M, error: ErrorCode) -> Result<Token<'a>, ParserError> {
        if let Some(token) = self.peek() {
            if matcher.matches(token.kind) {
                return Ok(self.next());
            }
        }
        Err(self.error(error))
    }

    fn expect_peek<M: TokenMatcher>(&mut self, matcher: M, error: ErrorCode) -> Result<Token<'a>, ParserError> {
        if let Some(token) = self.peek() {
            if matcher.matches(token.kind) {
                return Ok(token);
            }
        }
        Err(self.error(error))
    }

}