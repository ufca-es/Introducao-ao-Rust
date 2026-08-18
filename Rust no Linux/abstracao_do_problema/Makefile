CC = gcc
CFLAGS = -Wall -Wextra -pedantic -std=c11
RUSTC = rustc

.PHONY: all executar demonstrar-quebra limpar

all: driver_c driver_rust

camada_dma.o: camada_dma.c camada_dma.h
	$(CC) $(CFLAGS) -c camada_dma.c -o camada_dma.o

driver_c: driver_c.c camada_dma.o
	$(CC) $(CFLAGS) driver_c.c camada_dma.o -o driver_c

driver_rust: driver_rust.rs camada_dma.o
	$(RUSTC) --edition 2021 driver_rust.rs -C link-arg=camada_dma.o -o driver_rust

executar: all
	@echo "\n=== Driver escrito em C ==="
	./driver_c
	@echo "\n=== Driver escrito em Rust ==="
	./driver_rust

# Esta compilacao deve falhar: o binding procura um simbolo C que nao existe mais.
demonstrar-quebra: camada_dma.o
	@echo "Tentando compilar um binding que ainda usa o nome antigo..."
	@if $(RUSTC) --edition 2021 binding_desatualizado.rs -C link-arg=camada_dma.o -o binding_desatualizado; then \
		rm -f binding_desatualizado binding_desatualizado.*.rcgu.o; \
		echo "ERRO: a compilacao deveria ter falhado."; \
		exit 1; \
	else \
		rm -f binding_desatualizado binding_desatualizado.*.rcgu.o; \
		echo ""; \
		echo "RESULTADO ESPERADO: a API C mudou, e o binding Rust antigo quebrou."; \
		echo "A equipe Rust deve atualizar o binding, como foi feito em driver_rust.rs."; \
	fi

limpar:
	rm -f camada_dma.o driver_c driver_rust binding_desatualizado binding_desatualizado.*.rcgu.o
