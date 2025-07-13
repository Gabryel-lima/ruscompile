use std::collections::HashMap;
use crate::ast::*;
use crate::error::{CompilerError, CompilerResult};

#[derive(Debug)]
pub struct CodeGenerator {
    _optimization_level: u8,
    label_counter: usize,
    string_literals: HashMap<String, String>,
    current_function: Option<String>,
    local_variables: HashMap<String, i32>,
    stack_offset: i32,
}

impl CodeGenerator {
    pub fn new(optimization_level: u8) -> Self {
        Self {
            _optimization_level: optimization_level,
            label_counter: 0,
            string_literals: HashMap::new(),
            current_function: None,
            local_variables: HashMap::new(),
            stack_offset: 0,
        }
    }

    pub fn generate(&mut self, program: &Program) -> CompilerResult<String> {
        let mut assembly = String::new();

        // Seção de dados
        assembly.push_str("section .data\n");
        for (string, label) in &self.string_literals {
            assembly.push_str(&format!("{}: db \"{}\", 0\n", label, string));
        }

        // Seção de texto
        assembly.push_str("\nsection .text\n");
        assembly.push_str("global _start\n\n");

        // Gerar código para cada declaração
        for statement in &program.statements {
            assembly.push_str(&self.generate_statement(statement)?);
        }

        // Adicionar função main se não existir
        if !self.current_function.is_some() {
            assembly.push_str("\n_start:\n");
            assembly.push_str("    call main\n");
            assembly.push_str("    mov rax, 60\n");
            assembly.push_str("    xor rdi, rdi\n");
            assembly.push_str("    syscall\n");
        }

        Ok(assembly)
    }

    fn generate_statement(&mut self, statement: &Statement) -> CompilerResult<String> {
        match statement {
            Statement::Expression(expr_stmt) => {
                self.generate_expression(&expr_stmt.expression)?;
                Ok("    pop rax\n".to_string())
            }
            Statement::Declaration(decl_stmt) => {
                self.generate_declaration(decl_stmt)
            }
            Statement::Assignment(assign_stmt) => {
                self.generate_assignment(assign_stmt)
            }
            Statement::If(if_stmt) => {
                self.generate_if_statement(if_stmt)
            }
            Statement::While(while_stmt) => {
                self.generate_while_statement(while_stmt)
            }
            Statement::Function(func_stmt) => {
                self.generate_function(func_stmt)
            }
            Statement::Return(return_stmt) => {
                self.generate_return_statement(return_stmt)
            }
            Statement::Block(block_stmt) => {
                self.generate_block_statement(block_stmt)
            }
        }
    }

    fn generate_declaration(&mut self, decl: &DeclarationStatement) -> CompilerResult<String> {
        let mut assembly = String::new();

        // Alocar espaço na pilha para a variável
        self.stack_offset -= 8;
        let offset = self.stack_offset;
        self.local_variables.insert(decl.name.clone(), offset);

        // Se há inicializador, gerar código para ele
        if let Some(initializer) = &decl.initializer {
            assembly.push_str(&self.generate_expression(initializer)?);
            assembly.push_str("    pop rax\n");
            assembly.push_str(&format!("    mov [rbp{}], rax\n", offset));
        }

        Ok(assembly)
    }

    fn generate_assignment(&mut self, assign: &AssignmentStatement) -> CompilerResult<String> {
        let mut assembly = String::new();

        // Gerar código para o valor
        assembly.push_str(&self.generate_expression(&assign.value)?);
        assembly.push_str("    pop rax\n");

        // Encontrar offset da variável
        let offset = self.local_variables.get(&assign.target).ok_or_else(|| {
            CompilerError::codegen(format!("Variável '{}' não encontrada", assign.target))
        })?;

        assembly.push_str(&format!("    mov [rbp{}], rax\n", offset));

        Ok(assembly)
    }

    fn generate_if_statement(&mut self, if_stmt: &IfStatement) -> CompilerResult<String> {
        let mut assembly = String::new();
        let else_label = self.generate_label("else");
        let end_label = self.generate_label("endif");

        // Gerar código para a condição
        assembly.push_str(&self.generate_expression(&if_stmt.condition)?);
        assembly.push_str("    pop rax\n");
        assembly.push_str("    cmp rax, 0\n");
        assembly.push_str(&format!("    je {}\n", else_label));

        // Gerar código para o ramo then
        assembly.push_str(&self.generate_statement(&if_stmt.then_branch)?);
        assembly.push_str(&format!("    jmp {}\n", end_label));

        // Gerar código para o ramo else se presente
        assembly.push_str(&format!("{}:\n", else_label));
        if let Some(else_branch) = &if_stmt.else_branch {
            assembly.push_str(&self.generate_statement(else_branch)?);
        }

        assembly.push_str(&format!("{}:\n", end_label));

        Ok(assembly)
    }

