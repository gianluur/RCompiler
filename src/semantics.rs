use std::collections::HashMap;
use crate::parser::{Parameter, Parser, ParserError, RawStatement, Statement};

pub enum Symbol<'a> {
    Statement(&'a Statement<'a>),
    Parameter(&'a Parameter<'a>)
}

pub struct Scope<'a> {
    symbols: Vec<HashMap<&'a str, Symbol<'a>>>,
    scope: usize
}

impl<'a> Scope<'a> {
    pub fn new() -> Scope<'a> {
        Scope {
            symbols: vec![HashMap::new()],
            scope: 0
        }
    }

    pub fn enter(&mut self) {
        self.symbols.push(HashMap::new());
        self.scope += 1;
    }

    pub fn exit(&mut self) {
        self.symbols.pop();
        self.scope -= 1;
    }

    pub fn declare(&mut self, name: &'a str, symbol: Symbol<'a>) {
        self.symbols.last_mut().unwrap().insert(name, symbol);
    }

    pub fn is_declared(&mut self, name: &'a str) -> bool {
        for symbols in self.symbols.iter().rev() {
            if symbols.contains_key(name) {
                return true;
            }
        }
        return false;
    }

    pub fn is_redeclared(&mut self, name: &'a str) -> bool {
        self.symbols.last().unwrap().contains_key(name)
    }

    pub fn get(&mut self, name: &'a str) -> Option<&Symbol<'a>> {
        for symbols in self.symbols.iter().rev() {
            if let Some(symbol) = symbols.get(name) {
                return Some(&symbol);
            }
        }
        None
    }
}

pub struct SemanticAnalyzer<'a> {
    ast: Vec<Statement<'a>>
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(ast: Vec<Statement<'a>>) -> SemanticAnalyzer<'a> {
        SemanticAnalyzer {
            ast
        }
    }

    pub fn analyze_statement(statement: &Statement) -> Result<(), ParserError> {
        match &statement.node {
            RawStatement::VariableDeclaration { is_const, type_, name, value } => {

            },

            RawStatement::VariableAssignment { name, operator, value } => {

            },

            RawStatement::If { condition, body, elses } => {

            },

            RawStatement::While { condition, body } => {

            },
            
            RawStatement::LoopControl(keyword) => {

            },

            RawStatement::Function { name, parameters, type_, body } => {

            },

            RawStatement::Return(value) => {

            },

            RawStatement::FunctionCall { name, arguments } => {

            }

        }

        Ok(())
    }
}
