#!/bin/bash

# Script de desinstalação completa para RusCompile
# Este script remove o compilador, dependências, arquivos do projeto e configurações

set -e

echo "🗑️  Desinstalando RusCompile completamente..."

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

# Função para confirmar ação
confirm_action() {
    echo -e "${RED}⚠️  ATENÇÃO CRÍTICA:${NC} Esta ação irá remover COMPLETAMENTE o RusCompile do sistema."
    echo -e "Isso inclui:"
    echo -e "  • Executável do compilador"
    echo -e "  • Arquivos de compilação e cache"
    echo -e "  • Dependências (NASM, binutils)"
    echo -e "  • Arquivos do projeto atual"
    echo -e "  • Configurações e aliases"
    echo -e "  • Entradas do PATH"
    echo ""
    echo -e "${RED}⚠️  Esta ação é IRREVERSÍVEL!${NC}"
    echo ""
    read -p "Tem certeza ABSOLUTA que deseja continuar? (digite 'SIM' para confirmar): " -r
    echo
    if [[ ! $REPLY =~ ^SIM$ ]]; then
        print_status "Desinstalação cancelada pelo usuário."
        exit 0
    fi
    
    echo -e "${RED}⚠️  ÚLTIMA CHANCE:${NC} Todos os dados serão perdidos!"
    read -p "Confirma a desinstalação completa? (digite 'CONFIRMO'): " -r
    echo
    if [[ ! $REPLY =~ ^CONFIRMO$ ]]; then
        print_status "Desinstalação cancelada pelo usuário."
        exit 0
    fi
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
    
    for path in "${executable_paths[@]}"; do
        if [ -f "$path" ]; then
            print_status "Removendo executável: $path"
            sudo rm -f "$path"
            print_success "Executável removido: $path"
        fi
    done
}

# Remover diretórios de instalação
remove_directories() {
    local directories=(
        "/opt/ruscompile"
        "$HOME/.ruscompile"
        "/usr/local/share/ruscompile"
        "/usr/share/ruscompile"
        "/usr/local/lib/ruscompile"
        "/usr/lib/ruscompile"
    )
    
    for dir in "${directories[@]}"; do
        if [ -d "$dir" ]; then
            print_status "Removendo diretório: $dir"
            sudo rm -rf "$dir"
            print_success "Diretório removido: $dir"
        fi
    done
}

# Limpar arquivos de cache e temporários
clean_cache() {
    print_status "Limpando arquivos de cache e temporários..."
    
    # Limpar cache do Cargo (se existir)
    if [ -d "$HOME/.cargo/registry/cache" ]; then
        print_status "Limpando cache do Cargo..."
        cargo clean
    fi
    
    # Remover arquivos temporários do projeto atual
    if [ -d "target" ]; then
        print_status "Removendo arquivos de compilação..."
        cargo clean
    fi
    
    # Remover arquivos de assembly gerados
    if [ -d "examples" ]; then
        print_status "Removendo arquivos de assembly..."
        find examples -name "*.s" -delete
        find examples -name "*.o" -delete
        find examples -name "*.out" -delete
        find examples -name "*.exe" -delete
    fi
    
    # Remover arquivos temporários do sistema
    local temp_files=(
        "/tmp/ruscompile*"
        "/var/tmp/ruscompile*"
        "$HOME/.cache/ruscompile*"
    )
    
    for pattern in "${temp_files[@]}"; do
        if ls $pattern 1> /dev/null 2>&1; then
            print_status "Removendo arquivos temporários: $pattern"
            rm -f $pattern
        fi
    done
}

# Remover entradas do PATH
clean_path() {
    local shell_configs=(
        "$HOME/.bashrc"
        "$HOME/.bash_profile"
        "$HOME/.zshrc"
        "$HOME/.profile"
        "$HOME/.bash_login"
    )
    
    for config in "${shell_configs[@]}"; do
        if [ -f "$config" ]; then
            if grep -q "ruscompile" "$config"; then
                print_status "Removendo entradas do PATH em: $config"
                # Criar backup
                cp "$config" "${config}.backup.$(date +%Y%m%d_%H%M%S)"
                # Remover linhas relacionadas ao ruscompile
                sed -i '/ruscompile/d' "$config"
                print_success "Entradas removidas de: $config"
            fi
        fi
    done
}

