use std::rc::Rc;

use crate::tokenizer::TokenKind;
use crate::interner::SourceLiteral;

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
pub enum RawExpression {
    Variable(SourceLiteral),
    Literal {
        kind: TokenKind,
        value: SourceLiteral,
    },
    Binary {
        left: Expression,
        operator: TokenKind,
        right: Expression,
    },
    Unary {
        operator: TokenKind,
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
        kind: TokenKind,
        expression: Expression,
    },
    InitializerList {
        list: Vec<Expression>,
        expected_type: Option<Type>,
    }
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
        operator: TokenKind,
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
    LoopControl(SourceLiteral),
    Function {
      name: SourceLiteral,
      parameters: Vec<Parameter>,
      type_: Type,
      body: Body
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    I8, I16, I32, I64,
    U8, U16, U32, U64,
    F32, F64,
    Char, Str, Bool, Null,
}


impl PrimitiveType {
    pub fn from(kind: TokenKind) -> PrimitiveType {
        match kind {
            TokenKind::SignedInt8 => PrimitiveType::I8,
            TokenKind::SignedInt16 => PrimitiveType::I16,
            TokenKind::SignedInt32 => PrimitiveType::I32,
            TokenKind::SignedInt64 => PrimitiveType::I64,
            TokenKind::UnsignedInt8 => PrimitiveType::U8,
            TokenKind::UnsignedInt16 => PrimitiveType::U16,
            TokenKind::UnsignedInt32 => PrimitiveType::U32,
            TokenKind::UnsignedInt64 => PrimitiveType::U64,
            TokenKind::Float32 => PrimitiveType::F32,
            TokenKind::Float64 => PrimitiveType::F64,
            TokenKind::Character => PrimitiveType::Char,
            TokenKind::String => PrimitiveType::Str,
            TokenKind::Boolean => PrimitiveType::Bool,
            _ => unreachable!(),
        }
    }

    pub fn is_integer(&self) -> bool {
        use PrimitiveType::*;
        matches!(self, I8 | I16 | I32 | I64 | U8 | U16 | U32 | U64)
    }
    
    pub fn is_float(&self) -> bool {
        use PrimitiveType::*;
        matches!(self, F32 | F64)
    }
    
    pub fn numeric_rank(&self) -> u8 {
        use PrimitiveType::*;
        match self {
            I8  | U8  => 1,
            I16 | U16 => 2,
            I32 | U32 => 3,
            I64 | U64 => 4,
            F32       => 5,
            F64       => 6,
            _         => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Primitive(PrimitiveType),
    Array {
        kind: Box<Type>,
        length: Option<Expression>,
    },
}

impl Type {
    pub fn from(kind: TokenKind) -> Type {
        Type::Primitive(PrimitiveType::from(kind))
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Type::Primitive(p) => p.is_integer(),
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Type::Primitive(p) => p.is_float(),
            _ => false,
        }
    }

    pub fn is_numeric(&self) -> bool {
        match self {
            Type::Primitive(p) => p.is_integer() || p.is_float(),
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Type::Primitive(PrimitiveType::Bool))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Type::Primitive(PrimitiveType::Null))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Type::Array { .. })
    }

    pub fn i32() -> Self { 
        Type::Primitive(PrimitiveType::I32) 
    }

    pub fn f32() -> Self {
        Type::Primitive(PrimitiveType::F32) 
    }
    
    pub fn bool() -> Self {
        Type::Primitive(PrimitiveType::Bool) 
    }

    pub fn char() -> Self {
        Type::Primitive(PrimitiveType::Char)
    }

    pub fn string() -> Self { 
        Type::Primitive(PrimitiveType::Str) 
    }

    pub fn null() -> Self {
        Type::Primitive(PrimitiveType::Null)
    }

    pub fn array(kind: Type, length: Option<Expression>) -> Self {
        Type::Array { 
            kind: Box::new(kind), 
            length 
        }
    }

    pub fn are_equal(a: &Type, b: &Type) -> bool {
        match (a, b) {
            (Type::Primitive(a_p), Type::Primitive(b_p)) => a_p == b_p,
            (Type::Array { kind: a_k, .. }, Type::Array { kind: b_k, .. }) => {
                // Recursively check if the types inside the arrays are the same
                Type::are_equal(a_k, b_k)
            },
            _ => false,
        }
    }

    /// Returns the rank for numeric coercion (e.g., i32 + f32 -> f32)
    pub fn rank(&self) -> u8 {
        match self {
            Type::Primitive(p) => p.numeric_rank(),
            _ => 0,
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
pub type Statement = Rc<Spanned<RawStatement>>;