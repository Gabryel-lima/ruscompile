use std::collections::HashMap;
use crate::ast::*;
use crate::error::{CompilerError, CompilerResult};

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: Type,
    pub is_function: bool,
    pub parameters: Vec<Type>,
    pub return_type: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct Scope {
    symbols: HashMap<String, Symbol>,
    parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Scope) -> Self {
        Self {
            symbols: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn define(&mut self, symbol: Symbol) -> Result<(), CompilerError> {
        if self.symbols.contains_key(&symbol.name) {
            return Err(CompilerError::semantic(
                format!("Símbolo '{}' já está definido", symbol.name),
            ));
        }
        self.symbols.insert(symbol.name.clone(), symbol);
        Ok(())
    }

    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        if let Some(symbol) = self.symbols.get(name) {
            Some(symbol)
        } else if let Some(parent) = &self.parent {
            parent.resolve(name)
        } else {
            None
        }
    }
}

pub struct SemanticAnalyzer {
    current_scope: Scope,
    function_return_type: Option<Type>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            current_scope: Scope::new(),
            function_return_type: None,
        }
    }

    pub fn analyze(&mut self, program: &Program) -> CompilerResult<()> {
        // Definir funções built-in
        self.define_builtins()?;

        // Analisar todas as declarações
        for statement in &program.statements {
            self.analyze_statement(statement)?;
        }

        Ok(())
    }

    fn define_builtins(&mut self) -> CompilerResult<()> {
        // Função print
        self.current_scope.define(Symbol {
            name: "print".to_string(),
            symbol_type: Type::Function {
                parameters: vec![Type::String],
                return_type: Box::new(Type::Void),
            },
            is_function: true,
            parameters: vec![Type::String],
            return_type: Some(Type::Void),
        })?;

        // Função println - sobrecargas para diferentes tipos
        // println(string)
        self.current_scope.define(Symbol {
            name: "println".to_string(),
            symbol_type: Type::Function {
                parameters: vec![Type::String],
                return_type: Box::new(Type::Void),
            },
            is_function: true,
            parameters: vec![Type::String],
            return_type: Some(Type::Void),
        })?;

        // println(int)
        self.current_scope.define(Symbol {
            name: "println_int".to_string(),
            symbol_type: Type::Function {
                parameters: vec![Type::Int],
                return_type: Box::new(Type::Void),
            },
            is_function: true,
            parameters: vec![Type::Int],
            return_type: Some(Type::Void),
        })?;

        // println(float)
        self.current_scope.define(Symbol {
            name: "println_float".to_string(),
            symbol_type: Type::Function {
                parameters: vec![Type::Float],
                return_type: Box::new(Type::Void),
            },
            is_function: true,
            parameters: vec![Type::Float],
            return_type: Some(Type::Void),
        })?;

        // println(bool)
        self.current_scope.define(Symbol {
            name: "println_bool".to_string(),
            symbol_type: Type::Function {
                parameters: vec![Type::Bool],
                return_type: Box::new(Type::Void),
            },
            is_function: true,
            parameters: vec![Type::Bool],
            return_type: Some(Type::Void),
        })?;

        Ok(())
    }

    fn analyze_statement(&mut self, statement: &Statement) -> CompilerResult<()> {
        match statement {
            Statement::Expression(expr_stmt) => {
                self.analyze_expression(&expr_stmt.expression)?;
            }
            Statement::Declaration(decl_stmt) => {
                self.analyze_declaration(decl_stmt)?;
            }
            Statement::Assignment(assign_stmt) => {
                self.analyze_assignment(assign_stmt)?;
            }
            Statement::If(if_stmt) => {
                self.analyze_if_statement(if_stmt)?;
            }
            Statement::While(while_stmt) => {
                self.analyze_while_statement(while_stmt)?;
            }
            Statement::Function(func_stmt) => {
                self.analyze_function_declaration(func_stmt)?;
            }
            Statement::Return(return_stmt) => {
                self.analyze_return_statement(return_stmt)?;
            }
            Statement::Block(block_stmt) => {
                self.analyze_block_statement(block_stmt)?;
            }
        }
        Ok(())
    }

    fn analyze_declaration(&mut self, decl: &DeclarationStatement) -> CompilerResult<()> {
        // Verificar se a variável já foi declarada
        if self.current_scope.resolve(&decl.name).is_some() {
            return Err(CompilerError::semantic_with_location(
                format!("Variável '{}' já foi declarada", decl.name),
                decl.location.line,
                decl.location.column,
            ));
        }

        // Analisar inicializador se presente
        if let Some(initializer) = &decl.initializer {
            let init_type = self.analyze_expression(initializer)?;
            if !self.types_compatible(&decl.var_type, &init_type) {
                return Err(CompilerError::type_error_with_location(
                    format!(
                        "Tipo incompatível: esperado {}, encontrado {}",
                        decl.var_type, init_type
                    ),
                    decl.location.line,
                    decl.location.column,
                ));
            }
        }

        // Definir a variável no escopo atual
        self.current_scope.define(Symbol {
            name: decl.name.clone(),
            symbol_type: decl.var_type.clone(),
            is_function: false,
            parameters: vec![],
            return_type: None,
        })?;

        Ok(())
    }

    fn analyze_assignment(&mut self, assign: &AssignmentStatement) -> CompilerResult<()> {
        // Verificar se a variável existe e obter informações necessárias
        let symbol_info = {
            let symbol = self.current_scope.resolve(&assign.target).ok_or_else(|| {
                CompilerError::semantic_with_location(
                    format!("Variável '{}' não foi declarada", assign.target),
                    assign.location.line,
                    assign.location.column,
                )
            })?;
            
            (symbol.is_function, symbol.symbol_type.clone())
        };

        if symbol_info.0 {
            return Err(CompilerError::semantic_with_location(
                format!("Não é possível atribuir a função '{}'", assign.target),
                assign.location.line,
                assign.location.column,
            ));
        }

        // Analisar o valor da atribuição
        let value_type = self.analyze_expression(&assign.value)?;

        // Verificar compatibilidade de tipos
        if !self.types_compatible(&symbol_info.1, &value_type) {
            return Err(CompilerError::type_error_with_location(
                format!(
                    "Tipo incompatível na atribuição: esperado {}, encontrado {}",
                    symbol_info.1, value_type
                ),
                assign.location.line,
                assign.location.column,
            ));
        }

        Ok(())
    }

    fn analyze_if_statement(&mut self, if_stmt: &IfStatement) -> CompilerResult<()> {
        // Analisar condição
        let condition_type = self.analyze_expression(&if_stmt.condition)?;
        if condition_type != Type::Bool {
            return Err(CompilerError::type_error_with_location(
                format!(
                    "Condição do if deve ser bool, encontrado {}",
                    condition_type
                ),
                if_stmt.location.line,
                if_stmt.location.column,
            ));
        }

        // Analisar ramo then
        self.analyze_statement(&if_stmt.then_branch)?;

        // Analisar ramo else se presente
        if let Some(else_branch) = &if_stmt.else_branch {
            self.analyze_statement(else_branch)?;
        }

        Ok(())
    }

    fn analyze_while_statement(&mut self, while_stmt: &WhileStatement) -> CompilerResult<()> {
        // Analisar condição
        let condition_type = self.analyze_expression(&while_stmt.condition)?;
        if condition_type != Type::Bool {
            return Err(CompilerError::type_error_with_location(
                format!(
                    "Condição do while deve ser bool, encontrado {}",
                    condition_type
                ),
                while_stmt.location.line,
                while_stmt.location.column,
            ));
        }

        // Analisar corpo do loop
        self.analyze_statement(&while_stmt.body)?;

        Ok(())
    }

    fn analyze_function_declaration(&mut self, func: &FunctionStatement) -> CompilerResult<()> {
        // Verificar se a função já foi declarada
        if self.current_scope.resolve(&func.name).is_some() {
            return Err(CompilerError::semantic_with_location(
                format!("Função '{}' já foi declarada", func.name),
                func.location.line,
                func.location.column,
            ));
        }

        // Definir a função no escopo atual
        let param_types: Vec<Type> = func.parameters.iter().map(|p| p.param_type.clone()).collect();
        self.current_scope.define(Symbol {
            name: func.name.clone(),
            symbol_type: Type::Function {
                parameters: param_types.clone(),
                return_type: Box::new(func.return_type.clone()),
            },
            is_function: true,
            parameters: param_types,
            return_type: Some(func.return_type.clone()),
        })?;

        // Criar novo escopo para o corpo da função
        let mut function_scope = Scope::with_parent(self.current_scope.clone());

        // Adicionar parâmetros ao escopo da função
        for param in &func.parameters {
            function_scope.define(Symbol {
                name: param.name.clone(),
                symbol_type: param.param_type.clone(),
                is_function: false,
                parameters: vec![],
                return_type: None,
            })?;
        }

        // Analisar corpo da função
        let old_scope = std::mem::replace(&mut self.current_scope, function_scope);
        let old_return_type = self.function_return_type.take();
        self.function_return_type = Some(func.return_type.clone());

        self.analyze_block_statement(&func.body)?;

        // Restaurar escopo anterior
        self.current_scope = old_scope;
        self.function_return_type = old_return_type;

        Ok(())
    }

    fn analyze_return_statement(&mut self, return_stmt: &ReturnStatement) -> CompilerResult<()> {
        let expected_return_type = self.function_return_type.clone().ok_or_else(|| {
            CompilerError::semantic_with_location(
                "Return fora de função".to_string(),
                return_stmt.location.line,
                return_stmt.location.column,
            )
        })?;

        match &return_stmt.value {
            Some(value) => {
                let value_type = self.analyze_expression(value)?;
                if !self.types_compatible(&expected_return_type, &value_type) {
                    return Err(CompilerError::type_error_with_location(
                        format!(
                            "Tipo de retorno incompatível: esperado {}, encontrado {}",
                            expected_return_type, value_type
                        ),
                        return_stmt.location.line,
                        return_stmt.location.column,
                    ));
                }
            }
            None => {
                if expected_return_type != Type::Void {
                    return Err(CompilerError::type_error_with_location(
                        format!(
                            "Função deve retornar {}, mas não há valor de retorno",
                            expected_return_type
                        ),
                        return_stmt.location.line,
                        return_stmt.location.column,
                    ));
                }
            }
        }

        Ok(())
    }

    fn analyze_block_statement(&mut self, block: &BlockStatement) -> CompilerResult<()> {
        // Criar novo escopo para o bloco
        let block_scope = Scope::with_parent(self.current_scope.clone());
        let old_scope = std::mem::replace(&mut self.current_scope, block_scope);

        // Analisar todas as declarações no bloco
        for statement in &block.statements {
            self.analyze_statement(statement)?;
        }

        // Restaurar escopo anterior
        self.current_scope = old_scope;

        Ok(())
    }

    fn analyze_expression(&mut self, expression: &Expression) -> CompilerResult<Type> {
        match expression {
            Expression::Literal(literal_expr) => {
                Ok(self.literal_type(&literal_expr.value))
            }
            Expression::Identifier(identifier_expr) => {
                let symbol = self.current_scope.resolve(&identifier_expr.name).ok_or_else(|| {
                    CompilerError::semantic_with_location(
                        format!("Variável '{}' não foi declarada", identifier_expr.name),
                        identifier_expr.location.line,
                        identifier_expr.location.column,
                    )
                })?;
                Ok(symbol.symbol_type.clone())
            }
            Expression::Binary(binary_expr) => {
                self.analyze_binary_expression(binary_expr)
            }
            Expression::Unary(unary_expr) => {
                self.analyze_unary_expression(unary_expr)
            }
            Expression::Call(call_expr) => {
                self.analyze_call_expression(call_expr)
            }
            Expression::Assignment(assign_expr) => {
                self.analyze_assignment_expression(assign_expr)
            }
        }
    }

    fn analyze_binary_expression(&mut self, binary: &BinaryExpression) -> CompilerResult<Type> {
        let left_type = self.analyze_expression(&binary.left)?;
        let right_type = self.analyze_expression(&binary.right)?;

        match &binary.operator {
            BinaryOperator::Add | BinaryOperator::Subtract | BinaryOperator::Multiply | BinaryOperator::Divide => {
                if left_type == Type::Int && right_type == Type::Int {
                    Ok(Type::Int)
                } else if (left_type == Type::Int || left_type == Type::Float) && 
                          (right_type == Type::Int || right_type == Type::Float) {
                    Ok(Type::Float)
                } else {
                    Err(CompilerError::type_error_with_location(
                        format!(
                            "Operação {} não suportada entre {} e {}",
                            binary.operator, left_type, right_type
                        ),
                        binary.location.line,
                        binary.location.column,
                    ))
                }
            }
            BinaryOperator::Equal | BinaryOperator::NotEqual => {
                if self.types_compatible(&left_type, &right_type) {
                    Ok(Type::Bool)
                } else {
                    Err(CompilerError::type_error_with_location(
                        format!(
                            "Comparação {} não suportada entre {} e {}",
                            binary.operator, left_type, right_type
                        ),
                        binary.location.line,
                        binary.location.column,
                    ))
                }
            }
            BinaryOperator::LessThan | BinaryOperator::LessThanEqual | 
            BinaryOperator::GreaterThan | BinaryOperator::GreaterThanEqual => {
                if (left_type == Type::Int || left_type == Type::Float) && 
                   (right_type == Type::Int || right_type == Type::Float) {
                    Ok(Type::Bool)
                } else {
                    Err(CompilerError::type_error_with_location(
                        format!(
                            "Comparação {} não suportada entre {} e {}",
                            binary.operator, left_type, right_type
                        ),
                        binary.location.line,
                        binary.location.column,
                    ))
                }
            }
            BinaryOperator::And | BinaryOperator::Or => {
                if left_type == Type::Bool && right_type == Type::Bool {
                    Ok(Type::Bool)
                } else {
                    Err(CompilerError::type_error_with_location(
                        format!(
                            "Operação lógica {} não suportada entre {} e {}",
                            binary.operator, left_type, right_type
                        ),
                        binary.location.line,
                        binary.location.column,
                    ))
                }
            }
            BinaryOperator::Modulo => {
                if left_type == Type::Int && right_type == Type::Int {
                    Ok(Type::Int)
                } else {
                    Err(CompilerError::type_error_with_location(
                        format!(
                            "Operação módulo não suportada entre {} e {}",
                            left_type, right_type
                        ),
                        binary.location.line,
                        binary.location.column,
                    ))
                }
            }
        }
    }

    fn analyze_unary_expression(&mut self, unary: &UnaryExpression) -> CompilerResult<Type> {
        let operand_type = self.analyze_expression(&unary.operand)?;

        match &unary.operator {
            UnaryOperator::Minus => {
                if operand_type == Type::Int || operand_type == Type::Float {
                    Ok(operand_type)
                } else {
                    Err(CompilerError::type_error_with_location(
                        format!("Operador - não suportado para tipo {}", operand_type),
                        unary.location.line,
                        unary.location.column,
                    ))
                }
            }
            UnaryOperator::Not => {
                if operand_type == Type::Bool {
                    Ok(Type::Bool)
                } else {
                    Err(CompilerError::type_error_with_location(
                        format!("Operador ! não suportado para tipo {}", operand_type),
                        unary.location.line,
                        unary.location.column,
                    ))
                }
            }
            UnaryOperator::Negate => {
                if operand_type == Type::Int {
                    Ok(Type::Int)
                } else {
                    Err(CompilerError::type_error_with_location(
                        format!("Operador ~ não suportado para tipo {}", operand_type),
                        unary.location.line,
                        unary.location.column,
                    ))
                }
            }
        }
    }

    fn analyze_call_expression(&mut self, call: &CallExpression) -> CompilerResult<Type> {
        let symbol_info = {
            let symbol = self.current_scope.resolve(&call.function).ok_or_else(|| {
                CompilerError::semantic_with_location(
                    format!("Função '{}' não foi declarada", call.function),
                    call.location.line,
                    call.location.column,
                )
            })?;

            (symbol.is_function, symbol.parameters.clone(), symbol.return_type.clone())
        };

        if !symbol_info.0 {
            return Err(CompilerError::semantic_with_location(
                format!("'{}' não é uma função", call.function),
                call.location.line,
                call.location.column,
            ));
        }

        // Verificar número de argumentos
        if call.arguments.len() != symbol_info.1.len() {
            return Err(CompilerError::semantic_with_location(
                format!(
                    "Função '{}' espera {} argumentos, mas {} foram fornecidos",
                    call.function,
                    symbol_info.1.len(),
                    call.arguments.len()
                ),
                call.location.line,
                call.location.column,
            ));
        }

        // Verificar tipos dos argumentos
        for (i, (arg, expected_type)) in call.arguments.iter().zip(symbol_info.1.iter()).enumerate() {
            let arg_type = self.analyze_expression(arg)?;
            if !self.types_compatible(expected_type, &arg_type) {
                return Err(CompilerError::type_error_with_location(
                    format!(
                        "Argumento {} da função '{}': esperado {}, encontrado {}",
                        i + 1,
                        call.function,
                        expected_type,
                        arg_type
                    ),
                    call.location.line,
                    call.location.column,
                ));
            }
        }

        Ok(symbol_info.2.unwrap_or(Type::Void))
    }

    fn analyze_assignment_expression(&mut self, assign: &AssignmentExpression) -> CompilerResult<Type> {
        let symbol_type = {
            let symbol = self.current_scope.resolve(&assign.target).ok_or_else(|| {
                CompilerError::semantic_with_location(
                    format!("Variável '{}' não foi declarada", assign.target),
                    assign.location.line,
                    assign.location.column,
                )
            })?;
            symbol.symbol_type.clone()
        };

        let value_type = self.analyze_expression(&assign.value)?;

        if !self.types_compatible(&symbol_type, &value_type) {
            return Err(CompilerError::type_error_with_location(
                format!(
                    "Tipo incompatível na atribuição: esperado {}, encontrado {}",
                    symbol_type, value_type
                ),
                assign.location.line,
                assign.location.column,
            ));
        }

        Ok(symbol_type)
    }

    fn literal_type(&self, literal: &Literal) -> Type {
        match literal {
            Literal::Integer(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::Boolean(_) => Type::Bool,
            Literal::String(_) => Type::String,
        }
    }

    fn types_compatible(&self, expected: &Type, actual: &Type) -> bool {
        match (expected, actual) {
            (Type::Int, Type::Int) => true,
            (Type::Float, Type::Float) => true,
            (Type::Float, Type::Int) => true, // Int pode ser convertido para Float
            (Type::Bool, Type::Bool) => true,
            (Type::String, Type::String) => true,
            (Type::Void, Type::Void) => true,
            (Type::Function { parameters: p1, return_type: r1 }, 
             Type::Function { parameters: p2, return_type: r2 }) => {
                if p1.len() != p2.len() {
                    return false;
                }
                for (t1, t2) in p1.iter().zip(p2.iter()) {
                    if !self.types_compatible(t1, t2) {
                        return false;
                    }
                }
                self.types_compatible(r1, r2)
            }
            _ => false,
        }
    }
} 