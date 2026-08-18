# Lifetimes

Parte de **Funcionalidades Específicas** (Alan, Abner) — responsável: Abner.

Conteúdo teórico completo em [`lifetimes.md`](./lifetimes.md).

## Exemplos

| Pasta | Conceito demonstrado |
|---|---|
| [`exemplos/anotacao_explicita/`](./exemplos/anotacao_explicita) | Caso em que a elision não resolve sozinha e `'a` precisa ser escrito manualmente |
| [`exemplos/structs_com_referencia/`](./exemplos/structs_com_referencia) | Struct que guarda uma referência, com lifetime amarrado à declaração |
| [`exemplos/lifetime_static/`](./exemplos/lifetime_static) | `'static`, e por que ele não "desliga" a verificação do borrow checker |

O exemplo de **dangling reference** não está duplicado aqui — ele já existe
em [`Rust_x_C/dangling-reference/`](../../Rust_x_C/dangling-reference),
feito pelo Matheus como parte da comparação Rust × C, e é referenciado a
partir de `lifetimes.md`.

## Como rodar

Cada exemplo é um arquivo `.rs` (ou `.c`, no caso do par comparativo)
independente:

```bash
rustc caminho/do/arquivo.rs
./arquivo   # quando o exemplo compila
```

O exemplo em `dangling_reference/dangling.rs` **não compila de propósito**
— o objetivo é mostrar a mensagem de erro do `rustc` ao vivo na
apresentação.
