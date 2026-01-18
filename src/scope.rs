use crate::interner::SourceLiteral;
use crate::ast::{Expression, Type};

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ScopeKind {
    Global,
    If, 
    While,
    Function,
}

#[derive(Debug, PartialEq)]
pub enum SymbolKind {
    Variable,
    Function,
    Parameter,
}

#[derive(Debug, PartialEq)]
pub struct Symbol<'a> {
    pub is_const: Option<bool>,
    pub kind: SymbolKind,
    pub name: SourceLiteral,
    pub type_: &'a Type,
    pub value: &'a Option<Expression>,
}

pub struct Scope<'a> {
    pub kind: ScopeKind,
    pub symbols: Vec<HashMap<SourceLiteral, Symbol<'a>>>,
    pub scope: usize,
    pub loop_depth: Vec<bool>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Scope<'a> {
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

    pub fn declare(&mut self, name: SourceLiteral, symbol: Symbol<'a>) {
        self.symbols.last_mut().unwrap().insert(name, symbol);
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

    pub fn find(&self, name: SourceLiteral) -> Option<&Symbol<'a>> {
        for symbols in self.symbols.iter().rev() {
            if let Some(symbol) = symbols.get(&name) {
                return Some(&symbol);
            }
        }
        None
    }
}