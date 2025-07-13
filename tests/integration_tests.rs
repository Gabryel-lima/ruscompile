use ruscompile::*;
use std::fs;
use std::path::Path;

#[test]
fn test_hello_world_compilation() {
    let source = r#"
        func main() -> int {
            println("Hello, World!");
            return 0;
        }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Falha na análise léxica");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Falha na análise sintática");
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&ast).expect("Falha na análise semântica");
    
    let mut codegen = CodeGenerator::new(0);
    let assembly = codegen.generate(&ast).expect("Falha na geração de código");
    
    // Verificar se o assembly contém elementos esperados
    assert!(assembly.contains("section .data"));
    assert!(assembly.contains("section .text"));
    assert!(assembly.contains("main:"));
    assert!(assembly.contains("ret"));
}

#[test]
fn test_factorial_function() {
    let source = r#"
        func factorial(n: int) -> int {
            if (n <= 1) {
                return 1;
            } else {
                return n * factorial(n - 1);
            }
        }
        
        func main() -> int {
            var result: int = factorial(5);
            return result;
        }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Falha na análise léxica");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Falha na análise sintática");
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&ast).expect("Falha na análise semântica");
    
    let mut codegen = CodeGenerator::new(0);
    let assembly = codegen.generate(&ast).expect("Falha na geração de código");
    
    assert!(assembly.contains("factorial:"));
    assert!(assembly.contains("main:"));
    assert!(assembly.contains("call factorial"));
}

#[test]
fn test_variable_declaration_and_assignment() {
    let source = r#"
        func main() -> int {
            var x: int = 10;
            var y: int = 20;
            x = x + y;
            return x;
        }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Falha na análise léxica");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Falha na análise sintática");
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&ast).expect("Falha na análise semântica");
    
    let mut codegen = CodeGenerator::new(0);
    let assembly = codegen.generate(&ast).expect("Falha na geração de código");
    
    assert!(assembly.contains("add"));
    assert!(assembly.contains("mov"));
}

#[test]
fn test_if_else_statement() {
    let source = r#"
        func main() -> int {
            var x: int = 15;
            if (x > 10) {
                return 1;
            } else {
                return 0;
            }
        }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Falha na análise léxica");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Falha na análise sintática");
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&ast).expect("Falha na análise semântica");
    
    let mut codegen = CodeGenerator::new(0);
    let assembly = codegen.generate(&ast).expect("Falha na geração de código");
    
    assert!(assembly.contains("cmp"));
    assert!(assembly.contains("je"));
    assert!(assembly.contains("jmp"));
}

#[test]
fn test_while_loop() {
    let source = r#"
        func main() -> int {
            var i: int = 0;
            while (i < 5) {
                i = i + 1;
            }
            return i;
        }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Falha na análise léxica");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Falha na análise sintática");
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&ast).expect("Falha na análise semântica");
    
    let mut codegen = CodeGenerator::new(0);
    let assembly = codegen.generate(&ast).expect("Falha na geração de código");
    
    assert!(assembly.contains("while"));
    assert!(assembly.contains("endwhile"));
}

#[test]
fn test_binary_operations() {
    let source = r#"
        func main() -> int {
            var a: int = 10;
            var b: int = 5;
            var result: int = a + b * 2 - 3;
            return result;
        }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Falha na análise léxica");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Falha na análise sintática");
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&ast).expect("Falha na análise semântica");
    
    let mut codegen = CodeGenerator::new(0);
    let assembly = codegen.generate(&ast).expect("Falha na geração de código");
    
    assert!(assembly.contains("add"));
    assert!(assembly.contains("imul"));
    assert!(assembly.contains("sub"));
}

#[test]
fn test_logical_operations() {
    let source = r#"
        func main() -> int {
            var a: bool = true;
            var b: bool = false;
            var result: bool = a && b || !a;
            return result;
        }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Falha na análise léxica");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Falha na análise sintática");
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&ast).expect("Falha na análise semântica");
    
    let mut codegen = CodeGenerator::new(0);
    let assembly = codegen.generate(&ast).expect("Falha na geração de código");
    
    assert!(assembly.contains("and"));
    assert!(assembly.contains("or"));
}

#[test]
fn test_function_parameters() {
    let source = r#"
        func add(a: int, b: int) -> int {
            return a + b;
        }
        
        func main() -> int {
            var result: int = add(10, 20);
            return result;
        }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Falha na análise léxica");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Falha na análise sintática");
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&ast).expect("Falha na análise semântica");
    
    let mut codegen = CodeGenerator::new(0);
    let assembly = codegen.generate(&ast).expect("Falha na geração de código");
    
    assert!(assembly.contains("add:"));
    assert!(assembly.contains("call add"));
}

#[test]
fn test_error_handling() {
    // Teste com código inválido
    let source = "var x: int = ;"; // Falta valor na atribuição
    
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize();
    
    // Deve falhar na análise sintática
    if let Ok(tokens) = result {
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse();
        assert!(parse_result.is_err());
    }
}

#[test]
fn test_type_checking() {
    let source = r#"
        func main() -> int {
            var x: int = 10;
            var y: string = "hello";
            x = y; // Erro de tipo
            return x;
        }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Falha na análise léxica");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Falha na análise sintática");
    
    let mut analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze(&ast);
    
    // Deve falhar na análise semântica devido ao erro de tipo
    assert!(result.is_err());
}

#[test]
fn test_optimization_levels() {
    let source = r#"
        func main() -> int {
            var x: int = 2 + 3 * 4;
            return x;
        }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Falha na análise léxica");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Falha na análise sintática");
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&ast).expect("Falha na análise semântica");
    
    // Testar diferentes níveis de otimização
    for opt_level in 0..=3 {
        let mut codegen = CodeGenerator::new(opt_level);
        let assembly = codegen.generate(&ast).expect("Falha na geração de código");
        
        // Verificar se o assembly foi gerado corretamente
        assert!(!assembly.is_empty());
        assert!(assembly.contains("main:"));
    }
}

#[test]
fn test_string_literals() {
    let source = r#"
        func main() -> int {
            println("Hello, World!");
            println("Teste de string");
            return 0;
        }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Falha na análise léxica");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Falha na análise sintática");
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&ast).expect("Falha na análise semântica");
    
    let mut codegen = CodeGenerator::new(0);
    let assembly = codegen.generate(&ast).expect("Falha na geração de código");
    
    // Verificar se as strings foram incluídas na seção de dados
    assert!(assembly.contains("section .data"));
    assert!(assembly.contains("Hello, World!"));
    assert!(assembly.contains("Teste de string"));
}

#[test]
fn test_complex_expression() {
    let source = r#"
        func main() -> int {
            var a: int = 10;
            var b: int = 5;
            var c: int = 3;
            var result: int = (a + b) * c - (a / b);
            return result;
        }
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Falha na análise léxica");
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Falha na análise sintática");
    
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&ast).expect("Falha na análise semântica");
    
    let mut codegen = CodeGenerator::new(0);
    let assembly = codegen.generate(&ast).expect("Falha na geração de código");
    
    // Verificar se todas as operações foram geradas
    assert!(assembly.contains("add"));
    assert!(assembly.contains("imul"));
    assert!(assembly.contains("sub"));
    assert!(assembly.contains("idiv"));
} 