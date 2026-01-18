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
    }
}

impl RawExpression {
    pub fn is(kind: TokenKind) -> bool {
        use TokenKind::*;
        matches!(kind,
            IntegerLiteral | FloatLiteral | BooleanLiteral |
            CharLiteral | StringLiteral | LeftParen | Identifier | Minus | Not
        ) || Type::is(kind)
    }

    pub fn get_binding_power(kind: TokenKind) -> (u8, u8) {
        match kind {
            // Logical OR/AND (Lowest)
            TokenKind::Or | TokenKind::And => (5, 6),

            // Equality
            TokenKind::Equal | TokenKind::NotEqual => (10, 11),

            // Relational
            TokenKind::LessThan | TokenKind::LessThanOrEqual |
            TokenKind::GreaterThan | TokenKind::GreaterThanOrEqual => (20, 21),

            // Shifts
            TokenKind::BitwiseLShift | TokenKind::BitwiseRShift => (30, 31),

            // Additive
            TokenKind::Plus | TokenKind::Minus => (40, 41),

            // Multiplicative
            TokenKind::Multiplication | TokenKind::Division | TokenKind::Modulus => (50, 51),

            // Highest: Calls and Indexing
            TokenKind::LeftParen | TokenKind::LeftBracket => (80, 81),

            _ => (0, 0),
        }
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

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub kind: TokenKind,
    pub is_array: bool,
    pub array_length: Option<Expression>,
}

impl Type {    
    pub fn is(kind: TokenKind) -> bool {
        use TokenKind::*;
        matches!(kind, 
            SignedInt8 | SignedInt16 | SignedInt32 | SignedInt64 |
            UnsignedInt8 | UnsignedInt16 | UnsignedInt32 | UnsignedInt64 |
            Float32 | Float64 | Character | String | Boolean | Const
        )
    }

    pub fn is_integer(type_: &Type) -> bool {
        use TokenKind::*;
        matches!(type_.kind, 
            SignedInt8 | SignedInt16 | SignedInt32 | SignedInt64 |
            UnsignedInt8 | UnsignedInt16 | UnsignedInt32 | UnsignedInt64
        )
    }

    pub fn is_float(type_: &Type) -> bool {
        use TokenKind::*;
        matches!(type_.kind, 
            Float32 | Float64
        )
    }

    pub fn is_numeric(type_: &Type) -> bool {
        use TokenKind::*;
        matches!(type_.kind, 
            SignedInt8 | SignedInt16 | SignedInt32 | SignedInt64 |
            UnsignedInt8 | UnsignedInt16 | UnsignedInt32 | UnsignedInt64 |
            Float32 | Float64
        )
    }

    pub fn is_boolean(type_: &Type) -> bool {
        use TokenKind::*;
        matches!(type_.kind, 
            Boolean
        )
    }

    pub fn is_character(type_: &Type) -> bool {
        use TokenKind::*;
        matches!(type_.kind, 
            Character
        )
    }

    pub fn is_string(type_: &Type) -> bool {
        use TokenKind::*;
        matches!(type_.kind, 
            String
        )
    }

    pub fn is_null(type_: &Type) -> bool {
        use TokenKind::*;
        matches!(type_.kind, 
            Null
        )
    }

    fn rank(&self) -> u8 {
        use TokenKind::*;
        match self.kind {
            SignedInt8  | UnsignedInt8  => 1,
            SignedInt16 | UnsignedInt16 => 2,
            SignedInt32 | UnsignedInt32 => 3,
            SignedInt64 | UnsignedInt64 => 4,
            Float32 => 5,
            Float64 => 6,
            _ => 0, // Non-numeric or complex types
        }
    }

    pub fn are_equal(left: &Type, right: &Type) -> bool {
        if left.kind == right.kind && left.is_array == right.is_array {
            return true;
        }
        return false;

        // TODO Add other checks
    }

    pub fn integer() -> Type {
        Type {
            kind: TokenKind::SignedInt32,
            is_array: false,
            array_length: None,
        }
    }

    pub fn float() -> Type {
        Type {
            kind: TokenKind::Float32,
            is_array: false,
            array_length: None,
        }
    }

    pub fn boolean() -> Type {
        Type {
            kind: TokenKind::Boolean,
            is_array: false,
            array_length: None,
        }
    }

    pub fn character() -> Type {
        Type {
            kind: TokenKind::Character,
            is_array: false,
            array_length: None,
        }
    }

    pub fn string() -> Type {
        Type {
            kind: TokenKind::String,
            is_array: false,
            array_length: None,
        }
    }

    pub fn null() -> Type {
        Type {
            kind: TokenKind::Null,
            is_array: false,
            array_length: None,
        }
    }

    pub fn from(kind: TokenKind) -> Type {
        Type {
            kind,
            is_array: false,
            array_length: None,
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

pub type Expression = Box<Spanned<RawExpression>>;
pub type Statement = Box<Spanned<RawStatement>>;