# Remover aliases e funções
clean_aliases() {
    local shell_configs=(
        "$HOME/.bashrc"
        "$HOME/.bash_profile"
        "$HOME/.zshrc"
        "$HOME/.bash_login"
    )
    
    for config in "${shell_configs[@]}"; do
        if [ -f "$config" ]; then
            if grep -q "alias.*ruscompile\|function.*ruscompile" "$config"; then
                print_status "Removendo aliases e funções em: $config"
                sed -i '/alias.*ruscompile/d; /function.*ruscompile/d' "$config"
                print_success "Aliases e funções removidos de: $config"
            fi
        fi
    done
}

# Remover dependências
remove_dependencies() {
    print_status "Verificando e removendo dependências..."
    
    local dependencies=(
        "nasm"
        "binutils"
    )
    
    for dep in "${dependencies[@]}"; do
        if command -v "$dep" &> /dev/null; then
            print_status "Removendo $dep..."
            
            if command -v apt-get &> /dev/null; then
                sudo apt-get remove -y "$dep"
                sudo apt-get autoremove -y
            elif command -v yum &> /dev/null; then
                sudo yum remove -y "$dep"
            elif command -v dnf &> /dev/null; then
                sudo dnf remove -y "$dep"
            elif command -v pacman &> /dev/null; then
                sudo pacman -R "$dep"
            elif command -v brew &> /dev/null; then
                brew uninstall "$dep"
            else
                print_warning "Não foi possível remover $dep automaticamente."
            fi
            
            print_success "$dep removido!"
        fi
    done
}

# Remover arquivos do projeto atual
remove_project_files() {
    print_status "Removendo arquivos do projeto atual..."
    
    # Lista de arquivos e diretórios a serem removidos
    local project_items=(
        "target"
        "Cargo.lock"
        "examples/*.s"
        "examples/*.o"
        "examples/*.out"
        "examples/*.exe"
        "*.s"
        "*.o"
        "*.out"
        "*.exe"
    )
    
    for item in "${project_items[@]}"; do
        if [ -e "$item" ] || ls $item 1> /dev/null 2>&1; then
            print_status "Removendo: $item"
            rm -rf $item
        fi
    done
    
    print_success "Arquivos do projeto removidos!"
}

# Limpar configurações do sistema
clean_system_config() {
    print_status "Limpando configurações do sistema..."
    
    # Remover entradas de menu (se existirem)
    local menu_files=(
        "/usr/share/applications/ruscompile.desktop"
        "$HOME/.local/share/applications/ruscompile.desktop"
    )
    
    for file in "${menu_files[@]}"; do
        if [ -f "$file" ]; then
            print_status "Removendo entrada de menu: $file"
            sudo rm -f "$file"
        fi
    done
    
    # Remover man pages (se existirem)
    local man_dirs=(
        "/usr/local/share/man/man1"
        "/usr/share/man/man1"
    )
    
    for dir in "${man_dirs[@]}"; do
        if [ -f "$dir/ruscompile.1" ]; then
            print_status "Removendo man page: $dir/ruscompile.1"
            sudo rm -f "$dir/ruscompile.1"
        fi
    done
    
    # Atualizar cache de man pages
    if command -v mandb &> /dev/null; then
        sudo mandb
    fi
}

# Função principal
main() {
    echo "=========================================="
    echo "    DESINSTALADOR RUSCOMPILE (COMPLETO)"
    echo "=========================================="
    echo ""
    
    # Confirmar ação
    confirm_action
    
    # Verificar instalação
    check_installation
    
    # Executar etapas de desinstalação
    print_status "Iniciando processo de desinstalação completa..."
    
    # 1. Remover executável
    remove_executable
    
    # 2. Remover diretórios
    remove_directories
    
    # 3. Limpar cache e arquivos temporários
    clean_cache
    
    # 4. Limpar PATH e aliases
    clean_path
    clean_aliases
    
    # 5. Remover dependências
    remove_dependencies
    
    # 6. Remover arquivos do projeto
    remove_project_files
    
    # 7. Limpar configurações do sistema
    clean_system_config
    
    # Verificar se a desinstalação foi bem-sucedida
    if ! command -v ruscompile &> /dev/null; then
        print_success "✅ RusCompile foi completamente desinstalado!"
        echo ""
        echo "🎉 Desinstalação completa concluída!"
        echo ""
        echo "O que foi removido:"
        echo "  • Executável do compilador"
        echo "  • Diretórios de instalação"
        echo "  • Arquivos de cache e temporários"
        echo "  • Entradas do PATH"
        echo "  • Aliases e funções"
        echo "  • Dependências (NASM, binutils)"
        echo "  • Arquivos do projeto atual"
        echo "  • Configurações do sistema"
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
    print_error "Execute como usuário normal: ./uninstall-all.sh"
    exit 1
fi

# Executar função principal
main "$@" 