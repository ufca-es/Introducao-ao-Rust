# Lifetimes

Parte de **Funcionalidades Específicas** (Alan, Abner) — responsável: Abner.

Conteúdo teórico completo em [`lifetimes.md`](./lifetimes.md).

## Exemplos

| Pasta | Conceito demonstrado |
|---|---|
| [`exemplos/dangling_reference/`](./exemplos/dangling_reference) | Referência que sobreviveria ao dado original — rejeitada em tempo de compilação (comparado com C, onde compila e é UB) |
| [`exemplos/anotacao_explicita/`](./exemplos/anotacao_explicita) | Caso em que a elision não resolve sozinha e `'a` precisa ser escrito manualmente |
| [`exemplos/structs_com_referencia/`](./exemplos/structs_com_referencia) | Struct que guarda uma referência, com lifetime amarrado à declaração |
| [`exemplos/lifetime_static/`](./exemplos/lifetime_static) | `'static`, e por que ele não "desliga" a verificação do borrow checker |

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
