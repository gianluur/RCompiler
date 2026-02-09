use crate::tokenizer::TokenKind;
use crate::error::*;
use crate::ast::*;
use crate::interner::*;
use crate::scope::*;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Termination {
    None,      // The block doesn't terminate the function (keeps going)
    Returns,   // The block terminates the function via return
}

pub struct SemanticError {
    code: ErrorCode
}

impl SemanticError {
    pub fn to_diagnostic(&self, filename: &str) -> Diagnostic {
        todo!("Error");
    }
}

pub struct SemanticAnalyzer<'a> {
    ast: &'a Vec<Statement>,
    scope: Scope,

    function_type: Option<Type>,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(ast: &'a Vec<Statement>) -> SemanticAnalyzer<'a> {
        SemanticAnalyzer {
            ast,
            scope: Scope::new(),
            function_type: None,
        }
    }

    pub fn analyze(&mut self) -> Result<(), SemanticError> {
        for statement in self.ast.iter() {
            self.analyze_statement(statement)?;
        }

        Ok(())
    }

    fn analyze_statement(&mut self, statement: &'a Statement) -> Result<Termination, SemanticError> {
        let termination: Termination = match &statement.node {
            RawStatement::VariableDeclaration { is_const, type_, name, value } => {
                self.analyze_variable_declaration(*is_const, type_, *name, value)?
            },

            RawStatement::VariableAssignment { name, operator: _, value } => {
                self.analyze_variable_assignment(*name, value)?
            },

            RawStatement::If { condition, body, elses } => {
                self.analyze_if(condition, body, elses)?
            },

            RawStatement::While { condition, body } => {
                self.analyze_while(condition, body)?
            },
            
            RawStatement::LoopControl(_) => {
                self.analyze_loop_control()?
            },

            RawStatement::Function { name, parameters, type_, body } => {
                self.analyze_function(*name, parameters, type_, body)?
            },

            RawStatement::Return(value) => {
                self.analyze_return(value)?
            },

            RawStatement::FunctionCall { name, arguments } => {
                self.analyze_function_call(name, arguments)?;
                return Ok(Termination::None)
            }
        };

        Ok(termination)

    }

    fn analyze_variable_declaration(&mut self, is_const: bool, type_: &'a Type, name: SourceLiteral, value: &'a Option<Expression>) -> Result<Termination, SemanticError> {
        if self.scope.is_redeclared(name) {
            return Err(self.error(ErrorCode::ES000));
        }

        if let Some(value) = value {
            let value_type: Type = self.analyze_expression(value)?;

            if !Type::are_equal(type_, &value_type) {
                return Err(self.error(ErrorCode::ES000));
            }
        }

        if is_const && value.is_none() {
            return Err(self.error(ErrorCode::ES000));
        }

        self.scope.declare(Symbol::variable(is_const, type_, name, value));

        Ok(Termination::None)
    }

    fn analyze_variable_assignment(&mut self, name: SourceLiteral, value: &Expression) -> Result<Termination, SemanticError> {
        if !self.scope.is_declared(name) {
            return Err(self.error(ErrorCode::ES000));
        }
        
        let type_: &Type = match self.scope.find(name) {
            Some(symbol) => {
                if !symbol.is_variable() {
                    return Err(self.error(ErrorCode::ES000));
                }

                if symbol.is_const() {
                    return Err(self.error(ErrorCode::ES000));
                }

                symbol.get_type()
            },
            None => return Err(self.error(ErrorCode::ES000))
        };

        let value_type: Type = self.analyze_expression(value)?;

        if !Type::are_equal(type_, &value_type) {
            return Err(self.error(ErrorCode::ES000));
        }

        Ok(Termination::None)
    }

    fn analyze_condition(&mut self, condition: &Expression) -> Result<Type, SemanticError> {
        let condition_type: Type = self.analyze_expression(condition)?;
        if !Type::is_boolean(&condition_type) {
            return Err(self.error(ErrorCode::ES000));
        }

        Ok(condition_type)
    }

    fn declare_parameters(&mut self, to_declare: Option<&'a Vec<Parameter>>) {
        if let Some(parameters) = to_declare {
            for parameter in parameters.iter() {
                self.scope.declare(
                    Symbol::parameter(parameter)
                );
            }
        }
    }

    fn analyze_body(&mut self, body: &'a Body, scope_kind: ScopeKind, to_declare: Option<&'a Vec<Parameter>>) -> Result<Termination, SemanticError> {
        self.scope.enter(scope_kind);

        if matches!(scope_kind, ScopeKind::Function) {
            self.declare_parameters(to_declare);
        }
        
        // Helps to figure out if there's a return statement in the body
        let mut body_termination: Termination = Termination::None;
        
        for statement in body.statements.iter() {
            if body_termination != Termination::None {
                return Err(self.error(ErrorCode::ES000)); // Unreachable code
            }

            if self.analyze_statement(statement)? == Termination::Returns {
                body_termination = Termination::Returns;
            }
        }

        self.scope.exit();

        Ok(body_termination)
    }

    fn analyze_if(&mut self, condition: &Expression, body: &'a Body, elses: &'a Vec<ElseBranch>) -> Result<Termination, SemanticError> {
        self.analyze_condition(condition)?;
        let if_termination: Termination = self.analyze_body(body, ScopeKind::If, None)?;

        if elses.is_empty() {
            return Ok(Termination::None);
        }

        let mut is_exhaustive: bool = if_termination == Termination::Returns;

        for branch in elses.iter() {
            let branch_termination: Termination = match branch {
                ElseBranch::ElseIf(if_statement) => {
                    self.analyze_statement(if_statement)?
                },
                ElseBranch::Else(body) => {
                    self.analyze_body(body, ScopeKind::Else, None)?
                }
            };

            if branch_termination == Termination::None {
                is_exhaustive = false;
            }
        }
        
        if is_exhaustive {
            Ok(Termination::Returns)
        }
        else {
            Ok(Termination::None)
        }
    }

    fn analyze_while(&mut self, condition: &Expression, body: &'a Body) -> Result<Termination, SemanticError> {
        self.analyze_condition(condition)?;
        self.analyze_body(body, ScopeKind::While, None)?;

        Ok(Termination::None)
    }

    fn analyze_loop_control(&mut self) -> Result<Termination, SemanticError> {
        for scope in self.scope.loop_depth.iter().rev() {
            if *scope { // If this is set to true, this is a while loop
                return Ok(Termination::None);
            }
        }

        return Err(self.error(ErrorCode::ES000));
    }

    fn analyze_function(&mut self, name: SourceLiteral, parameters: &'a Vec<Parameter>, type_: &'a Type, body: &'a Body) -> Result<Termination, SemanticError> {        
        if !self.scope.is_redeclared(name) {
            self.scope.declare(Symbol::function(name, parameters, type_));
        }

        self.function_type = Some(type_.clone());
        let termination_status: Termination = self.analyze_body(body, ScopeKind::Function, Some(parameters))?;

        if !type_.is_null() {
            if termination_status != Termination::Returns {
                return Err(self.error(ErrorCode::ES000));
            }

            self.function_type = None;
        }
        
        Ok(Termination::None)
    }

    fn analyze_return(&mut self, value: &Option<Expression>) -> Result<Termination, SemanticError> {
        if let Some(value) = value {
            let return_type_: Type = self.analyze_expression(value)?;

            if let Some(function_type) = &self.function_type {
                if !Type::are_equal(function_type, &return_type_) {
                    return Err(self.error(ErrorCode::ES000));
                }
            }
            else {
                return Err(self.error(ErrorCode::ES000));
            }
        }

        Ok(Termination::Returns)
    }

    fn analyze_expression(&self, expression: &Expression) -> Result<Type, SemanticError> {
        // TODO: Check if the default size is big enough
        match &expression.node {
            RawExpression::Literal { kind, value: _ } => {
                match kind {
                    TokenKind::IntegerLiteral => Ok(Type::i32()),
                    TokenKind::FloatLiteral => Ok(Type::f32()),
                    TokenKind::BooleanLiteral => Ok(Type::bool()),
                    TokenKind::CharLiteral => Ok(Type::char()),
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

            RawExpression::ArrayAccess { name, index } => {
                self.analyze_array_access(name, index)
            },

            RawExpression::InitializerList{ list, expected_type}=> {
                self.analyze_initializer_list(list, expected_type.clone())
            }
        }
    }

    fn analyze_binary_op(&self, left: &Expression, operator: &TokenKind, right: &Expression) -> Result<Type, SemanticError> {
        let left_type: Type = self.analyze_expression(left)?;
        let right_type: Type = self.analyze_expression(right)?;

        if left_type != right_type {
            return Err(self.error(ErrorCode::ES000));
        }

        if self.is_comparison_operator(operator) || 
            self.is_logical_operator(operator) {
            return Ok(Type::bool());
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

        if !variable.is_variable() && !variable.is_parameter() {
            return Err(self.error(ErrorCode::ES000));
        }

        Ok(variable.get_type().clone())
    }

    fn analyze_function_call(&self, name: &SourceLiteral, arguments: &Vec<Expression>) -> Result<Type, SemanticError> { 
        let function: &Symbol = match self.scope.find(*name) {
            Some(symbol) => symbol,
            None => return Err(self.error(ErrorCode::ES000))
        };
        
        if !function.is_function() {
            return Err(self.error(ErrorCode::ES000));
        }

        let parameters: &Vec<Symbol> = function.get_parameters().unwrap();

        if parameters.len() != arguments.len() {
            return Err(self.error(ErrorCode::ES000));
        }

        for (parameter, argument) in parameters.iter().zip(arguments.iter()) {
            let argument_type: Type = self.analyze_expression(argument)?;
            
            if !Type::are_equal(&argument_type, parameter.get_type()) {
                return Err(self.error(ErrorCode::ES000));
            }
        }
        
        Ok(function.get_type().clone())
    }

    fn analyze_array_access(&self, name: &Expression, index: &Expression) -> Result<Type, SemanticError> {
        let index_type: Type = self.analyze_expression(index)?;
        if !index_type.is_integer() {
            return Err(self.error(ErrorCode::ES000));
        }

        let container_type: Type = self.analyze_expression(name)?;

        if let Type::Array { kind, .. } = container_type {
            Ok(*kind) 
        } 
        else {
            Err(self.error(ErrorCode::ES000))
        }

    }

    fn analyze_initializer_list(&self, list: &Vec<Expression>, expected_type: Option<Type>) -> Result<Type, SemanticError> {
        if expected_type.is_none() {
            return Err(self.error(ErrorCode::ES000));
        }
        
        for element in list.iter() {
            let element_type: Type = self.analyze_expression(element)?;
            if !Type::are_equal(&element_type, &expected_type.as_ref().unwrap()) {
                return Err(self.error(ErrorCode::ES000));
            }
        }

        Ok(Type::array(expected_type.unwrap(), None))
    }

    fn is_comparison_operator(&self, operator: &TokenKind) -> bool {
        match operator {
            TokenKind::Equal | TokenKind::NotEqual | TokenKind::LessThan | TokenKind::GreaterThan |
            TokenKind::LessThanOrEqual | TokenKind::GreaterThanOrEqual => true,
            _ => false
        }
    }

    fn is_logical_operator(&self, operator: &TokenKind) -> bool {
        match operator {
            TokenKind::And | TokenKind::Or => true,
            _ => false
        }
    }

    fn error(&self, code: ErrorCode) -> SemanticError {
        SemanticError {
            code
        }
    }
}
