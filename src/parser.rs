// use crate::tokenizer::{TokenKind, Token};
// use std::iter::Peekable;
// use std::slice::Iter;

// #[derive(Debug, Clone, Copy)]
// pub struct Span {
//     pub start: usize,
//     pub end: usize,
// }

// #[derive(Debug, Clone)]
// pub struct Spanned<T> {
//     pub node: T,
//     pub span: Span,
// }

// impl<T> Spanned<T> {
//     pub fn new(node: T, span: Span) -> Self {
//         Self { node, span }
//     }
// }

// pub type Expression = Box<Spanned<RawExpression>>;
// pub type Statement = Box<Spanned<RawStatement>>;

// #[derive(Debug, Clone)]
// pub struct Type {
//     pub kind: TokenKind,
//     pub is_array: bool,
// }

// #[derive(Debug, Clone)]
// pub struct Parameter {
//     pub name: String,
//     pub ty: Type,
// }

// #[derive(Debug, Clone)]
// pub struct Block {
//     pub statements: Vec<Statement>,
//     pub span: Span,
// }

// #[derive(Debug, Clone)]
// pub enum RawExpression {
//     Variable(String),

//     Literal {
//         kind: TokenKind,
//         value: String,
//     },

//     Binary {
//         left: Expression,
//         operator: TokenKind,
//         right: Expression,
//     },

//     Unary {
//         operator: TokenKind,
//         operand: Expression,
//     },


//     FunctionCall {
//         name: String,
//         arguments: Vec<Expression>,
//     },

//     ArrayAccess {
//         array: Expression,
//         index: Expression,
//     },
// }

// #[derive(Debug, Clone)]
// pub enum RawStatement {
//     VariableDeclaration {
//         keyword: TokenKind,
//         name: String,
//         ty: Type,
//         value: Option<Expression>,
//     },

//     VariableAssignment {
//         name: String,
//         operator: TokenKind,
//         value: Expression,
//     },

//     If {
//         condition: Expression,
//         then_branch: Block,
//         elses_branch: Option<ElseBranch>,
//     },

//     While {
//         condition: Expression,
//         body: Block,
//     },

//     Break,
//     Continue,

//     FunctionCall(Expression),
//     Return(Option<Expression>),
// }

// #[derive(Debug, Clone)]
// pub enum ElseBranch {
//     ElseIf(Statement),
//     Else(Block),
// }

// struct Parser {
//     tokens: Peekable<std::vec::IntoIter<Token>>,
//     position: usize
// }

// impl Parser {
//     pub fn new(tokens: Vec<Token>) -> Self {
//         Self {
//             tokens: tokens.into_iter().peekable(),
//             position: 0
//         }
//     }

//     pub fn parse(&mut self) -> Vec<Statement> {
//         let mut statements: Vec<Statement> = Vec::new();
        
//         while let Some(token) = self.next() {
//             statements.push(self.get_statement(token.kind));
//         }

//         return statements
//     }

//     fn get_statement(&mut self, token: TokenKind) -> Box<Spanned<RawStatement>> {
        
//     }

//     fn is_type(&self, token: &TokenKind) -> bool {
//         match token {
//             TokenKind::SignedInt8 | TokenKind::SignedInt16 | TokenKind::SignedInt32 | TokenKind::SignedInt64 |
//             TokenKind::UnsignedInt8 | TokenKind::UnsignedInt16 | TokenKind::UnsignedInt32 | TokenKind::UnsignedInt64 |
//             TokenKind::Float32 | TokenKind::Float64 | 
//             TokenKind::Character | TokenKind::Boolean | TokenKind::String => true,

//             _ => false
//         }
//     }

//     // TODO: For now just parse simple type then add array when expression are done
//     fn parse_type() -> Type {
//         Type {
//             kind: TokenKind::SignedInt32,
//             is_array: false,
//         }
//     }

//     fn parse_variable_declaration() {
        
//     }

//     fn next(&mut self) -> Option<Token> {
//         self.tokens.next()
//     }

//     fn peek(&mut self) -> Option<&Token> {
//         self.tokens.peek()
//     }
// }