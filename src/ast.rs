use std::rc::Rc;

use crate::interner::SourceLiteral;
use crate::tokens::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StatementSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, PartialEq)]
pub struct Spanned<T> {
    pub node: T,
    pub span: StatementSpan,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.node.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer,
    Float,
    Char,
    Boolean,
    String,
}

impl Literal {
    pub fn from(kind: TokenKind) -> Literal {
        use Literal::*;
        return match kind {
            TokenKind::IntegerLiteral => Integer,
            TokenKind::FloatLiteral => Float,
            TokenKind::CharLiteral => Char,
            TokenKind::BooleanLiteral => Boolean,
            TokenKind::StringLiteral => String,
            _ => unreachable!("Every literal is beforehand checked"),
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    // Math
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,

    // Logic
    And,
    Or,
    Not,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,

    // Bits
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseLShift,
    BitwiseRShift,

    // Assignment
    Assignment,
    AdditionAssignment,
    SubtractAssignment,
    MultiplyAssignment,
    DivideAssignment,
    ModulusAssignment,
    BitwiseAndAssignment,
    BitwiseOrAssignment,
    BitwiseXorAssignment,
    BitwiseLShiftAssignment,
    BitwiseRShiftAssignment,
}

impl Operator {
    pub fn from(kind: TokenKind) -> Operator {
        use Operator::*;

        return match kind {
            // Math
            TokenKind::Plus => Addition,
            TokenKind::Minus => Subtraction,
            TokenKind::Multiplication => Multiplication,
            TokenKind::Division => Division,
            TokenKind::Modulus => Modulus,

            // Logic
            TokenKind::And => And,
            TokenKind::Or => Or,
            TokenKind::Not => Not,
            TokenKind::Equal => Equal,
            TokenKind::NotEqual => NotEqual,
            TokenKind::GreaterThan => GreaterThan,
            TokenKind::LessThan => LessThan,
            TokenKind::GreaterThanOrEqual => GreaterThanOrEqual,
            TokenKind::LessThanOrEqual => LessThanOrEqual,

            // Bits
            TokenKind::BitwiseAnd => BitwiseAnd,
            TokenKind::BitwiseOr => BitwiseOr,
            TokenKind::BitwiseXor => BitwiseXor,
            TokenKind::BitwiseLShift => BitwiseLShift,
            TokenKind::BitwiseRShift => BitwiseRShift,

            // Assignment
            TokenKind::Assignment => Assignment,
            TokenKind::AdditionAssignment => AdditionAssignment,
            TokenKind::SubtractAssignment => SubtractAssignment,
            TokenKind::MultiplyAssignment => MultiplyAssignment,
            TokenKind::DivideAssignment => DivideAssignment,
            TokenKind::ModulusAssignment => ModulusAssignment,
            TokenKind::BitwiseAndAssignment => BitwiseAndAssignment,
            TokenKind::BitwiseOrAssignment => BitwiseOrAssignment,
            TokenKind::BitwiseXorAssignment => BitwiseXorAssignment,
            TokenKind::BitwiseLShiftAssignment => BitwiseLShiftAssignment,
            TokenKind::BitwiseRShiftAssignment => BitwiseRShiftAssignment,

            _ => unreachable!("Operators are checked beforehand"),
        };
    }

    pub fn is(kind: TokenKind) -> bool {
        matches!(
            kind,
            // Math
            TokenKind::Plus
          | TokenKind::Minus
          | TokenKind::Multiplication
          | TokenKind::Division
          | TokenKind::Modulus

          // Logic
          | TokenKind::And
          | TokenKind::Or
          | TokenKind::Not
          | TokenKind::Equal
          | TokenKind::NotEqual
          | TokenKind::GreaterThan
          | TokenKind::LessThan
          | TokenKind::GreaterThanOrEqual
          | TokenKind::LessThanOrEqual

          // Bits
          | TokenKind::BitwiseAnd
          | TokenKind::BitwiseOr
          | TokenKind::BitwiseXor
          | TokenKind::BitwiseLShift
          | TokenKind::BitwiseRShift

          // Assignment
          | TokenKind::Assignment
          | TokenKind::AdditionAssignment
          | TokenKind::SubtractAssignment
          | TokenKind::MultiplyAssignment
          | TokenKind::DivideAssignment
          | TokenKind::ModulusAssignment
          | TokenKind::BitwiseAndAssignment
          | TokenKind::BitwiseOrAssignment
          | TokenKind::BitwiseXorAssignment
          | TokenKind::BitwiseLShiftAssignment
          | TokenKind::BitwiseRShiftAssignment
        )
    }

    pub fn is_assignment(kind: TokenKind) -> bool {
        matches!(
            kind,
            // Assignment
            |TokenKind::Assignment| TokenKind::AdditionAssignment
                | TokenKind::SubtractAssignment
                | TokenKind::MultiplyAssignment
                | TokenKind::DivideAssignment
                | TokenKind::ModulusAssignment
                | TokenKind::BitwiseAndAssignment
                | TokenKind::BitwiseOrAssignment
                | TokenKind::BitwiseXorAssignment
                | TokenKind::BitwiseLShiftAssignment
                | TokenKind::BitwiseRShiftAssignment
        )
    }

    pub fn is_math(kind: TokenKind) -> bool {
        matches!(
            kind,
            // Math
            TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Multiplication
                | TokenKind::Division
                | TokenKind::Modulus
        )
    }

    pub fn is_logic(&self) -> bool {
        use Operator::*;
        matches!(self, And | Or)
    }

    pub fn is_comparison(&self) -> bool {
        use Operator::*;
        matches!(
            self,
            Equal | NotEqual | LessThan | GreaterThan | LessThanOrEqual | GreaterThanOrEqual
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RawExpression {
    Variable(SourceLiteral),
    Literal {
        kind: Literal,
        value: SourceLiteral,
    },
    Binary {
        left: Expression,
        operator: Operator,
        right: Expression,
    },
    Unary {
        operator: Operator,
        operand: Expression,
    },
    FunctionCall {
        name: SourceLiteral,
        arguments: Vec<Expression>,
    },
    ArrayAccess {
        name: Expression,
        index: Expression,
    },
    Cast {
        kind: Type,
        expression: Expression,
    },
    InitializerList {
        list: Vec<Expression>,
        expected_type: Option<Type>,
    },
}

#[derive(Debug, Clone)]
pub enum RawStatement {
    VariableDeclaration {
        is_const: bool,
        type_: Type,
        name: SourceLiteral,
        value: Option<Expression>,
    },
    VariableAssignment {
        name: SourceLiteral,
        operator: Operator,
        value: Expression,
    },
    If {
        condition: Expression,
        body: Body,
        elses: Vec<ElseBranch>,
    },
    While {
        condition: Expression,
        body: Body,
    },
    Break,
    Continue,
    Function {
        name: SourceLiteral,
        parameters: Vec<Parameter>,
        type_: Type,
        body: Body,
    },
    Return(Option<Expression>),
    FunctionCall {
        name: SourceLiteral,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum ElseBranch {
    ElseIf(Statement),
    Else(Body),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Char,
    String,
    Bool,
    Null,
    Array {
        kind: Box<Type>,
        length: Option<Expression>,
    },
}

impl Type {
    pub fn is() -> impl Fn(TokenKind) -> bool {
        use TokenKind::*;
        |kind: TokenKind| {
            matches!(
                kind,
                Int8 | Int16
                    | Int32
                    | Int64
                    | UInt8
                    | UInt16
                    | UInt32
                    | UInt64
                    | Float32
                    | Float64
                    | Character
                    | String
                    | Boolean
            )
        }
    }

    pub fn from(kind: TokenKind) -> Type {
        match kind {
            TokenKind::Int8 => Type::Int8,
            TokenKind::Int16 => Type::Int16,
            TokenKind::Int32 => Type::Int32,
            TokenKind::Int64 => Type::Int64,
            TokenKind::UInt8 => Type::UInt8,
            TokenKind::UInt16 => Type::UInt16,
            TokenKind::UInt32 => Type::UInt32,
            TokenKind::UInt64 => Type::UInt64,
            TokenKind::Float32 => Type::Float32,
            TokenKind::Float64 => Type::Float64,
            TokenKind::Character => Type::Char,
            TokenKind::String => Type::String,
            TokenKind::Boolean => Type::Bool,
            TokenKind::Null => Type::Null,
            _ => unreachable!(),
        }
    }

    pub fn is_integer(&self) -> bool {
        self.is_signed_integer() || self.is_unsigned_integer()
    }

    pub fn is_signed_integer(&self) -> bool {
        use Type::*;
        matches!(self, Int8 | Int16 | Int32 | Int64)
    }

    pub fn is_unsigned_integer(&self) -> bool {
        use Type::*;
        matches!(self, UInt8 | UInt16 | UInt32 | UInt64)
    }

    pub fn is_float(&self) -> bool {
        use Type::*;
        matches!(self, Float32 | Float64)
    }

    pub fn is_numeric(&self) -> bool {
        self.is_integer() || self.is_float()
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Type::Bool)
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Type::Null)
    }

    pub fn i32() -> Self {
        Type::Int32
    }

    pub fn f32() -> Self {
        Type::Float32
    }

    pub fn bool() -> Self {
        Type::Bool
    }

    pub fn char() -> Self {
        Type::Char
    }

    pub fn string() -> Self {
        Type::String
    }

    pub fn null() -> Self {
        Type::Null
    }

    pub fn array(kind: Type, length: Option<Expression>) -> Self {
        Type::Array {
            kind: Box::new(kind),
            length,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub is_const: bool,
    pub name: SourceLiteral,
    pub type_: Type,
}

#[derive(Debug, Clone)]
pub struct Body {
    pub statements: Vec<Statement>,
}

pub type Expression = Rc<Spanned<RawExpression>>;
pub type Statement = Spanned<RawStatement>;