    fn generate_while_statement(&mut self, while_stmt: &WhileStatement) -> CompilerResult<String> {
        let mut assembly = String::new();
        let loop_label = self.generate_label("while");
        let end_label = self.generate_label("endwhile");

        assembly.push_str(&format!("{}:\n", loop_label));

        // Gerar código para a condição
        assembly.push_str(&self.generate_expression(&while_stmt.condition)?);
        assembly.push_str("    pop rax\n");
        assembly.push_str("    cmp rax, 0\n");
        assembly.push_str(&format!("    je {}\n", end_label));

        // Gerar código para o corpo do loop
        assembly.push_str(&self.generate_statement(&while_stmt.body)?);
        assembly.push_str(&format!("    jmp {}\n", loop_label));

        assembly.push_str(&format!("{}:\n", end_label));

        Ok(assembly)
    }

    fn generate_function(&mut self, func: &FunctionStatement) -> CompilerResult<String> {
        let mut assembly = String::new();

        // Salvar estado anterior
        let old_function = self.current_function.take();
        let old_variables = std::mem::take(&mut self.local_variables);
        let old_stack_offset = self.stack_offset;

        self.current_function = Some(func.name.clone());
        self.stack_offset = 0;

        // Prologue da função
        assembly.push_str(&format!("{}:\n", func.name));
        assembly.push_str("    push rbp\n");
        assembly.push_str("    mov rbp, rsp\n");

        // Alocar espaço para variáveis locais
        let local_size = 8 * 10; // Espaço para 10 variáveis locais
        assembly.push_str(&format!("    sub rsp, {}\n", local_size));

        // Salvar parâmetros em variáveis locais
        for (i, param) in func.parameters.iter().enumerate() {
            let offset = -(i as i32 + 1) * 8;
            self.local_variables.insert(param.name.clone(), offset);
        }

        // Gerar código para o corpo da função
        assembly.push_str(&self.generate_block_statement(&func.body)?);

        // Epilogue da função
        assembly.push_str("    mov rsp, rbp\n");
        assembly.push_str("    pop rbp\n");
        assembly.push_str("    ret\n\n");

        // Restaurar estado anterior
        self.current_function = old_function;
        self.local_variables = old_variables;
        self.stack_offset = old_stack_offset;

        Ok(assembly)
    }

    fn generate_return_statement(&mut self, return_stmt: &ReturnStatement) -> CompilerResult<String> {
        let mut assembly = String::new();

        if let Some(value) = &return_stmt.value {
            assembly.push_str(&self.generate_expression(value)?);
            assembly.push_str("    pop rax\n");
        }

        assembly.push_str("    mov rsp, rbp\n");
        assembly.push_str("    pop rbp\n");
        assembly.push_str("    ret\n");

        Ok(assembly)
    }

    fn generate_block_statement(&mut self, block: &BlockStatement) -> CompilerResult<String> {
        let mut assembly = String::new();

        for statement in &block.statements {
            assembly.push_str(&self.generate_statement(statement)?);
        }

        Ok(assembly)
    }

    fn generate_expression(&mut self, expression: &Expression) -> CompilerResult<String> {
        match expression {
            Expression::Literal(literal_expr) => {
                self.generate_literal(&literal_expr.value)
            }
            Expression::Identifier(identifier_expr) => {
                self.generate_identifier(&identifier_expr.name)
            }
            Expression::Binary(binary_expr) => {
                self.generate_binary_expression(binary_expr)
            }
            Expression::Unary(unary_expr) => {
                self.generate_unary_expression(unary_expr)
            }
            Expression::Call(call_expr) => {
                self.generate_call_expression(call_expr)
            }
            Expression::Assignment(assign_expr) => {
                self.generate_assignment_expression(assign_expr)
            }
        }
    }

    fn generate_literal(&mut self, literal: &Literal) -> CompilerResult<String> {
        match literal {
            Literal::Integer(n) => {
                Ok(format!("    push {}\n", n))
            }
            Literal::Float(x) => {
                // Para simplificar, tratamos float como int
                Ok(format!("    push {}\n", *x as i64))
            }
            Literal::Boolean(b) => {
                let value = if *b { 1 } else { 0 };
                Ok(format!("    push {}\n", value))
            }
            Literal::String(s) => {
                let label = self.add_string_literal(s);
                Ok(format!("    push {}\n", label))
            }
        }
    }

    fn generate_identifier(&mut self, name: &str) -> CompilerResult<String> {
        let offset = self.local_variables.get(name).ok_or_else(|| {
            CompilerError::codegen(format!("Variável '{}' não encontrada", name))
        })?;

        Ok(format!("    mov rax, [rbp{}]\n    push rax\n", offset))
    }

