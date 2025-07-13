# Makefile para RusCompile
# Comandos úteis para desenvolvimento

.PHONY: help build release test clean bench docs examples install uninstall

# Variáveis
BINARY_NAME = ruscompile
TARGET_DIR = target
RELEASE_DIR = $(TARGET_DIR)/release
DEBUG_DIR = $(TARGET_DIR)/debug

# Comando padrão
help:
	@echo "Comandos disponíveis:"
	@echo "  build     - Compilar em modo debug"
	@echo "  release   - Compilar em modo release"
	@echo "  test      - Executar testes"
	@echo "  bench     - Executar benchmarks"
	@echo "  clean     - Limpar arquivos de compilação"
	@echo "  docs      - Gerar documentação"
	@echo "  examples  - Compilar exemplos"
	@echo "  install   - Instalar o compilador"
	@echo "  uninstall - Desinstalar o compilador"
	@echo "  run       - Executar exemplo hello"
	@echo "  check     - Verificar código sem compilar"
	@echo "  setup     - Configurar ambiente completo"
	@echo "  remove    - Remover completamente"

# Compilar em modo debug
build:
	cargo build

# Compilar em modo release
release:
	cargo build --release

# Executar testes
test:
	cargo test
	cargo test --test integration_tests

# Executar benchmarks
bench:
	cargo bench

# Limpar arquivos de compilação
clean:
	cargo clean
	rm -f examples/*.s examples/*.o examples/*.out

# Gerar documentação
docs:
	cargo doc --open

# Compilar exemplos
examples: release
	@echo "Compilando exemplos..."
	./$(RELEASE_DIR)/$(BINARY_NAME) examples/hello.rs -o examples/hello.s
	@echo "Exemplos compilados!"

# Executar exemplo hello
run: examples
	@echo "Executando exemplo hello..."
	@echo "Assembly gerado:"
	@cat examples/hello.s

# Verificar código sem compilar
check:
	cargo check
	cargo clippy

# Instalar o compilador
install: release
	@echo "Instalando ruscompile..."
	sudo cp $(RELEASE_DIR)/$(BINARY_NAME) /usr/local/bin/
	@echo "RusCompile instalado em /usr/local/bin/"

# Desinstalar o compilador
uninstall:
	@echo "Desinstalando ruscompile..."
	./uninstall.sh

# Configurar ambiente completo
setup: deps build test examples
	@echo "✅ Ambiente configurado com sucesso!"

# Remover completamente
remove: uninstall clean-all
	@echo "🗑️  Remoção completa concluída!"

# Compilar e executar um arquivo específico
%.s: %.rs
	./$(RELEASE_DIR)/$(BINARY_NAME) $< -o $@

# Compilar assembly para executável
%.out: %.s
	nasm -f elf64 $< -o $(<:.s=.o)
	ld $(<:.s=.o) -o $@
	rm $(<:.s=.o)

# Executar um programa compilado
%.run: %.out
	./$<

# Desenvolvimento rápido
dev: build test check

# Preparar para release
prepare-release: clean release test bench docs

# Verificar dependências
deps:
	@echo "Verificando dependências..."
	@which cargo > /dev/null || (echo "Cargo não encontrado. Instale Rust primeiro." && exit 1)
	@which nasm > /dev/null || (echo "NASM não encontrado. Instale NASM para compilar assembly." && exit 1)
	@which ld > /dev/null || (echo "Linker não encontrado. Instale binutils." && exit 1)
	@echo "Todas as dependências estão instaladas!"

# Formatar código
fmt:
	cargo fmt

# Verificar formatação
fmt-check:
	cargo fmt -- --check

# Análise de segurança
audit:
	cargo audit

# Estatísticas do projeto
stats:
	@echo "Estatísticas do projeto:"
	@echo "Linhas de código Rust:"
	@find src -name "*.rs" -exec wc -l {} + | tail -1
	@echo "Arquivos de teste:"
	@find tests -name "*.rs" -exec wc -l {} + | tail -1
	@echo "Exemplos:"
	@find examples -name "*.rs" -exec wc -l {} + | tail -1

# Criar release
create-release: prepare-release
	@echo "Criando release..."
	@version=$$(grep '^version = ' Cargo.toml | cut -d'"' -f2); \
	echo "Versão: $$version"; \
	tar -czf ruscompile-$$version.tar.gz \
		--exclude=target \
		--exclude=.git \
		--exclude=*.s \
		--exclude=*.o \
		--exclude=*.out \
		.

# Limpeza completa
clean-all: clean
	rm -f *.tar.gz
	rm -f *.zip
	rm -rf docs/

# Verificar se tudo está funcionando
verify: deps build test examples
	@echo "✅ Tudo funcionando corretamente!"

# Comando para desenvolvimento contínuo
watch:
	@echo "Observando mudanças nos arquivos..."
	@while inotifywait -r -e modify src/ tests/ examples/; do \
		echo "Mudanças detectadas, executando testes..."; \
		cargo test; \
	done 