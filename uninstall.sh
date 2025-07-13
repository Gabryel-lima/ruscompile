#!/bin/bash

# Script de desinstalação simples para RusCompile
# Remove apenas o executável do compilador do sistema

set -e

echo "🗑️  Desinstalando RusCompile..."

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

# Verificar se o RusCompile está instalado
check_installation() {
    if command -v ruscompile &> /dev/null; then
        print_status "RusCompile encontrado: $(which ruscompile)"
        return 0
    else
        print_warning "RusCompile não foi encontrado no PATH do sistema."
        return 1
    fi
}

# Remover executável do sistema
remove_executable() {
    local executable_paths=(
        "/usr/local/bin/ruscompile"
        "/usr/bin/ruscompile"
        "/opt/ruscompile/bin/ruscompile"
        "$HOME/.local/bin/ruscompile"
    )
    
    local removed=false
    
    for path in "${executable_paths[@]}"; do
        if [ -f "$path" ]; then
            print_status "Removendo executável: $path"
            sudo rm -f "$path"
            print_success "Executável removido: $path"
            removed=true
        fi
    done
    
    if [ "$removed" = false ]; then
        print_warning "Nenhum executável foi encontrado nos locais padrão."
    fi
}

# Função principal
main() {
    echo "=========================================="
    echo "    DESINSTALADOR RUSCOMPILE (SIMPLES)"
    echo "=========================================="
    echo ""
    
    # Verificar instalação
    if ! check_installation; then
        print_warning "RusCompile não parece estar instalado no sistema."
        echo ""
        echo "Este script remove apenas o executável do compilador."
        echo "Para uma desinstalação completa, use o script completo."
        exit 0
    fi
    
    # Confirmar ação
    echo -e "${YELLOW}⚠️  ATENÇÃO:${NC} Esta ação irá remover o executável do RusCompile do sistema."
    echo ""
    read -p "Tem certeza que deseja continuar? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_status "Desinstalação cancelada pelo usuário."
        exit 0
    fi
    
    # Remover executável
    print_status "Removendo executável do sistema..."
    remove_executable
    
    # Verificar se a desinstalação foi bem-sucedida
    if ! command -v ruscompile &> /dev/null; then
        print_success "✅ RusCompile foi desinstalado com sucesso!"
        echo ""
        echo "🎉 Desinstalação concluída!"
        echo ""
        echo "O executável do compilador foi removido do sistema."
        echo ""
        echo "Para reinstalar, execute:"
        echo "  ./install.sh"
        echo ""
        print_success "Obrigado por usar RusCompile! 👋"
    else
        print_error "❌ Falha na desinstalação. RusCompile ainda está disponível."
        print_error "Verifique as mensagens acima e tente novamente."
        exit 1
    fi
}

# Verificar se o script está sendo executado como root
if [ "$EUID" -eq 0 ]; then
    print_error "Este script não deve ser executado como root."
    print_error "Execute como usuário normal: ./uninstall.sh"
    exit 1
fi

# Executar função principal
main "$@" 