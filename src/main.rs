use std::path::PathBuf;
use anyhow::Result;
use clap::Parser;

mod lexer;
mod parser;
mod ast;
mod semantic;
mod codegen;
mod error;
mod utils;

use error::CompilerError;
use lexer::Lexer;
use parser::Parser as AstParser;
use semantic::SemanticAnalyzer;
use codegen::CodeGenerator;

#[derive(Parser)]
#[command(name = "ruscompile")]
#[command(about = "Um compilador simples escrito em Rust")]
#[command(version)]
struct Cli {
    /// Arquivo fonte para compilar
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// Arquivo de saída (opcional)
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Mostrar tokens léxicos
    #[arg(short, long)]
    tokens: bool,

    /// Mostrar AST
    #[arg(short, long)]
    ast: bool,

    /// Mostrar código assembly gerado
    #[arg(short, long)]
    assembly: bool,

    /// Nível de otimização (0-3)
    #[arg(short, long, default_value = "0")]
    optimization: u8,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Ler arquivo fonte
    let source = std::fs::read_to_string(&cli.input)
        .map_err(|e| CompilerError::FileReadError(cli.input.clone(), e))?;

    println!("Compilando: {}", cli.input.display());

    // Análise léxica
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;

    if cli.tokens {
        println!("\n=== TOKENS ===");
        for token in &tokens {
            println!("{:?}", token);
        }
    }

    // Análise sintática
    let mut parser = AstParser::new(tokens);
    let ast = parser.parse()?;

    if cli.ast {
        println!("\n=== AST ===");
        println!("{:#?}", ast);
    }

    // Análise semântica
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&ast)?;

    // Geração de código
    let mut codegen = CodeGenerator::new(cli.optimization);
    let assembly = codegen.generate(&ast)?;

    if cli.assembly {
        println!("\n=== ASSEMBLY ===");
        println!("{}", assembly);
    }

    // Salvar arquivo de saída
    let output_path = cli.output.unwrap_or_else(|| {
        cli.input.with_extension("s")
    });

    std::fs::write(&output_path, assembly)
        .map_err(|e| CompilerError::FileWriteError(output_path.clone(), e))?;

    println!("Compilação concluída: {}", output_path.display());
    Ok(())
} 