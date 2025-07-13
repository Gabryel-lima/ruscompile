# RusCompile - Compilador em Rust

Um compilador simples e educacional escrito em Rust que demonstra os conceitos fundamentais de compilação, desde análise léxica até geração de código assembly.

## 🚀 Características

- **Análise Léxica**: Tokenização usando a biblioteca `logos`
- **Análise Sintática**: Parser recursivo descendente
- **Análise Semântica**: Verificação de tipos e escopo
- **Geração de Código**: Assembly x86-64
- **Tratamento de Erros**: Sistema robusto de mensagens de erro
- **Otimizações**: Múltiplos níveis de otimização
- **CLI**: Interface de linha de comando com `clap`

## 📋 Pré-requisitos

- Rust 1.70+ (edição 2021)
- NASM (Netwide Assembler) para compilar o assembly gerado
- Linux x86-64 (testado)

## 🛠️ Instalação

### Instalação Automática (Recomendada)

1. Clone o repositório:
```bash
git clone https://github.com/seu-usuario/ruscompile.git
cd ruscompile
```

2. Execute o script de instalação:
```bash
./install.sh
```

O script irá:
- Verificar e instalar dependências (Rust, NASM, binutils)
- Compilar o projeto
- Executar testes
- Instalar o compilador no sistema

### Instalação Manual

1. Clone o repositório:
```bash
git clone https://github.com/seu-usuario/ruscompile.git
cd ruscompile
```

2. Compile o projeto:
```bash
cargo build --release
```

3. Execute os testes:
```bash
cargo test
```

4. Instale manualmente:
```bash
sudo cp target/release/ruscompile /usr/local/bin/
```

## 📖 Uso

### Compilação Básica

```bash
# Compilar um arquivo fonte
./target/release/ruscompile examples/hello.rs

# Especificar arquivo de saída
./target/release/ruscompile examples/hello.rs -o hello.s

# Mostrar tokens léxicos
./target/release/ruscompile examples/hello.rs --tokens

# Mostrar AST
./target/release/ruscompile examples/hello.rs --ast

# Mostrar assembly gerado
./target/release/ruscompile examples/hello.rs --assembly
```

### Opções Disponíveis

- `-o, --output <FILE>`: Especificar arquivo de saída
- `-t, --tokens`: Mostrar tokens léxicos
- `-a, --ast`: Mostrar árvore sintática abstrata
- `-s, --assembly`: Mostrar código assembly gerado
- `-O, --optimization <LEVEL>`: Nível de otimização (0-3)

## 🗣️ Linguagem

### Sintaxe Básica

```rust
// Declaração de variáveis
var x: int = 10;
var y: float = 3.14;
var flag: bool = true;
var message: string = "Hello, World!";

// Declaração de funções
func add(a: int, b: int) -> int {
    return a + b;
}

// Estruturas de controle
if (x > 5) {
    println("x é maior que 5");
} else {
    println("x é menor ou igual a 5");
}

while (x > 0) {
    println(x);
    x = x - 1;
}
```

### Tipos Suportados

- `int`: Números inteiros (64 bits)
- `float`: Números de ponto flutuante (64 bits)
- `bool`: Valores booleanos (true/false)
- `string`: Cadeias de caracteres
- `void`: Tipo vazio (para funções sem retorno)

### Operadores

**Aritméticos:**
- `+`, `-`, `*`, `/`, `%`

**Comparação:**
- `==`, `!=`, `<`, `<=`, `>`, `>=`

**Lógicos:**
- `&&`, `||`, `!`

**Atribuição:**
- `=`

## 🏗️ Arquitetura

### Estrutura do Projeto

```
src/
├── main.rs          # Ponto de entrada e CLI
├── lexer.rs         # Analisador léxico
├── parser.rs        # Analisador sintático
├── ast.rs           # Árvore sintática abstrata
├── semantic.rs      # Analisador semântico
├── codegen.rs       # Gerador de código
├── error.rs         # Tratamento de erros
└── utils.rs         # Utilitários
```

