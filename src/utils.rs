use std::collections::HashMap;
use std::fmt;

/// Estrutura para armazenar estatísticas do compilador
#[derive(Debug, Default)]
pub struct CompilerStats {
    pub lines_processed: usize,
    pub tokens_generated: usize,
    pub ast_nodes: usize,
    pub functions_defined: usize,
    pub variables_declared: usize,
    pub errors_found: usize,
    pub warnings_found: usize,
    pub compilation_time_ms: u64,
}

impl CompilerStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn print_summary(&self) {
        println!("=== Estatísticas da Compilação ===");
        println!("Linhas processadas: {}", self.lines_processed);
        println!("Tokens gerados: {}", self.tokens_generated);
        println!("Nós da AST: {}", self.ast_nodes);
        println!("Funções definidas: {}", self.functions_defined);
        println!("Variáveis declaradas: {}", self.variables_declared);
        println!("Erros encontrados: {}", self.errors_found);
        println!("Avisos encontrados: {}", self.warnings_found);
        println!("Tempo de compilação: {}ms", self.compilation_time_ms);
    }
}

/// Estrutura para configurações do compilador
#[derive(Debug, Clone)]
pub struct CompilerConfig {
    pub _optimization_level: u8,
    pub _show_tokens: bool,
    pub _show_ast: bool,
    pub _show_assembly: bool,
    pub _warnings_as_errors: bool,
    pub _target_architecture: String,
    pub _output_format: OutputFormat,
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Assembly,
    #[allow(dead_code)]
    Object,
    #[allow(dead_code)]
    Executable,
}

impl Default for CompilerConfig {
    fn default() -> Self {
        Self {
            _optimization_level: 0,
            _show_tokens: false,
            _show_ast: false,
            _show_assembly: false,
            _warnings_as_errors: false,
            _target_architecture: "x86_64".to_string(),
            _output_format: OutputFormat::Assembly,
        }
    }
}

/// Utilitário para formatação de código fonte
#[allow(dead_code)]
pub struct SourceFormatter {
    indent_size: usize,
    _max_line_length: usize,
}

impl SourceFormatter {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            indent_size: 4,
            _max_line_length: 80,
        }
    }

    #[allow(dead_code)]
    pub fn format_source(&self, source: &str) -> String {
        let lines: Vec<&str> = source.lines().collect();
        let mut formatted = String::new();
        let mut indent_level: usize = 0;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                formatted.push('\n');
                continue;
            }

            // Ajustar indentação baseado em palavras-chave
            if trimmed.starts_with('}') {
                indent_level = indent_level.saturating_sub(1);
            }

            // Aplicar indentação
            let indent = " ".repeat(indent_level * self.indent_size);
            formatted.push_str(&format!("{}{}\n", indent, trimmed));

            // Aumentar indentação para blocos
            if trimmed.ends_with('{') {
                indent_level += 1;
            }
        }

        formatted
    }
}

/// Utilitário para análise de complexidade ciclomática
#[allow(dead_code)]
pub struct ComplexityAnalyzer {
    complexity_map: HashMap<String, usize>,
}

impl ComplexityAnalyzer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            complexity_map: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn analyze_function(&mut self, function_name: &str, ast: &crate::ast::Statement) -> usize {
        let complexity = self.calculate_complexity(ast);
        self.complexity_map.insert(function_name.to_string(), complexity);
        complexity
    }

    #[allow(dead_code)]
    fn calculate_complexity(&self, statement: &crate::ast::Statement) -> usize {
        match statement {
            crate::ast::Statement::If(_) => 1,
            crate::ast::Statement::While(_) => 1,
            crate::ast::Statement::Function(func) => {
                let mut complexity = 1; // Base complexity
                for stmt in &func.body.statements {
                    complexity += self.calculate_complexity(stmt);
                }
                complexity
            }
            crate::ast::Statement::Block(block) => {
                let mut complexity = 0;
                for stmt in &block.statements {
                    complexity += self.calculate_complexity(stmt);
                }
                complexity
            }
            _ => 0,
        }
    }

    #[allow(dead_code)]
    pub fn get_complexity_report(&self) -> String {
        let mut report = String::from("=== Relatório de Complexidade Ciclomática ===\n");
        
        for (function, complexity) in &self.complexity_map {
            let risk_level = match complexity {
                1..=10 => "Baixo",
                11..=20 => "Médio",
                21..=50 => "Alto",
                _ => "Muito Alto",
            };
            
            report.push_str(&format!(
                "{}: {} ({})\n",
                function, complexity, risk_level
            ));
        }
        
        report
    }
}

/// Utilitário para otimizações básicas
#[allow(dead_code)]
pub struct Optimizer {
    config: CompilerConfig,
}

impl Optimizer {
    #[allow(dead_code)]
    pub fn new(config: CompilerConfig) -> Self {
        Self { config }
    }

    #[allow(dead_code)]
    pub fn optimize_ast(&self, program: &mut crate::ast::Program) -> Result<(), String> {
        match self.config._optimization_level {
            0 => Ok(()), // Sem otimizações
            1 => self.constant_folding(program),
            2 => {
                self.constant_folding(program)?;
                self.dead_code_elimination(program)
            }
            3 => {
                self.constant_folding(program)?;
                self.dead_code_elimination(program)?;
                self.expression_simplification(program)
            }
            _ => Err("Nível de otimização inválido".to_string()),
        }
    }

