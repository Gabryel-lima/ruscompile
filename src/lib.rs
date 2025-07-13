//! RusCompile - Um compilador educacional em Rust
//! 
//! Este crate fornece uma implementação completa de um compilador,
//! desde análise léxica até geração de código assembly.

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod codegen;
pub mod error;
pub mod utils;

// Re-export principais tipos para facilitar o uso
pub use ast::*;
pub use lexer::{Lexer, Token, TokenInfo};
pub use parser::Parser;
pub use semantic::SemanticAnalyzer;
pub use codegen::CodeGenerator;
pub use error::{CompilerError, CompilerResult};
pub use utils::*;

/// Estrutura principal do compilador
pub struct Compiler {
    config: CompilerConfig,
    stats: CompilerStats,
}

impl Compiler {
    /// Cria uma nova instância do compilador com configurações padrão
    pub fn new() -> Self {
        Self {
            config: CompilerConfig::default(),
            stats: CompilerStats::new(),
        }
    }

    /// Cria uma nova instância do compilador com configurações personalizadas
    pub fn with_config(config: CompilerConfig) -> Self {
        Self {
            config,
            stats: CompilerStats::new(),
        }
    }

    /// Compila código fonte em assembly
    pub fn compile(&mut self, source: &str) -> CompilerResult<String> {
        let start_time = std::time::Instant::now();

        // Análise léxica
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        self.stats.tokens_generated = tokens.len();

        // Análise sintática
        let mut parser = Parser::new(tokens);
        let mut ast = parser.parse()?;
        self.stats.ast_nodes = self.count_ast_nodes(&ast);

        // Análise semântica
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.analyze(&ast)?;

        // Otimização (se habilitada)
        if self.config._optimization_level > 0 {
            let optimizer = Optimizer::new(self.config.clone());
            optimizer.optimize_ast(&mut ast)?;
        }

        // Geração de código
        let mut codegen = CodeGenerator::new(self.config._optimization_level);
        let assembly = codegen.generate(&ast)?;

        // Atualizar estatísticas
        self.stats.compilation_time_ms = start_time.elapsed().as_millis() as u64;
        self.stats.lines_processed = source.lines().count();

        Ok(assembly)
    }

    /// Compila um arquivo fonte
    pub fn compile_file(&mut self, file_path: &str) -> CompilerResult<String> {
        let source = std::fs::read_to_string(file_path)
            .map_err(|e| CompilerError::FileReadError(file_path.into(), e))?;
        
        self.compile(&source)
    }

    /// Retorna as estatísticas da última compilação
    pub fn get_stats(&self) -> &CompilerStats {
        &self.stats
    }

    /// Retorna as configurações do compilador
    pub fn get_config(&self) -> &CompilerConfig {
        &self.config
    }

    /// Atualiza as configurações do compilador
    pub fn set_config(&mut self, config: CompilerConfig) {
        self.config = config;
    }

    /// Valida código fonte sem gerar assembly
    pub fn validate(&self, source: &str) -> CompilerResult<()> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        let mut analyzer = SemanticAnalyzer::new();
        analyzer.analyze(&ast)?;

