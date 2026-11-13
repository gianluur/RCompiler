use crate::ast::*;
use crate::error::*;
use crate::interner::*;
use crate::scope::*;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Termination {
    None,    // The block doesn't terminate the function (keeps going)
    Returns, // The block terminates the function via return
}

pub struct SemanticError {
    code: ErrorCode,
}

impl SemanticError {
    pub fn to_diagnostic(&self, filename: &str) -> Diagnostic {
        todo!("Error");
    }
}

pub struct SemanticAnalyzer<'a> {
    scope: Scope,
    interner: &'a mut Interner,

    function_type: Option<Type>,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(interner: &'a mut Interner) -> SemanticAnalyzer<'a> {
        SemanticAnalyzer {
            scope: Scope::new(),
            interner,
            function_type: None,
        }
    }

    pub fn analyze(&mut self, ast: Vec<Statement>) -> Result<(), SemanticError> {
        for statement in ast {
            self.analyze_statement(statement)?;
        }

        Ok(())
    }

    fn analyze_statement(&mut self, statement: Statement) -> Result<Termination, SemanticError> {
        use RawStatement::*;

        let termination: Termination = match statement.node {
            VariableDeclaration {
                is_const,
                type_,
                name,
                value,
            } => self.analyze_variable_declaration(is_const, type_, name, value)?,

            VariableAssignment {
                name,
                operator: _,
                value,
            } => self.analyze_variable_assignment(&name, value)?,

            If {
                condition,
                body,
                elses,
            } => self.analyze_if(condition, body, elses)?,

            While { condition, body } => self.analyze_while(condition, body)?,

            Break | Continue => self.analyze_loop_control()?,

            Function {
                name,
                parameters,
                type_,
                body,
            } => self.analyze_function(name, parameters, type_, body)?,

            Return(value) => self.analyze_return(value)?,

            FunctionCall { name, arguments } => {
                self.analyze_function_call(&name, &arguments)?;
                return Ok(Termination::None);
            }
        };

        Ok(termination)
    }

    fn analyze_variable_declaration(
        &mut self,
        is_const: bool,
        type_: Type,
        name: SourceLiteral,
        value: Option<Expression>,
    ) -> Result<Termination, SemanticError> {
        if self.scope.is_redeclared(&name) {
            return Err(self.error(ErrorCode::ES001));
        }

        if is_const && value.is_none() {
            return Err(self.error(ErrorCode::ES003));
        }

        if let Some(value) = &value {
            let value_type: Type = self.analyze_expression(value)?;

            if type_ != value_type {
                return Err(self.error(ErrorCode::ES002));
            }
        }

        self.scope
            .declare(Symbol::variable(is_const, type_, name, value));

        Ok(Termination::None)
    }

    fn analyze_variable_assignment(
        &mut self,
        name: &SourceLiteral,
        value: Expression,
    ) -> Result<Termination, SemanticError> {
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
            }
            None => return Err(self.error(ErrorCode::ES000)),
        };

        let value_type: Type = self.analyze_expression(&value)?;

        if *type_ != value_type {
            return Err(self.error(ErrorCode::ES000));
        }

        Ok(Termination::None)
    }

    fn analyze_condition(&mut self, condition: Expression) -> Result<Type, SemanticError> {
        let condition_type: Type = self.analyze_expression(&condition)?;
        if !Type::is_boolean(&condition_type) {
            return Err(self.error(ErrorCode::ES000));
        }

        Ok(condition_type)
    }

    fn declare_parameters(&mut self, to_declare: Option<Vec<Parameter>>) {
        if let Some(parameters) = to_declare {
            for parameter in parameters {
                self.scope.declare(Symbol::parameter(parameter));
            }
        }
    }

    fn analyze_body(
        &mut self,
        body: Body,
        scope_kind: ScopeKind,
        to_declare: Option<Vec<Parameter>>,
    ) -> Result<Termination, SemanticError> {
        self.scope.enter(scope_kind);

        if matches!(scope_kind, ScopeKind::Function) {
            self.declare_parameters(to_declare);
        }

        // Helps to figure out if there's a return statement in the body
        let mut body_termination: Termination = Termination::None;

        for statement in body.statements {
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

    fn analyze_if(
        &mut self,
        condition: Expression,
        body: Body,
        elses: Vec<ElseBranch>,
    ) -> Result<Termination, SemanticError> {
        self.analyze_condition(condition)?;
        let if_termination: Termination = self.analyze_body(body, ScopeKind::If, None)?;

        if elses.is_empty() {
            return Ok(Termination::None);
        }

        let mut is_exhaustive: bool = if_termination == Termination::Returns;

        for branch in elses {
            let branch_termination: Termination = match branch {
                ElseBranch::ElseIf(if_statement) => self.analyze_statement(if_statement)?,
                ElseBranch::Else(body) => self.analyze_body(body, ScopeKind::Else, None)?,
            };

            if branch_termination == Termination::None {
                is_exhaustive = false;
            }
        }

        if is_exhaustive {
            Ok(Termination::Returns)
        } else {
            Ok(Termination::None)
        }
    }

    fn analyze_while(
        &mut self,
        condition: Expression,
        body: Body,
    ) -> Result<Termination, SemanticError> {
        self.analyze_condition(condition)?;
        self.analyze_body(body, ScopeKind::While, None)?;

        Ok(Termination::None)
    }

    fn analyze_loop_control(&mut self) -> Result<Termination, SemanticError> {
        for scope in self.scope.loop_depth.iter().rev() {
            if *scope {
                // If this is set to true, this is a while loop
                return Ok(Termination::None);
            }
        }

        return Err(self.error(ErrorCode::ES000));
    }

    fn analyze_function(
        &mut self,
        name: SourceLiteral,
        parameters: Vec<Parameter>,
        type_: Type,
        body: Body,
    ) -> Result<Termination, SemanticError> {
        if !self.scope.is_redeclared(&name) {
            self.scope
                .declare(Symbol::function(name, parameters.clone(), type_.clone()));
        }

        self.function_type = Some(type_.clone());
        let termination_status = self.analyze_body(body, ScopeKind::Function, Some(parameters))?;

        if !type_.is_null() {
            if termination_status != Termination::Returns {
                return Err(self.error(ErrorCode::ES000));
            }

            self.function_type = None;
        }

        Ok(Termination::None)
    }

    fn analyze_return(&mut self, value: Option<Expression>) -> Result<Termination, SemanticError> {
        if let Some(value) = &value {
            let return_type: Type = self.analyze_expression(value)?;

            if let Some(function_type) = &self.function_type {
                if function_type != &return_type {
                    return Err(self.error(ErrorCode::ES000));
                }
            } else {
                return Err(self.error(ErrorCode::ES000));
            }
        }

        Ok(Termination::Returns)
    }

    fn analyze_expression(&self, expression: &Expression) -> Result<Type, SemanticError> {
        use Literal::*;

        // TODO: Check if the default size is big enough
        match &expression.node {
            RawExpression::Literal { kind, value } => match kind {
                Integer => Ok(Type::i32()),
                Float => Ok(Type::f32()),
                Boolean => Ok(Type::bool()),
                Char => Ok(Type::char()),
                String => Ok(Type::string()),
            },

            RawExpression::Binary {
                left,
                operator,
                right,
            } => self.analyze_binary_op(left, operator, right),

            RawExpression::Unary { operator, operand } => self.analyze_unary_op(operator, operand),

            RawExpression::Cast { kind, expression } => self.analyze_cast(kind, expression),

            RawExpression::Variable(name) => self.analyze_variable(name),

            RawExpression::FunctionCall { name, arguments } => {
                self.analyze_function_call(name, arguments)
            }

            RawExpression::ArrayAccess { name, index } => self.analyze_array_access(name, index),

            RawExpression::InitializerList {
                list,
                expected_type,
            } => self.analyze_initializer_list(list, expected_type),
        }
    }

    // fn analyze_integer(&self, value: &SourceLiteral) -> Result<Type, SemanticError> {
    //     let value_str = self.interner.lookup(*value);
    //     if value_str.starts_with('-') {
    //     } else {
    //     }

    //     return Ok(Type::i32());
    // }

    fn analyze_binary_op(
        &self,
        left: &Expression,
        operator: &Operator,
        right: &Expression,
    ) -> Result<Type, SemanticError> {
        let left_type: Type = self.analyze_expression(left)?;
        let right_type: Type = self.analyze_expression(right)?;

        if left_type != right_type {
            return Err(self.error(ErrorCode::ES000));
        }

        if operator.is_comparison() || operator.is_logic() {
            return Ok(Type::bool());
        }

        Ok(left_type)
    }

    fn analyze_unary_op(
        &self,
        operator: &Operator,
        operand: &Expression,
    ) -> Result<Type, SemanticError> {
        let operand_type: Type = self.analyze_expression(operand)?;

        if operator == &Operator::Not && !Type::is_boolean(&operand_type) {
            return Err(self.error(ErrorCode::ES000));
        }

        // Operator::Subtraction is ambiguous, it just refers to the '-' token
        if operator == &Operator::Subtraction && !Type::is_numeric(&operand_type) {
            return Err(self.error(ErrorCode::ES000));
        }

        Ok(operand_type)
    }

    fn analyze_cast(&self, kind: &Type, expression: &Expression) -> Result<Type, SemanticError> {
        let expression_type: Type = self.analyze_expression(expression)?;

        if kind == &Type::String && Type::is_numeric(&expression_type) {
            return Err(self.error(ErrorCode::ES000));
        }

        Ok(kind.clone())
    }

    fn analyze_variable(&self, name: &SourceLiteral) -> Result<Type, SemanticError> {
        let variable: &Symbol = match self.scope.find(name) {
            Some(symbol) => symbol,
            None => return Err(self.error(ErrorCode::ES000)),
        };

        if !variable.is_variable() && !variable.is_parameter() {
            return Err(self.error(ErrorCode::ES000));
        }

        Ok(variable.get_type().clone())
    }

    fn analyze_function_call(
        &self,
        name: &SourceLiteral,
        arguments: &Vec<Expression>,
    ) -> Result<Type, SemanticError> {
        let function: &Symbol = match self.scope.find(&name) {
            Some(symbol) => symbol,
            None => return Err(self.error(ErrorCode::ES000)),
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

            if &argument_type != parameter.get_type() {
                return Err(self.error(ErrorCode::ES000));
            }
        }

        Ok(function.get_type().clone())
    }

    fn analyze_array_access(
        &self,
        name: &Expression,
        index: &Expression,
    ) -> Result<Type, SemanticError> {
        let index_type: Type = self.analyze_expression(index)?;
        if !index_type.is_integer() {
            return Err(self.error(ErrorCode::ES000));
        }

        let container_type: Type = self.analyze_expression(name)?;

        if let Type::Array { kind, .. } = container_type {
            Ok(*kind)
        } else {
            Err(self.error(ErrorCode::ES000))
        }
    }

    fn analyze_initializer_list(
        &self,
        list: &Vec<Expression>,
        expected_type: &Option<Type>,
    ) -> Result<Type, SemanticError> {
        if let Some(expected_type) = expected_type {
            for element in list {
                let element_type = self.analyze_expression(element)?;
                if &element_type != expected_type {
                    return Err(self.error(ErrorCode::ES000));
                }
            }

            Ok(Type::array(expected_type.clone(), None))
        } else {
            // Ideally this error is impossibile because it means the parser
            // didn't have a value for the initializer list, but this is possibilee
            // only at the start of the compiler becuase i gave the initial value as None
            return Err(self.error(ErrorCode::ES000));
        }
    }

    fn error(&self, code: ErrorCode) -> SemanticError {
        SemanticError { code }
    }
}
