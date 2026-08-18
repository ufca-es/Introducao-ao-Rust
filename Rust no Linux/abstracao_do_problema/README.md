# Abstracao simples do debate Rust for Linux

Este pequeno projeto **nao e um modulo real do kernel nem executa DMA real**.
Ele e uma simulacao em espaco de usuario para mostrar, com pouco codigo, a
relacao apresentada em `Rust_For_Linux.md`:

```text
                   camada_dma.c
                    API em C
                    /      \
                   /        \
          driver_c.c      binding Rust
                              |
                              v
                        driver_rust.rs
```

## O papel de cada arquivo

- `camada_dma.c` e `camada_dma.h`: simulam a API DMA mantida pela equipe C.
- `driver_c.c`: representa um usuario da API escrito em C.
- `driver_rust.rs`: declara o binding e cria uma pequena abstracao segura.
- `binding_desatualizado.rs`: representa um binding que nao acompanhou uma
  mudanca de nome da API C.
- `Makefile`: guarda os comandos de compilacao e execucao.

## Executar o exemplo correto

Dentro desta pasta, execute:

```bash
make limpar
make executar
```

Os dois drivers enviam os mesmos quatro numeros para a mesma camada C. A
diferenca e que o driver C chama a API diretamente, enquanto o Rust precisa
de uma declaracao `extern "C"` (o binding).

## Demonstrar o problema de manutencao

Imagine que a funcao C antes se chamava `dma_transferir` e passou a se chamar
`dma_transferir_v2`. O driver C e o binding Rust correto ja foram atualizados,
mas `binding_desatualizado.rs` ainda usa o nome antigo.

Execute:

```bash
make demonstrar-quebra
```

A mensagem `undefined symbol: dma_transferir` (ou
`undefined reference to dma_transferir`) e **o resultado esperado**.
Ela mostra que a camada C continua compilando, mas o binding Rust precisa ser
corrigido pela equipe Rust. O arquivo `driver_rust.rs` mostra a correcao: seu
binding declara `dma_transferir_v2`.

Para voltar ao exemplo funcional, basta executar:

```bash
make executar
```

## Onde aparece a seguranca do Rust

A funcao C recebe um ponteiro e uma quantidade. O Rust concentra a parte
arriscada em uma unica chamada `unsafe` e oferece a funcao segura
`transferir_com_seguranca`, que recebe uma fatia (`&[i32]`) e rejeita uma lista
vazia. Isso ilustra a ideia de criar abstracoes Rust seguras sobre APIs C.
