use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ruscompile::Lexer;

fn lexer_benchmark(c: &mut Criterion) {
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

    c.bench_function("lexer_tokenize", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(source));
            lexer.tokenize().unwrap();
        });
    });
}

fn lexer_simple_benchmark(c: &mut Criterion) {
    let simple_source = "var x: int = 42; var y: int = x + 10;";

    c.bench_function("lexer_simple", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(simple_source));
            lexer.tokenize().unwrap();
        });
    });
}

fn lexer_large_benchmark(c: &mut Criterion) {
    // Criar um arquivo fonte grande para benchmark
    let mut large_source = String::new();
    for i in 0..1000 {
        large_source.push_str(&format!("var x{}: int = {};\n", i, i));
    }

    c.bench_function("lexer_large", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(&large_source));
            lexer.tokenize().unwrap();
        });
    });
}

criterion_group!(benches, lexer_benchmark, lexer_simple_benchmark, lexer_large_benchmark);
criterion_main!(benches); 