### Fluxo de Compilação

1. **Análise Léxica**: Converte código fonte em tokens
2. **Análise Sintática**: Constrói a AST a partir dos tokens
3. **Análise Semântica**: Verifica tipos e escopo
4. **Geração de Código**: Produz assembly x86-64

## 🧪 Testes

Execute os testes unitários:

```bash
cargo test
```

Execute os benchmarks:

```bash
cargo bench
```

## 🗑️ Desinstalação

Para desinstalar o RusCompile completamente:

```bash
./uninstall.sh
```

O script irá:
- Remover o executável do sistema
- Limpar arquivos de cache e temporários
- Remover entradas do PATH (se existirem)
- Oferecer opção de remover dependências (NASM, binutils)

## 📊 Exemplos

### Exemplo 1: Hello World

```rust
func main() -> int {
    println("Hello, World!");
    return 0;
}
```

### Exemplo 2: Fatorial

```rust
func factorial(n: int) -> int {
    if (n <= 1) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

func main() -> int {
    var result: int = factorial(5);
    println("5! = ");
    println(result);
    return 0;
}
```

### Exemplo 3: Fibonacci

```rust
func fibonacci(n: int) -> int {
    if (n <= 1) {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

func main() -> int {
    var i: int = 0;
    while (i < 10) {
        println("fib(");
        println(i);
        println(") = ");
        println(fibonacci(i));
        i = i + 1;
    }
    return 0;
}
```

## 🔧 Desenvolvimento

### Adicionando Novos Recursos

1. **Novos Tokens**: Adicione ao enum `Token` em `lexer.rs`
2. **Novas Expressões**: Estenda o enum `Expression` em `ast.rs`
3. **Novos Operadores**: Implemente em `semantic.rs` e `codegen.rs`
4. **Otimizações**: Adicione ao módulo `utils.rs`

### Debugging

Para debuggar o compilador:

```bash
# Compilar em modo debug
cargo build

# Executar com logs detalhados
RUST_LOG=debug cargo run examples/hello.rs --tokens --ast
```

### Comandos Úteis do Makefile

```bash
make help          # Mostrar todos os comandos disponíveis
make setup         # Configurar ambiente completo
make remove        # Remover completamente
make deps          # Verificar dependências
make fmt           # Formatar código
make audit         # Análise de segurança
make stats         # Estatísticas do projeto
```

## 📈 Performance

O compilador foi otimizado para:

- **Velocidade**: Compilação rápida mesmo para arquivos grandes
- **Memória**: Uso eficiente de memória
- **Precisão**: Análise semântica rigorosa
- **Extensibilidade**: Fácil adição de novos recursos

## 🤝 Contribuindo

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## 📝 Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

## 🙏 Agradecimentos

- [Logos](https://github.com/maciejhirsz/logos) - Biblioteca de análise léxica
- [Clap](https://github.com/clap-rs/clap) - Framework para CLI
- [ThisError](https://github.com/dtolnay/thiserror) - Derivação de erros
- [Serde](https://github.com/serde-rs/serde) - Serialização

## 📚 Referências

- [Dragon Book](https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools)
- [LLVM Tutorial](https://llvm.org/docs/tutorial/)
- [NASM Documentation](https://www.nasm.us/doc/)

## 🐛 Problemas Conhecidos

- Suporte limitado a arrays e estruturas
- Não há garbage collection
- Otimizações básicas implementadas
- Suporte apenas para x86-64 Linux

## 🔮 Roadmap

- [ ] Suporte a arrays
- [ ] Estruturas de dados
- [ ] Ponteiros
- [ ] Módulos e namespaces
- [ ] Otimizações avançadas
- [ ] Suporte a outras arquiteturas
- [ ] Debugger integrado
- [ ] IDE support (LSP) 