use crate::tokenizer::Token;
use crate::tokenizer::TokenKind;
use crate::error::*;
use crate::ast::*;
use crate::interner::*;
use crate::scope::*;

pub struct SemanticError {
    code: ErrorCode
}

impl SemanticError {
    pub fn to_diagnostic(&self, filename: &str) -> Diagnostic {
        todo!();
    }
}

pub struct SemanticAnalyzer<'a> {
    ast: &'a Vec<Statement>,
    scope: Scope<'a>,

    return_type: Option<Type>,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(ast: &'a Vec<Statement>) -> SemanticAnalyzer<'a> {
        SemanticAnalyzer {
            ast,
            scope: Scope::new(),
            return_type: None
        }
    }

    pub fn analyze(&mut self) -> Result<(), SemanticError> {
        for statement in self.ast.iter() {
            self.analyze_statement(statement)?;
        }

        Ok(())
    }

    fn analyze_statement(&mut self, statement: &'a Statement) -> Result<(), SemanticError> {
        match &statement.node {
            RawStatement::VariableDeclaration { is_const, type_, name, value } => {
                self.analyze_variable_declaration(*is_const, type_, *name, value)?;
            },

            RawStatement::VariableAssignment { name, operator: _, value } => {
                self.analyze_variable_assignment(*name, value)?;
            },

            RawStatement::If { condition, body, elses } => {
                self.analyze_if(condition, body, elses)?;
            },

            RawStatement::While { condition, body } => {
                self.analyze_while(condition, body)?;
            },
            
            RawStatement::LoopControl(_) => {
                self.analyze_loop_control()?;
            },

            RawStatement::Function { name, parameters, type_, body } => {
                self.analyze_function(*name, parameters, type_, body)?;
            },

            RawStatement::Return(value) => {
                self.analyze_return(value)?;
            },

            RawStatement::FunctionCall { name, arguments } => {
                // self.analyze_function_call_statement(name, arguments)?;
            }

        }

        Ok(())
    }

    fn analyze_variable_declaration(&mut self, is_const: bool, type_: &'a Type, name: SourceLiteral, value: &'a Option<Expression>) -> Result<(), SemanticError> {
        if let Some(value) = value {
            let value_type: Type = self.analyze_expression(value)?;

            if !Type::are_equal(type_, &value_type) {
                return Err(self.error(ErrorCode::ES000));
            }
        }

        self.scope.declare(name, Symbol { 
            is_const: Some(is_const),
            name,
            type_, 
            kind: 
            SymbolKind::Variable, 
            value
        });

        Ok(())
    }

    fn analyze_variable_assignment(&mut self, name: SourceLiteral, value: &Expression) -> Result<(), SemanticError> {
        let type_: &Type = match self.scope.find(name) {
            Some(symbol) => {
                if symbol.kind != SymbolKind::Variable {
                    return Err(self.error(ErrorCode::ES000));
                }

                symbol.type_
            },
            None => return Err(self.error(ErrorCode::ES000))
        };

        let value_type: Type = self.analyze_expression(value)?;

        if !Type::are_equal(type_, &value_type) {
            return Err(self.error(ErrorCode::ES000));
        }

        Ok(())
    }

    fn analyze_condition(&mut self, condition: &Expression) -> Result<Type, SemanticError> {
        let condition_type: Type = self.analyze_expression(condition)?;

        if Type::is_boolean(&condition_type) {
            return Err(self.error(ErrorCode::ES000));
        }

        Ok(condition_type)
    }

    fn analyze_body(&mut self, body: &'a Body, scope_kind: ScopeKind, to_declare: Option<&'a Vec<Parameter>>) -> Result<Option<Type>, SemanticError> {
        self.scope.enter(scope_kind);

        if matches!(scope_kind, ScopeKind::Function) {
            if let Some(parameters) = to_declare {
                for parameter in parameters.iter() {
                    self.scope.declare(parameter.name, Symbol {
                        is_const: Some(parameter.is_const),
                        name: parameter.name,
                        type_: &parameter.type_,
                        kind: SymbolKind::Parameter,
                        value: &None
                    });
                }
            }
        }

        for statement in body.statements.iter() {
            self.analyze_statement(statement)?;
        }

        self.scope.exit();

        Ok(Some(Type::null()))
    }

    fn analyze_if(&mut self, condition: &Expression, body: &'a Body, elses: &'a Vec<ElseBranch>) -> Result<(), SemanticError> {
        self.analyze_condition(condition)?;
        self.analyze_body(body, ScopeKind::If, None)?;

        if elses.len() > 0 {
            for else_branch in elses.iter() {
                match else_branch {
                    ElseBranch::ElseIf(if_statement) => {
                        match &if_statement.node {
                            RawStatement::If { condition, body, elses } => {
                                self.analyze_if(condition, body, elses)?
                            }

                            _ => ()
                        }
                    },
                    ElseBranch::Else(body) => {
                        for statement in body.statements.iter() {
                            self.analyze_statement(statement)?;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    fn analyze_while(&mut self, condition: &Expression, body: &'a Body) -> Result<(), SemanticError> {
        self.analyze_condition(condition)?;
        self.analyze_body(body, ScopeKind::While, None)?;

        Ok(())
    }

    fn analyze_loop_control(&mut self) -> Result<(), SemanticError> {
        for scope in self.scope.loop_depth.iter().rev() {
            if *scope { // If this is set to true, this is a while loop
                return Ok(());
            }
        }

        return Err(self.error(ErrorCode::ES000));
    }

    fn analyze_function(&mut self, name: SourceLiteral, parameters: &'a Vec<Parameter>, type_: &'a Type, body: &'a Body) -> Result<(), SemanticError> {        
        self.scope.declare(name, Symbol {
            is_const: None,
            name,
            type_,
            kind: SymbolKind::Function,
            value: &None
        });

        self.analyze_body(body, ScopeKind::Function, Some(parameters))?;

        if type_.kind != TokenKind::Null {
            if let Some(return_type) = &self.return_type {
                if !Type::are_equal(&return_type, type_) {
                    return Err(self.error(ErrorCode::ES000));
                }
            }
            else {
                return Err(self.error(ErrorCode::ES000));
            }
        }
        
        Ok(())
    }

    fn analyze_return(&mut self, value: &Option<Expression>) -> Result<(), SemanticError> {
        if let Some(value) = value {
            let type_: Type = self.analyze_expression(value)?;
            self.return_type = Some(type_);
        }
        else {
            self.return_type = Some(Type::null());
        }


        Ok(())
    }

    fn analyze_expression(&self, expression: &Expression) -> Result<Type, SemanticError> {
        // TODO: Check if the default size is big enough
        match &expression.node {
            RawExpression::Literal { kind, value: _ } => {
                match kind {
                    TokenKind::IntegerLiteral => Ok(Type::integer()),
                    TokenKind::FloatLiteral => Ok(Type::float()),
                    TokenKind::BooleanLiteral => Ok(Type::boolean()),
                    TokenKind::CharLiteral => Ok(Type::character()),
                    TokenKind::StringLiteral => Ok(Type::string()),
                    _ => unreachable!("Invalid literal type")
                }
            },

            RawExpression::Binary { left, operator, right } => {
                self.analyze_binary_op(left, operator, right)
            },

            RawExpression::Unary { operator, operand } => {
                self.analyze_unary_op(operator, operand)
            },

            RawExpression::Cast { kind, expression } => {
                self.analyze_cast(kind, expression)
            },

            RawExpression::Variable(name) => {
                self.analyze_variable(name)
            },

            RawExpression::FunctionCall { name , arguments} => {
                self.analyze_function_call(name, arguments)
            },

            // RawExpression::ArrayAccess { name, index } => {
            //     self.analyze_array_access(name, index)
            // }

            _ => todo!()
        }

    }

    fn analyze_binary_op(&self, left: &Expression, operator: &TokenKind, right: &Expression) -> Result<Type, SemanticError> {
        let left_type: Type = self.analyze_expression(left)?;
        let right_type: Type = self.analyze_expression(right)?;

        if left_type != right_type {
            return Err(self.error(ErrorCode::ES000));
        }

        if self.is_comparison_operator(operator) {
            return Ok(Type::boolean());
        }

        Ok(left_type)
    }

    fn analyze_unary_op(&self, operator: &TokenKind, operand: &Expression) -> Result<Type, SemanticError> { 
        let operand_type: Type = self.analyze_expression(operand)?;

        if operator == &TokenKind::Not && !Type::is_boolean(&operand_type) {
            return Err(self.error(ErrorCode::ES000));
        }

        if operator == &TokenKind::Minus && !Type::is_numeric(&operand_type) {
            return Err(self.error(ErrorCode::ES000));
        }

        Ok(operand_type)
    } 

    fn analyze_cast(&self, kind: &TokenKind, expression: &Expression) -> Result<Type, SemanticError> {
        let expression_type: Type = self.analyze_expression(expression)?;

        if kind == &TokenKind::String && Type::is_numeric(&expression_type) {
            return Err(self.error(ErrorCode::ES000));
        }
        
        Ok(Type::from(*kind))
    }

    fn analyze_variable(&self, name: &SourceLiteral) -> Result<Type, SemanticError> {
        let variable: &Symbol = match self.scope.find(*name) {
            Some(symbol) => symbol,
            None => return Err(self.error(ErrorCode::ES000))
        };

        if !matches!(variable.kind, SymbolKind::Variable | SymbolKind::Parameter)  {
            return Err(self.error(ErrorCode::ES000));
        }

        Ok(Type::from(variable.type_.kind))
    }

    fn analyze_function_call(&self, name: &SourceLiteral, arguments: &Vec<Expression>) -> Result<Type, SemanticError> {
        for argument in arguments.iter() {
            self.analyze_expression(argument)?;
        }

        let function: &Symbol = match self.scope.find(*name) {
            Some(symbol) => symbol,
            None => return Err(self.error(ErrorCode::ES000))
        };

        if function.kind != SymbolKind::Function  {
            return Err(self.error(ErrorCode::ES000));
        }

        Ok(Type::from(function.type_.kind))
    }

    fn analyze_array_access(&self, name: &Expression, index: &Expression) -> Result<Type, SemanticError> {
        todo!();
    }

    fn is_comparison_operator(&self, operator: &TokenKind) -> bool {
        match operator {
            TokenKind::Equal | TokenKind::NotEqual | TokenKind::LessThan | TokenKind::GreaterThan |
            TokenKind::LessThanOrEqual | TokenKind::GreaterThanOrEqual => true,
            _ => false
        }
    }

    fn error(&self, code: ErrorCode) -> SemanticError {
        SemanticError {
            code
        }
    }
}