    #[allow(dead_code)]
    fn constant_folding(&self, _program: &mut crate::ast::Program) -> Result<(), String> {
        // Implementar dobramento de constantes
        // Ex: 2 + 3 -> 5
        Ok(())
    }

    #[allow(dead_code)]
    fn dead_code_elimination(&self, _program: &mut crate::ast::Program) -> Result<(), String> {
        // Implementar eliminação de código morto
        // Ex: remover variáveis não utilizadas
        Ok(())
    }

    #[allow(dead_code)]
    fn expression_simplification(&self, _program: &mut crate::ast::Program) -> Result<(), String> {
        // Implementar simplificação de expressões
        // Ex: x + 0 -> x, x * 1 -> x
        Ok(())
    }
}

/// Utilitário para validação de código
#[allow(dead_code)]
pub struct CodeValidator {
    warnings: Vec<String>,
    errors: Vec<String>,
}

impl CodeValidator {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn validate(&mut self, program: &crate::ast::Program) -> bool {
        self.warnings.clear();
        self.errors.clear();

        // Verificar se há função main
        let has_main = program.statements.iter().any(|stmt| {
            if let crate::ast::Statement::Function(func) = stmt {
                func.name == "main"
            } else {
                false
            }
        });

        if !has_main {
            self.warnings.push("Função 'main' não encontrada".to_string());
        }

        // Verificar variáveis não utilizadas
        self.check_unused_variables(program);

        // Verificar funções não utilizadas
        self.check_unused_functions(program);

        self.errors.is_empty()
    }

    #[allow(dead_code)]
    fn check_unused_variables(&mut self, _program: &crate::ast::Program) {
        // Implementar verificação de variáveis não utilizadas
    }

    #[allow(dead_code)]
    fn check_unused_functions(&mut self, _program: &crate::ast::Program) {
        // Implementar verificação de funções não utilizadas
    }

    #[allow(dead_code)]
    pub fn get_warnings(&self) -> &[String] {
        &self.warnings
    }

    #[allow(dead_code)]
    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }

    #[allow(dead_code)]
    pub fn print_report(&self) {
        if !self.errors.is_empty() {
            println!("=== Erros ===");
            for error in &self.errors {
                println!("❌ {}", error);
            }
        }

        if !self.warnings.is_empty() {
            println!("=== Avisos ===");
            for warning in &self.warnings {
                println!("⚠️  {}", warning);
            }
        }
    }
}

/// Utilitário para geração de documentação
#[allow(dead_code)]
pub struct DocumentationGenerator {
    output_format: DocumentationFormat,
}

#[derive(Debug, Clone)]
pub enum DocumentationFormat {
    #[allow(dead_code)]
    Markdown,
    #[allow(dead_code)]
    HTML,
    #[allow(dead_code)]
    PlainText,
}

impl DocumentationGenerator {
    #[allow(dead_code)]
    pub fn new(output_format: DocumentationFormat) -> Self {
        Self { output_format }
    }

    #[allow(dead_code)]
    pub fn generate_docs(&self, program: &crate::ast::Program) -> String {
        match self.output_format {
            DocumentationFormat::Markdown => self.generate_markdown(program),
            DocumentationFormat::HTML => self.generate_html(program),
            DocumentationFormat::PlainText => self.generate_plain_text(program),
        }
    }

    #[allow(dead_code)]
    fn generate_markdown(&self, program: &crate::ast::Program) -> String {
        let mut docs = String::from("# Documentação do Código\n\n");

        // Documentar funções
        docs.push_str("## Funções\n\n");
        for statement in &program.statements {
            if let crate::ast::Statement::Function(func) = statement {
                docs.push_str(&format!("### {}\n\n", func.name));
                docs.push_str(&format!("**Tipo de retorno:** {}\n\n", func.return_type));
                
                if !func.parameters.is_empty() {
                    docs.push_str("**Parâmetros:**\n");
                    for param in &func.parameters {
                        docs.push_str(&format!("- `{}`: {}\n", param.name, param.param_type));
                    }
                    docs.push_str("\n");
                }
            }
        }

        docs
    }

    #[allow(dead_code)]
    fn generate_html(&self, _program: &crate::ast::Program) -> String {
        let mut docs = String::from("<html><head><title>Documentação</title></head><body>");
        docs.push_str("<h1>Documentação do Código</h1>");
        
        // Implementar geração HTML
        
        docs.push_str("</body></html>");
        docs
    }

    #[allow(dead_code)]
    fn generate_plain_text(&self, _program: &crate::ast::Program) -> String {
        let mut docs = String::from("DOCUMENTAÇÃO DO CÓDIGO\n");
        docs.push_str("=======================\n\n");
        
        // Implementar geração de texto simples
        
        docs
    }
}

impl fmt::Display for CompilerStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Estatísticas: {} linhas, {} tokens, {} nós AST, {} funções, {} variáveis, {} erros, {} avisos, {}ms",
            self.lines_processed,
            self.tokens_generated,
            self.ast_nodes,
            self.functions_defined,
            self.variables_declared,
            self.errors_found,
            self.warnings_found,
            self.compilation_time_ms
        )
    }
} 