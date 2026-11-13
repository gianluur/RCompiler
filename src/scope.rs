use crate::ast::{Expression, Parameter, Type};
use crate::interner::SourceLiteral;

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
    Boolean(bool),
    Array {
        elements: Vec<Value>,
        element_type_: Type,
    },
}

impl Value {
    pub fn from(type_: &Type, expression: Option<Expression>) -> Value {
        if let Some(expression) = expression {
            use crate::ast::RawExpression::*;
            match expression.node {
                // Literal { kind, value } => {}
                _ => todo!("Conversion from expression not finished yet"),
            }
        } else {
            Value::Null
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SymbolKind {
    Variable {
        is_const: bool,
        type_: Type,
        value: Value,
    },
    Function {
        parameters: Vec<Symbol>,
        return_type: Type,
    },
    Parameter {
        is_const: bool,
        type_: Type,
    },
}

impl SymbolKind {
    fn get_type(&self) -> &Type {
        match self {
            SymbolKind::Variable { type_, .. } => type_,
            SymbolKind::Function { return_type, .. } => return_type,
            SymbolKind::Parameter { type_, .. } => type_,
        }
    }

    fn get_parameters(&self) -> Option<&Vec<Symbol>> {
        match self {
            SymbolKind::Function { parameters, .. } => Some(parameters),
            _ => None,
        }
    }

    fn is_const(&self) -> bool {
        match self {
            SymbolKind::Variable { is_const, .. } => *is_const,
            SymbolKind::Parameter { is_const, .. } => *is_const,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Symbol {
    pub name: SourceLiteral,
    pub kind: SymbolKind,
}

impl Symbol {
    pub fn variable(
        is_const: bool,
        type_: Type,
        name: SourceLiteral,
        value: Option<Expression>,
    ) -> Symbol {
        Symbol {
            name,
            kind: SymbolKind::Variable {
                is_const,
                value: Value::from(&type_, value),
                type_,
            },
        }
    }

    pub fn is_variable(&self) -> bool {
        matches!(self.kind, SymbolKind::Variable { .. })
    }

    pub fn parameter(parameter: Parameter) -> Symbol {
        Symbol {
            name: parameter.name,
            kind: SymbolKind::Parameter {
                is_const: parameter.is_const,
                type_: parameter.type_,
            },
        }
    }

    pub fn is_parameter(&self) -> bool {
        matches!(self.kind, SymbolKind::Parameter { .. })
    }

    pub fn function(name: SourceLiteral, parameters: Vec<Parameter>, type_: Type) -> Symbol {
        let mut parameters_symbols = Vec::new();
        for parameter in parameters {
            parameters_symbols.push(Symbol::parameter(parameter));
        }

        Symbol {
            name,
            kind: SymbolKind::Function {
                parameters: parameters_symbols,
                return_type: type_,
            },
        }
    }

    pub fn is_function(&self) -> bool {
        matches!(self.kind, SymbolKind::Function { .. })
    }

    pub fn get_type(&self) -> &Type {
        self.kind.get_type()
    }

    pub fn get_parameters(&self) -> Option<&Vec<Symbol>> {
        self.kind.get_parameters()
    }

    pub fn is_const(&self) -> bool {
        self.kind.is_const()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ScopeKind {
    Global,
    If,
    Else,
    While,
    Function,
}

pub struct Scope {
    pub kind: ScopeKind,
    pub symbols: Vec<HashMap<SourceLiteral, Symbol>>,
    pub scope: usize,
    pub loop_depth: Vec<bool>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            kind: ScopeKind::Global,
            symbols: vec![HashMap::new()],
            scope: 0,
            loop_depth: Vec::new(),
        }
    }

    pub fn enter(&mut self, kind: ScopeKind) {
        if kind == ScopeKind::While {
            self.loop_depth.push(true);
        }

        self.symbols.push(HashMap::new());
        self.kind = kind;
        self.scope += 1;
    }

    pub fn exit(&mut self) {
        if self.kind == ScopeKind::While {
            self.loop_depth.pop();
        }

        self.symbols.pop();
        self.scope -= 1;
    }

    pub fn declare(&mut self, symbol: Symbol) {
        self.symbols.last_mut().unwrap().insert(symbol.name, symbol);
    }

    pub fn is_declared(&self, name: &SourceLiteral) -> bool {
        for symbols in self.symbols.iter().rev() {
            if symbols.contains_key(name) {
                return true;
            }
        }
        return false;
    }

    pub fn is_redeclared(&self, name: &SourceLiteral) -> bool {
        self.symbols.last().unwrap().contains_key(name)
    }

    pub fn find(&self, name: &SourceLiteral) -> Option<&Symbol> {
        for symbols in self.symbols.iter().rev() {
            if let Some(symbol) = symbols.get(name) {
                return Some(&symbol);
            }
        }
        None
    }
}
