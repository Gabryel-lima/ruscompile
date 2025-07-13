use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ruscompile::{Lexer, Parser};

fn parser_benchmark(c: &mut Criterion) {
    let source = r#"
        func factorial(n: int) -> int {
            if (n <= 1) {
                return 1;
            } else {
                return n * factorial(n - 1);
            }
        }
        
        func main() -> int {
            var result: int = factorial(10);
            println("Resultado: ");
            println(result);
            return result;
        }
    "#;

    c.bench_function("parser_parse", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(source));
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap();
        });
    });
}

fn parser_simple_benchmark(c: &mut Criterion) {
    let simple_source = r#"
        func add(a: int, b: int) -> int {
            return a + b;
        }
        
        func main() -> int {
            var x: int = 10;
            var y: int = 20;
            var result: int = add(x, y);
            return result;
        }
    "#;

    c.bench_function("parser_simple", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(simple_source));
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap();
        });
    });
}

fn parser_expression_benchmark(c: &mut Criterion) {
    let expression_source = r#"
        func main() -> int {
            var a: int = 1;
            var b: int = 2;
            var c: int = 3;
            var d: int = 4;
            var e: int = 5;
            
            var result: int = a + b * c - d / e;
            var complex: int = (a + b) * (c - d) + e;
            var nested: int = ((a + b) * c) - (d / e);
            
            return result + complex + nested;
        }
    "#;

    c.bench_function("parser_expressions", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(expression_source));
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap();
        });
    });
}

fn parser_large_benchmark(c: &mut Criterion) {
    // Criar um arquivo fonte grande para benchmark
    let mut large_source = String::new();
    large_source.push_str("func main() -> int {\n");
    for i in 0..100 {
        large_source.push_str(&format!("    var x{}: int = {};\n", i, i));
    }
    large_source.push_str("    return 0;\n}\n");

    c.bench_function("parser_large", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(&large_source));
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse().unwrap();
        });
    });
}

criterion_group!(
    benches,
    parser_benchmark,
    parser_simple_benchmark,
    parser_expression_benchmark,
    parser_large_benchmark
);
criterion_main!(benches); 