// Exemplo de programa simples em nossa linguagem
// Este arquivo demonstra a sintaxe basica do compilador

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

func factorial(n: int) -> int {
    if (n <= 1) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

func fibonacci(n: int) -> int {
    if (n <= 1) {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}