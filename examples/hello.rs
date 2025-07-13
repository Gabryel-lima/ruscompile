use ruscompile::*;

fn main() {
    // Exemplo de uso do compilador ruscompile
    let source = r#"
        func main() -> int {
            var x: int = 10;
            var y: int = 20;
            var result: int = x + y;
            
            if (result > 25) {
                println("Resultado e maior que 25");
            } else {
                println("Resultado e menor ou igual a 25");
            }
            
            while (x > 0) {
                println("x = ");
                println_int(x);
                x = x - 1;
            }
            
            return result;
        }
    "#;

    println!("Compilando código fonte...");
    
    match compile(source) {
        Ok(assembly) => {
            println!("Compilação bem-sucedida!");
            println!("Assembly gerado:");
            println!("{}", assembly);
        }
        Err(e) => {
            eprintln!("Erro na compilação: {}", e);
        }
    }
}