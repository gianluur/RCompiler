use crate::tokenizer::{Token, TokenKind, TokenSpan};
use crate::ast::*;
use crate::error::*;
use crate::interner::*;

use core::panic;

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
    pub line: usize,
    pub column: usize,
}

impl ParserError {
    pub fn to_diagnostic<'a>(&'a self, filename: &'a str) -> Diagnostic<'a> {
        Diagnostic { 
            kind: DiagnosticKind::Error(self.code), 
            info: DiagnosticInfo { filename, line: self.line, column: self.column }, 
            hint: Some(self.get_hint(self.code)) 
        }
    }

    fn get_hint(&self, code: ErrorCode) -> &str {
        match code {
            ErrorCode::EP000 => "an 'else' or 'elif' must be preceded by an 'if' block",
            ErrorCode::EP001 => "arrays require a fixed size, e.g., i32[10]",
            ErrorCode::EP002 => "close the array size declaration with ']'",
            ErrorCode::EP003 => "provide a name for your variable after the type",
            ErrorCode::EP004 => "assign a value or expression to the variable",
            ErrorCode::EP005 => "terminate the variable declaration with a ';'",
            ErrorCode::EP006 => "complete the variable declaration with an assignment (=) or a semicolon (;)",
            ErrorCode::EP007 => "add a boolean expression in parentheses or after the 'if' keyword",
            ErrorCode::EP008 => "start the 'if' body with an opening brace '{'",
            ErrorCode::EP009 => "start the 'else' body with an opening brace '{'",
            ErrorCode::EP010 => "add a closing brace '}' to end the code block",
            ErrorCode::EP011 => "the 'while' loop requires a condition to evaluate",
            ErrorCode::EP012 => "start the 'while' loop body with an opening brace '{'",
            ErrorCode::EP013 => "identifiers must be followed by '(' for calls or '=' for assignments",
            ErrorCode::EP014 => "close the function call arguments with ')'",
            ErrorCode::EP015 => "provide at least one value or variable as an argument",
            ErrorCode::EP016 => "finish the argument list or add a closing ')'",
            ErrorCode::EP017 => "add a semicolon ';' after the function call",
            ErrorCode::EP018 => "provide a value to be assigned to the variable",
            ErrorCode::EP019 => "add a semicolon ';' to end the assignment statement",
            ErrorCode::EP020 => "keywords like 'break' and 'continue' must be followed by a ';'",
            ErrorCode::EP021 => "provide a value to return or a ';' for void functions",
            ErrorCode::EP022 => "add a semicolon ';' to end the return statement",
            ErrorCode::EP023 => "give your function a name, e.g., fn my_function()",
            ErrorCode::EP024 => "add '(' after the function name to define parameters",
            ErrorCode::EP025 => "specify a return type (like i32) or leave blank for void",
            ErrorCode::EP026 => "start the function body with an opening brace '{'",
            ErrorCode::EP027 => "close the parameter list with ')'",
            ErrorCode::EP028 => "parameters must have a type, e.g., fn(i32 x)",
            ErrorCode::EP029 => "provide a name for the parameter after its type",
            ErrorCode::EP030 => "separate parameters with a comma or close with ')'",
            ErrorCode::EP031 => "close the grouped expression with ')'",
            ErrorCode::EP032 => "check your syntax; an expression cannot start with this token",
            ErrorCode::EP033 => "close the array index access with ']'",
            ErrorCode::EP034 => "only functions can be called; ensure the identifier is a valid function name",
            _ => "",
        }
    }
}

pub struct Parser {
    tokens: std::iter::Peekable<std::vec::IntoIter<Token>>,
    peeked: Token,

    // Tracking current stament span state
    statement_start: usize,
    statement_end: usize,

    // we don't have here the expression_start becuase of the
    // recursive nature of the parse_expression function
    expression_end: usize,

    line: usize,
    column: usize,

