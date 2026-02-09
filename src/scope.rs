use crate::interner::SourceLiteral;
use crate::ast::{Expression, Type, Parameter};

use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

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
        element_type_: Type
    },
}

impl Value {
    pub fn from(expression: &Option<Expression>) -> Value {
        Value::Null
    }
}

#[derive(Debug, PartialEq)]
pub enum SymbolKind {
    Variable {
        is_const: bool,
        type_: Rc<Type>,
        value: Value,
    },
    Function {
        parameters: Vec<Symbol>,
        return_type: Rc<Type>,
    },
    Parameter {
        is_const: bool,
        type_: Rc<Type>,
    },
}

impl SymbolKind {
    fn get_type(&self) -> &Rc<Type> {
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
    pub fn variable<'a>(is_const: bool, type_: &'a Type, name: SourceLiteral, value: &'a Option<Expression>) -> Symbol {
        Symbol {
            name,
            kind: SymbolKind::Variable {
                is_const,
                type_: Rc::new(type_.clone()),
                value: Value::from(value),
            },
        }
    }

    pub fn is_variable(&self) -> bool {
        matches!(self.kind, SymbolKind::Variable { .. })
    }

    pub fn parameter<'a>(parameter: &'a Parameter) -> Symbol {
        Symbol {
            name: parameter.name,
            kind: SymbolKind::Parameter {
                is_const: parameter.is_const,
                type_: Rc::new(parameter.type_.clone()),
            }
        }
    }

    pub fn is_parameter(&self) -> bool {
        matches!(self.kind, SymbolKind::Parameter { .. })
    }

    pub fn function<'a>(name: SourceLiteral, parameters: &'a Vec<Parameter>, type_: &'a Type) -> Symbol {
        let mut parameters_symbols: Vec<Symbol> = Vec::new();
        for parameter in parameters.iter() {
            parameters_symbols.push(Symbol::parameter(parameter));
        }   

        Symbol {
            name,
            kind: SymbolKind::Function { 
                parameters: parameters_symbols, 
                return_type: Rc::new(type_.clone()) 
            }
        }
    }

    pub fn is_function(&self) -> bool {
        matches!(self.kind, SymbolKind::Function { .. })
    }

    pub fn get_type(&self) -> &Type {
        self.kind.get_type().deref()
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

    pub fn declare(&mut self,symbol: Symbol) {
        self.symbols.last_mut().unwrap().insert(symbol.name, symbol);
    }

    pub fn is_declared(&self, name: SourceLiteral) -> bool {
        for symbols in self.symbols.iter().rev() {
            if symbols.contains_key(&name) {
                return true;
            }
        }
        return false;
    }

    pub fn is_redeclared(&self, name: SourceLiteral) -> bool {
        self.symbols.last().unwrap().contains_key(&name)
    }

    pub fn find(&self, name: SourceLiteral) -> Option<&Symbol> {
        for symbols in self.symbols.iter().rev() {
            if let Some(symbol) = symbols.get(&name) {
                return Some(&symbol);
            }
        }
        None
    }
}