        Ok(())
    }

    /// Analisa a complexidade ciclomática do código
    pub fn analyze_complexity(&self, source: &str) -> CompilerResult<String> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        let mut analyzer = ComplexityAnalyzer::new();
        
        for statement in &ast.statements {
            if let Statement::Function(func) = statement {
                analyzer.analyze_function(&func.name, statement);
            }
        }

        Ok(analyzer.get_complexity_report())
    }

    /// Gera documentação do código
    pub fn generate_docs(&self, source: &str, format: DocumentationFormat) -> CompilerResult<String> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        let doc_gen = DocumentationGenerator::new(format);
        Ok(doc_gen.generate_docs(&ast))
    }

    /// Formata código fonte
    pub fn format_source(&self, source: &str) -> String {
        let formatter = SourceFormatter::new();
        formatter.format_source(source)
    }

    /// Conta nós na AST recursivamente
    fn count_ast_nodes(&self, program: &Program) -> usize {
        let mut count = 0;
        for statement in &program.statements {
            count += self.count_statement_nodes(statement);
        }
        count
    }

    fn count_statement_nodes(&self, statement: &Statement) -> usize {
        let mut count = 1; // O próprio statement

        match statement {
            Statement::Expression(expr_stmt) => {
                count += self.count_expression_nodes(&expr_stmt.expression);
            }
            Statement::Declaration(decl_stmt) => {
                if let Some(init) = &decl_stmt.initializer {
                    count += self.count_expression_nodes(init);
                }
            }
            Statement::Assignment(assign_stmt) => {
                count += self.count_expression_nodes(&assign_stmt.value);
            }
            Statement::If(if_stmt) => {
                count += self.count_expression_nodes(&if_stmt.condition);
                count += self.count_statement_nodes(&if_stmt.then_branch);
                if let Some(else_branch) = &if_stmt.else_branch {
                    count += self.count_statement_nodes(else_branch);
                }
            }
            Statement::While(while_stmt) => {
                count += self.count_expression_nodes(&while_stmt.condition);
                count += self.count_statement_nodes(&while_stmt.body);
            }
            Statement::Function(func_stmt) => {
                count += self.count_block_nodes(&func_stmt.body);
            }
            Statement::Return(return_stmt) => {
                if let Some(value) = &return_stmt.value {
                    count += self.count_expression_nodes(value);
                }
            }
            Statement::Block(block_stmt) => {
                count += self.count_block_nodes(block_stmt);
            }
        }

        count
    }

    fn count_expression_nodes(&self, expression: &Expression) -> usize {
        let mut count = 1; // A própria expressão

        match expression {
            Expression::Binary(binary_expr) => {
                count += self.count_expression_nodes(&binary_expr.left);
                count += self.count_expression_nodes(&binary_expr.right);
            }
            Expression::Unary(unary_expr) => {
                count += self.count_expression_nodes(&unary_expr.operand);
            }
            Expression::Call(call_expr) => {
                for arg in &call_expr.arguments {
                    count += self.count_expression_nodes(arg);
                }
            }
            Expression::Assignment(assign_expr) => {
                count += self.count_expression_nodes(&assign_expr.value);
            }
            _ => {}
        }

        count
    }

    fn count_block_nodes(&self, block: &BlockStatement) -> usize {
        let mut count = 1; // O próprio bloco

        for statement in &block.statements {
            count += self.count_statement_nodes(statement);
        }

        count
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Função de conveniência para compilação rápida
pub fn compile(source: &str) -> CompilerResult<String> {
    let mut compiler = Compiler::new();
    compiler.compile(source)
}

/// Função de conveniência para validação rápida
pub fn validate(source: &str) -> CompilerResult<()> {
    let compiler = Compiler::new();
    compiler.validate(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_creation() {
        let compiler = Compiler::new();
        assert_eq!(compiler.get_config()._optimization_level, 0);
    }

    #[test]
    fn test_simple_compilation() {
        let source = r#"
            func main() -> int {
                return 42;
            }
        "#;

        let result = compile(source);
        assert!(result.is_ok());
        
        let assembly = result.unwrap();
        assert!(assembly.contains("main:"));
        assert!(assembly.contains("ret"));
    }

    #[test]
    fn test_validation() {
        let source = r#"
            func main() -> int {
                var x: int = 10;
                return x;
            }
        "#;

        let result = validate(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_code() {
        let source = "invalid code";
        let result = validate(source);
        assert!(result.is_err());
    }

    #[test]
    fn test_complexity_analysis() {
        let source = r#"
            func factorial(n: int) -> int {
                if (n <= 1) {
                    return 1;
                } else {
                    return n * factorial(n - 1);
                }
            }
        "#;

        let compiler = Compiler::new();
        let result = compiler.analyze_complexity(source);
        assert!(result.is_ok());
        
        let report = result.unwrap();
        assert!(report.contains("factorial"));
    }

    #[test]
    fn test_documentation_generation() {
        let source = r#"
            func add(a: int, b: int) -> int {
                return a + b;
            }
        "#;

        let compiler = Compiler::new();
        let result = compiler.generate_docs(source, DocumentationFormat::Markdown);
        assert!(result.is_ok());
        
        let docs = result.unwrap();
        assert!(docs.contains("add"));
        assert!(docs.contains("int"));
    }
} 