    fn generate_binary_expression(&mut self, binary: &BinaryExpression) -> CompilerResult<String> {
        let mut assembly = String::new();

        // Gerar código para o operando direito
        assembly.push_str(&self.generate_expression(&binary.right)?);
        // Gerar código para o operando esquerdo
        assembly.push_str(&self.generate_expression(&binary.left)?);

        // Carregar operandos
        assembly.push_str("    pop rbx\n"); // Operando esquerdo
        assembly.push_str("    pop rax\n"); // Operando direito

        // Aplicar operação
        match &binary.operator {
            BinaryOperator::Add => {
                assembly.push_str("    add rax, rbx\n");
            }
            BinaryOperator::Subtract => {
                assembly.push_str("    sub rax, rbx\n");
            }
            BinaryOperator::Multiply => {
                assembly.push_str("    imul rax, rbx\n");
            }
            BinaryOperator::Divide => {
                assembly.push_str("    cqo\n");
                assembly.push_str("    idiv rbx\n");
            }
            BinaryOperator::Modulo => {
                assembly.push_str("    cqo\n");
                assembly.push_str("    idiv rbx\n");
                assembly.push_str("    mov rax, rdx\n");
            }
            BinaryOperator::Equal => {
                assembly.push_str("    cmp rax, rbx\n");
                assembly.push_str("    sete al\n");
                assembly.push_str("    movzx rax, al\n");
            }
            BinaryOperator::NotEqual => {
                assembly.push_str("    cmp rax, rbx\n");
                assembly.push_str("    setne al\n");
                assembly.push_str("    movzx rax, al\n");
            }
            BinaryOperator::LessThan => {
                assembly.push_str("    cmp rax, rbx\n");
                assembly.push_str("    setl al\n");
                assembly.push_str("    movzx rax, al\n");
            }
            BinaryOperator::LessThanEqual => {
                assembly.push_str("    cmp rax, rbx\n");
                assembly.push_str("    setle al\n");
                assembly.push_str("    movzx rax, al\n");
            }
            BinaryOperator::GreaterThan => {
                assembly.push_str("    cmp rax, rbx\n");
                assembly.push_str("    setg al\n");
                assembly.push_str("    movzx rax, al\n");
            }
            BinaryOperator::GreaterThanEqual => {
                assembly.push_str("    cmp rax, rbx\n");
                assembly.push_str("    setge al\n");
                assembly.push_str("    movzx rax, al\n");
            }
            BinaryOperator::And => {
                assembly.push_str("    and rax, rbx\n");
            }
            BinaryOperator::Or => {
                assembly.push_str("    or rax, rbx\n");
            }
        }

        assembly.push_str("    push rax\n");

        Ok(assembly)
    }

    fn generate_unary_expression(&mut self, unary: &UnaryExpression) -> CompilerResult<String> {
        let mut assembly = String::new();

        // Gerar código para o operando
        assembly.push_str(&self.generate_expression(&unary.operand)?);
        assembly.push_str("    pop rax\n");

        // Aplicar operação
        match &unary.operator {
            UnaryOperator::Minus => {
                assembly.push_str("    neg rax\n");
            }
            UnaryOperator::Not => {
                assembly.push_str("    cmp rax, 0\n");
                assembly.push_str("    sete al\n");
                assembly.push_str("    movzx rax, al\n");
            }
            UnaryOperator::Negate => {
                assembly.push_str("    not rax\n");
            }
        }

        assembly.push_str("    push rax\n");

        Ok(assembly)
    }

    fn generate_call_expression(&mut self, call: &CallExpression) -> CompilerResult<String> {
        let mut assembly = String::new();

        // Gerar código para os argumentos (em ordem reversa)
        for arg in call.arguments.iter().rev() {
            assembly.push_str(&self.generate_expression(arg)?);
        }

        // Chamar a função
        assembly.push_str(&format!("    call {}\n", call.function));

        // Limpar argumentos da pilha
        let arg_count = call.arguments.len();
        if arg_count > 0 {
            assembly.push_str(&format!("    add rsp, {}\n", arg_count * 8));
        }

        // O resultado está em rax, empurrar para a pilha
        assembly.push_str("    push rax\n");

        Ok(assembly)
    }

    fn generate_assignment_expression(&mut self, assign: &AssignmentExpression) -> CompilerResult<String> {
        let mut assembly = String::new();

        // Gerar código para o valor
        assembly.push_str(&self.generate_expression(&assign.value)?);
        assembly.push_str("    pop rax\n");

        // Encontrar offset da variável
        let offset = self.local_variables.get(&assign.target).ok_or_else(|| {
            CompilerError::codegen(format!("Variável '{}' não encontrada", assign.target))
        })?;

        assembly.push_str(&format!("    mov [rbp{}], rax\n", offset));
        assembly.push_str("    push rax\n");

        Ok(assembly)
    }

    fn generate_label(&mut self, prefix: &str) -> String {
        self.label_counter += 1;
        format!("{}_{}", prefix, self.label_counter)
    }

    fn add_string_literal(&mut self, string: &str) -> String {
        let label = format!("str_{}", self.string_literals.len());
        self.string_literals.insert(string.to_string(), label.clone());
        label
    }
} 