    // context related stuff
    inside_if: bool,
    inside_elif: bool,
    inside_while: bool,
    inside_function: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {        
        Self {
            tokens: tokens.into_iter().peekable(),
            peeked: Token{ 
                kind: TokenKind::Eof, 
                span: TokenSpan { 
                    start: 0, 
                    end: 0, 
                    literal: SourceLiteral::dummy(), 
                    line: 0, 
                    column: 0 
                } 
            },

            statement_start: 0,
            statement_end: 0,
            expression_end: 0,

            line: 1,
            column: 1,

            inside_if: false,
            inside_elif: false,
            inside_while: false,
            inside_function: false,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut statements: Vec<Statement> = Vec::new();

        while let Some(_) = self.peek() {
            if self.peeked.kind == TokenKind::Eof {
                break;
            }
            
            statements.push(self.get_statement()?);
        }

        Ok(statements)
    }

    fn get_statement(&mut self) -> Result<Statement, ParserError> {
        self.statement_start = self.peeked.span.start;

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
            if self.match_peek(TokenKind::ElseIf) ||
               self.match_peek(TokenKind::Else) {
                return Err(self.error(ErrorCode::EP000));
            }

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

    fn is_const(&mut self) -> bool {
        let mut is_const: bool = false; 
        if self.peeked.kind == TokenKind::Const {
            self.next();
            is_const = true;
        }

        is_const
    }

    fn parse_type(&mut self) -> Result<Type, ParserError> {
        let kind: TokenKind  = self.next().kind; // Consumes the kind like 'i32'
        let mut array_length: Option<Expression> = None; // Stores the array size
        let mut is_array: bool = false; // Check if the type is an array type

        // Check for a left bracket '['
        if self.match_peek(TokenKind::LeftBracket) {
            self.next();

            // Check if next character is the start of an expression
            self.expect_peek(RawExpression::is, ErrorCode::EP001)?;
            array_length = Some(self.parse_expression(0)?);
            
            // Chech for a right bracket ']'
            self.expect_next(TokenKind::RightBracket, ErrorCode::EP002)?;
            is_array = true;
        }

        Ok(Type { 
            kind, 
            is_array, 
            array_length 
        })
    }

    fn parse_variable(&mut self) -> Result<Statement, ParserError> {       
        // Check if the variable is const 
        let is_const: bool = self.is_const();
        let type_: Type = self.parse_type()?;
        let name: SourceLiteral  = self.expect_next(TokenKind::Identifier, 
            ErrorCode::EP003)?.span.literal;
        let mut value: Option<Expression> = None;

        // This is a declaration without an initial value
        if self.match_peek(TokenKind::Semicolon) {
            self.next();

            Ok(self.statement(
                RawStatement::VariableDeclaration { 
                    is_const, 
                    type_, 
                    name, 
                    value
                }
            ))
        }
        // This is a declaration with an initial value
        else if self.match_peek(TokenKind::Assignment)  {
            self.next();

            self.expect_peek(RawExpression::is, ErrorCode::EP004)?;
            value = Some(self.parse_expression(0)?);

            self.expect_next(TokenKind::Semicolon, ErrorCode::EP005)?;

            Ok(self.statement(
                RawStatement::VariableDeclaration { 
                    is_const, 
                    type_, 
                    name, 
                    value
                }
            ))
        }
        // The user inserted some unexpected token after the type and name
        else {
            Err(self.error(ErrorCode::EP006))
        }
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ParserError> {
        // This checks and the inside_elif check serves to prevent infinite loops
        // and to provide better errors
        self.inside_if = true; 

        let condition: Expression;
        let body: Body;
        let mut elses: Vec<ElseBranch> = Vec::new();

        self.next(); // Consumes the 'if' keyword or 'elif' keyword

        // Consumes the condition
        self.expect_peek(RawExpression::is, ErrorCode::EP007)?;
        condition = self.parse_expression(0)?;
        
        // Consumes the body
        self.expect_peek(TokenKind::LeftBrace, ErrorCode::EP008)?;
        body = self.parse_body()?;

        // If we are not inside an elif statement we can start parsing all the
        // other 'elif' and 'else' statements
        if !self.inside_elif {
            while let Some(_) = self.peek() {
                if self.peeked.kind == TokenKind::ElseIf {
                    self.inside_elif = true;
                    elses.push(ElseBranch::ElseIf(self.parse_if_statement()?));
                    self.inside_elif = false;
                }
                else if self.peeked.kind == TokenKind::Else {
                    self.next(); // Parses the 'else' keyword
                    self.expect_peek(TokenKind::LeftBrace, ErrorCode::EP009)?;

                    elses.push(ElseBranch::Else(self.parse_body()?));
                }
                // If it's not an elif or else then it's something else and we can stop
                else {
                    break;
                }
            }
        }
        self.inside_if = false;

        Ok(self.statement(RawStatement::If { condition, body, elses }))

    }

    fn parse_body(&mut self) -> Result<Body, ParserError> {
        self.next(); // Consumes '{'

        let mut statements: Vec<Statement> = Vec::new();
        
        // Consumes all the various statements that are inside the body
        while let Some(_) = self.peek() {
            // This here means an incomplete body, because it's missing the '}'
            if self.peeked.kind == TokenKind::Eof {
                return Err(self.error(ErrorCode::EP010));
            }

            // Found the '}' we can quit after we consume it
            if self.peeked.kind == TokenKind::RightBrace {
                self.next(); // Consumes the '}'
                break;
            }

            statements.push(self.get_statement()?);
        }

        Ok(Body {
            statements,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Statement, ParserError> {
        self.inside_while = true;
        self.next(); // Consumes the 'while' keyword

        let condition: Expression;
        let body: Body;

        // Consumes the condtion
        self.expect_peek(RawExpression::is, ErrorCode::EP011)?;
        condition = self.parse_expression(0)?;
        
        // Consumes the body
        self.expect_peek(TokenKind::LeftBrace, ErrorCode::EP012)?;
        body = self.parse_body()?;

        self.inside_while = false;

        Ok(self.statement(
            RawStatement::While { 
                condition, 
                body 
            }
        ))
    }

    fn parse_identifier(&mut self) -> Result<Statement, ParserError> {
        let identifier: Token = self.next();

        // If the next token is an assigment operator (=, +=, -=, etc.)
        // Then it's a variable assignment
        if self.match_peek(Parser::is_assignment()) {
            self.parse_variable_assignment(identifier)
        }
        // If'the next token is a '(' then it's a function call
        else if self.match_peek(TokenKind::LeftParen){
            self.parse_function_call(identifier)
        }
        // The user wrote something that has no sense after the identifier
        else {
            Err(self.error(ErrorCode::EP013))
        }
    }

    fn parse_function_call(&mut self, name: Token) -> Result<Statement, ParserError> {
        // Function call are a bit more complex because they can be either
        // expression or statements, this function here parse the statement version
        // which is usually points to a function with no return value

        self.next(); // Consumes the '('

        // Consumes the arguments
        let arguments: Vec<Expression> = self.parse_arguments()?;

        // Consumes the ';'
        self.expect_next(TokenKind::Semicolon, ErrorCode::EP017)?;

        Ok(self.statement(RawStatement::FunctionCall { 
            name: name.span.literal, 
            arguments 
        }))
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut arguments: Vec<Expression> = Vec::new();
        
        // Loop to get all the arguments
        while let Some(_) = self.peek() {
            // Here means incomplete arguments, because it abruptly ends
            if self.peeked.kind == TokenKind::Eof {
                return Err(self.error(ErrorCode::EP014));
            }

            // Found the ')' we can safely quit
            if self.peeked.kind == TokenKind::RightParen {
                self.next();
                break;
            }

            // We check for an expression because argument are just either
            // variables, function calls, or plain math expressions
            self.expect_peek(RawExpression::is, ErrorCode::EP015)?;
            arguments.push(self.parse_expression(0)?);

            // Now after we got the expression we expect a comma or a right paren
            // The check for the right paren is at the start so no point in repeating code
            if self.match_peek(TokenKind::Comma){
                self.next();

                // We correctly found the comma, but if the next token is a
                // right paren it means the user didn't end the arguments properly
                if self.match_peek(TokenKind::RightParen) {
                    return Err(self.error(ErrorCode::EP016));
                }
            }            
        }

        Ok(arguments)
    }

    fn parse_variable_assignment(&mut self, name: Token) -> Result<Statement, ParserError> {
        let operator: TokenKind = self.next().kind; // Consumes the '=', '+=' and similar
        
        // Consumes the expression to assing to the variable
        self.expect_peek(RawExpression::is, ErrorCode::EP018)?;
        let value: Expression = self.parse_expression(0)?;
        
        // Consumes the ';'
        self.expect_next(TokenKind::Semicolon, ErrorCode::EP019)?;
        
        Ok(self.statement(
            RawStatement::VariableAssignment {
            name: name.span.literal, 
            operator, 
            value 
        }))
    }

    fn parse_loop_control(&mut self) -> Result<Statement, ParserError> {
        let keyword: Token = self.next(); // Parses either 'break' or 'continue'

        // Consumes the ';'
        self.expect_next(TokenKind::Semicolon, ErrorCode::EP020)?;

        Ok(self.statement(
            RawStatement::LoopControl(keyword.span.literal)
        ))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.next(); // Parses the 'return' keyword
        
        let mut expression: Option<Expression> = None;

        // This parses a return with no value 'return;'
        if self.match_peek(TokenKind::Semicolon) {
            self.next();
        }
        // This parses a return with a value
        else if self.match_peek(RawExpression::is) {
            expression = Some(self.parse_expression(0)?);
            self.expect_next(TokenKind::Semicolon, ErrorCode::EP022)?;
        }
        // The user has put something invalid after the return keyword
        else {
            return Err(self.error(ErrorCode::EP021));
        }

        Ok(self.statement(
            RawStatement::Return(expression))
        )
    }

    fn parse_function(&mut self) -> Result<Statement, ParserError> {
        // This will help later on when i have to check for the validity
        // of a return statement, because if it's not in a function
        // it's obviously invalid
        self.inside_function = true;

        self.next(); // Consumes the 'fn' keyword
        
        // Consumes the name
        let name: SourceLiteral = self.expect_next(TokenKind::Identifier, 
                    ErrorCode::EP023)?.span.literal;
        
        // Consumes the parameters
        self.expect_next(TokenKind::LeftParen, ErrorCode::EP024)?;
        let parameters: Vec<Parameter> = self.parse_parameters()?;
        
        // This parses the type
        let mut type_: Type = Type { kind: TokenKind::Null, is_array: false, array_length: None };
        // If the next token isn't a left brace then it's a type
        if !self.match_peek(TokenKind::LeftBrace){
            // Now if the token is actually type then parse it
            if self.match_peek(Type::is) {
                type_ = self.parse_type()?;
            }
            // Otherwise it's an error
            else {
                return Err(self.error(ErrorCode::EP025));
            }
        }

        // Consumes the body
        self.expect_peek(TokenKind::LeftBrace, ErrorCode::EP026)?;
        let body: Body = self.parse_body()?;

        self.inside_function = false;
        Ok(self.statement(
            RawStatement::Function { 
                name, 
                parameters, 
                type_, 
                body 
            }
        ))
    }

    fn parse_parameters(&mut self) -> Result<Vec<Parameter>, ParserError> {
        let mut parameters: Vec<Parameter> = Vec::new();
        
        // Loops to find all the parameters
        while let Some(_) = self.peek() {
            // This means incomplete parameters, because it's missing the ')'
            if self.peeked.kind == TokenKind::Eof {
                return Err(self.error(ErrorCode::EP027));
            }

            // Found the ')' we can quit
            if self.peeked.kind == TokenKind::RightParen {
                self.next();
                break;
            }
            
            let is_const: bool = self.is_const();

            // Consumes the parameter type
            self.expect_peek(Type::is, ErrorCode::EP028)?;
            let type_: Type = self.parse_type()?;

            // Consumes the name
            let name: SourceLiteral = self.expect_next(TokenKind::Identifier, 
                        ErrorCode::EP029)?.span.literal;

            parameters.push(Parameter {
                is_const,
                name, 
                type_ 
            });

            // Same thing as for arguments, if the next token is a comma
            // we consume it and check if the next token is a type otherwise
            // it's an error
            if self.match_peek(TokenKind::Comma){
                self.next();
                if !self.match_peek(Type::is) {
                    return Err(self.error(ErrorCode::EP030));
                }
            }            
        }

        Ok(parameters)
    }

    // TODO: Provide errors function calls uses keyword names, add as hint
    fn parse_expression(&mut self, min_bp: u8) -> Result<Expression, ParserError> {
        let expression_start: usize = self.peeked.span.start;
        
        let token: Token = self.next(); // Consume the first token

        // This is the first part of the expression 
        // So this is either a literal, variable, parenthesis or a unary operator
        let mut left: Expression = match token.kind {
            TokenKind::IntegerLiteral | TokenKind::FloatLiteral | TokenKind::BooleanLiteral |
            TokenKind::CharLiteral    | TokenKind::StringLiteral => {
                self.expression(expression_start, 
                    RawExpression::Literal { 
                        kind: token.kind, 
                        value: token.span.literal 
                    }
                )
            },

            TokenKind::Identifier => {
                self.expression(expression_start, 
                    RawExpression::Variable(token.span.literal)
                )
            },

            kind if Type::is(kind) => {
                if kind == TokenKind::Const {
                    return Err(self.error(ErrorCode::EP035));
                }

                // 1. Expect the opening parenthesis '('
                self.expect_next(TokenKind::LeftParen, ErrorCode::EP036)?;

                self.expect_peek(RawExpression::is, ErrorCode::EP037)?;
                let expression: Expression = self.parse_expression(0)?;
                
                // 3. Expect the closing parenthesis ')'
                self.expect_next(TokenKind::RightParen, ErrorCode::EP038)?;

                self.expression(expression_start, RawExpression::Cast {
                    kind,
                    expression,
                })
            },
            
            TokenKind::Minus | TokenKind::Not => {
                // Unary Op: recursive call with high binding power
                let operand: Expression = self.parse_expression(4)?; 
                self.expression(expression_start, 
                        RawExpression::Unary { 
                        operator: token.kind, 
                        operand 
                    }
                )
            },
            
            TokenKind::LeftParen => {
                // Grouping: reset BP to 0 to parse inside the parens
                let expression: Expression = self.parse_expression(0)?;
                self.expect_next(TokenKind::RightParen, ErrorCode::EP031)?;
                expression
            }

            _ => return Err(self.error(ErrorCode::EP032)),
        };

        // This is the operator and the right part of the expression
        loop {
            let operator: Token = match self.peek() {
                    Some(t) if t.kind != TokenKind::Eof => t,
                    _ => break,
                };

                let (l_bp, r_bp) = RawExpression::get_binding_power(operator.kind);
                
                // If it's not an operator (bp 0) or isn't strong enough, stop immediately.
                if l_bp == 0 || l_bp < min_bp {
                    break;
                }

                self.next();

            match operator.kind {
                // Standard Math
                TokenKind::Plus | TokenKind::Minus | 
                TokenKind::Multiplication | TokenKind::Division |
                TokenKind::Modulus => {
                    let right: Expression = self.parse_expression(r_bp)?;
                    left = self.expression(expression_start, 
                            RawExpression::Binary {
                            left,
                            operator: operator.kind,
                            right,
                        }
                    );
                }

                // Array Access: name[index]
                TokenKind::LeftBracket => {
                    let index: Expression = self.parse_expression(0)?; // Inner expr
                    self.expect_next(TokenKind::RightBracket, ErrorCode::EP033)?;
                    left = self.expression(expression_start, RawExpression::ArrayAccess {
                        name: left,
                        index,
                    });
                }

                // Function Call: name(arg1, arg2)
                TokenKind::LeftParen => {
                    // Here 'left' is the function name (Expression::Variable)
                    let name: SourceLiteral = match left.node {
                        RawExpression::Variable(n) => n,
            
                        // The error here means that it's trying to call a non-function
                        _ => return Err(self.error(ErrorCode::EP034)),
                    };

                    let arguments: Vec<Expression> = self.parse_arguments()?;

                    left = self.expression(expression_start, 
                        RawExpression::FunctionCall {
                            name,
                            arguments,
                        }
                    );
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn statement(&mut self, node: RawStatement) -> Statement {
        Box::new(Spanned { 
            node, 
            span: StatementSpan { 
                start: self.statement_start, 
                end: self.statement_end 
            } 
        })
    }

    fn expression(&mut self, expression_start: usize, node: RawExpression) -> Expression {
        Box::new(Spanned { 
            node, 
            span: StatementSpan { 
                start: expression_start, 
                end: self.expression_end 
            } 
        })
    }

    fn error(&self, code: ErrorCode) -> ParserError {
        ParserError { 
            code, 
            span: StatementSpan { 
                start: self.statement_start, 
                end: self.statement_end 
            },
            line: self.line,
            column: self.column
        }
    }


    fn next(&mut self) -> Token {
        match self.tokens.next() {
            Some(token) => {
                self.statement_end = token.span.end;
                self.expression_end = token.span.end;
                self.line = token.span.line;
                self.column = token.span.column;
                token
            }
            None => {
                panic!("Compiler Error! Before each call to next(), call peek()");
            }
        }
    }

    fn peek(&mut self) -> Option<Token> { 
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

    fn expect_next<M: TokenMatcher>(&mut self, matcher: M, error: ErrorCode) -> Result<Token, ParserError> {
        if let Some(token) = self.peek() {
            if matcher.matches(token.kind) {
                return Ok(self.next());
            }
        }
        Err(self.error(error))
    }

    fn expect_peek<M: TokenMatcher>(&mut self, matcher: M, error: ErrorCode) -> Result<Token, ParserError> {
        if let Some(token) = self.peek() {
            if matcher.matches(token.kind) {
                return Ok(token);
            }
        }
        Err(self.error(error))
    }

}