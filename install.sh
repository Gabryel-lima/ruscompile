#!/bin/bash

# Script de instalação para RusCompile
# Este script instala todas as dependências necessárias e compila o projeto

set -e

echo "🚀 Instalando RusCompile..."

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Função para imprimir mensagens coloridas
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Verificar se estamos no diretório correto
if [ ! -f "Cargo.toml" ]; then
    print_error "Este script deve ser executado no diretório raiz do projeto RusCompile"
    exit 1
fi

# Verificar se o Rust está instalado
print_status "Verificando se o Rust está instalado..."
if ! command -v rustc &> /dev/null; then
    print_error "Rust não está instalado. Instalando..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    print_success "Rust instalado com sucesso!"
else
    print_success "Rust já está instalado: $(rustc --version)"
fi

# Verificar se o NASM está instalado
print_status "Verificando se o NASM está instalado..."
if ! command -v nasm &> /dev/null; then
    print_warning "NASM não está instalado. Tentando instalar..."
    
    if command -v apt-get &> /dev/null; then
        # Ubuntu/Debian
        sudo apt-get update
        sudo apt-get install -y nasm
    elif command -v yum &> /dev/null; then
        # CentOS/RHEL
        sudo yum install -y nasm
    elif command -v dnf &> /dev/null; then
        # Fedora
        sudo dnf install -y nasm
    elif command -v pacman &> /dev/null; then
        # Arch Linux
        sudo pacman -S nasm
    elif command -v brew &> /dev/null; then
        # macOS
        brew install nasm
    else
        print_error "Não foi possível instalar o NASM automaticamente. Por favor, instale manualmente."
        print_error "Visite: https://www.nasm.us/"
        exit 1
    fi
    print_success "NASM instalado com sucesso!"
else
    print_success "NASM já está instalado: $(nasm --version | head -n1)"
fi

# Verificar se o binutils está instalado (para o linker)
print_status "Verificando se o binutils está instalado..."
if ! command -v ld &> /dev/null; then
    print_warning "Linker não encontrado. Tentando instalar binutils..."
    
    if command -v apt-get &> /dev/null; then
        sudo apt-get install -y binutils
    elif command -v yum &> /dev/null; then
        sudo yum install -y binutils
    elif command -v dnf &> /dev/null; then
        sudo dnf install -y binutils
    elif command -v pacman &> /dev/null; then
        sudo pacman -S binutils
    elif command -v brew &> /dev/null; then
        brew install binutils
    else
        print_error "Não foi possível instalar o binutils automaticamente. Por favor, instale manualmente."
        exit 1
    fi
    print_success "Binutils instalado com sucesso!"
else
    print_success "Linker já está disponível: $(ld --version | head -n1)"
fi

# Atualizar Rust
print_status "Atualizando Rust..."
rustup update

# Compilar o projeto
print_status "Compilando RusCompile..."
cargo build --release

# Executar testes
print_status "Executando testes..."
cargo test

# Instalar o compilador
print_status "Instalando RusCompile no sistema..."
sudo cp target/release/ruscompile /usr/local/bin/
sudo chmod +x /usr/local/bin/ruscompile

# Criar diretório de exemplos se não existir
mkdir -p examples

# Compilar um exemplo para testar
print_status "Testando a instalação..."
if [ -f "examples/hello.rs" ]; then
    ruscompile examples/hello.rs -o examples/hello.s
    print_success "Exemplo compilado com sucesso!"
else
    print_warning "Arquivo de exemplo não encontrado. Criando um exemplo básico..."
    cat > examples/hello.rs << 'EOF'
func main() -> int {
    println("Hello, World!");
    return 0;
}
EOF
    ruscompile examples/hello.rs -o examples/hello.s
    print_success "Exemplo criado e compilado com sucesso!"
fi

# Verificar se a instalação foi bem-sucedida
if command -v ruscompile &> /dev/null; then
    print_success "RusCompile instalado com sucesso!"
    echo ""
    echo "🎉 Instalação concluída!"
    echo ""
    echo "Comandos disponíveis:"
    echo "  ruscompile --help          - Mostrar ajuda"
    echo "  ruscompile arquivo.rs      - Compilar um arquivo"
    echo "  ruscompile arquivo.rs -o saida.s - Especificar arquivo de saída"
    echo "  ruscompile arquivo.rs --tokens - Mostrar tokens léxicos"
    echo "  ruscompile arquivo.rs --ast - Mostrar AST"
    echo "  ruscompile arquivo.rs --assembly - Mostrar assembly"
    echo ""
    echo "Exemplo de uso:"
    echo "  ruscompile examples/hello.rs -o hello.s"
    echo "  nasm -f elf64 hello.s -o hello.o"
    echo "  ld hello.o -o hello"
    echo "  ./hello"
    echo ""
    print_success "Versão instalada: $(ruscompile --version)"
else
    print_error "Falha na instalação. Verifique as mensagens acima."
    exit 1
fi 