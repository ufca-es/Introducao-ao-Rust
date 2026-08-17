# Exemplos de Ownership (cap. 4.1 do Rust Book)

Réplica dos exemplos de
["What is Ownership?"](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
(cap. 4.1 do Rust Book), cada um comparado com o comportamento equivalente
em C.

Cada pasta é um projeto Cargo independente, contendo apenas o código-fonte e
o `Cargo.toml`, sem binários ou pastas geradas pelo Cargo — e também um
`comparacao.c` com o equivalente em C do mesmo cenário.

| Pasta | Conceito (Rust Book) | O que compara com C |
|---|---|---|
| [`escopo_e_drop`](./escopo_e_drop) | Variable Scope / Memory and Allocation | Escopo de bloco é igual nas duas linguagens; `drop` automático do Rust ao final do escopo vs. `free()` manual em C — esquecê-lo vaza memória sem nenhum aviso do compilador. |
| [`tipo_string`](./tipo_string) | The String Type | Literal imutável embutido no binário nas duas linguagens; `String` (heap, crescível) vs. buffer manual com `malloc`/`realloc`/`strcat`. O comentário mostra que mutar um literal em C compila sem nenhum aviso e falha em tempo de execução (segfault) — em Rust nem compila. |
| [`move`](./move) | Ways Variables and Data Interact: Move | O exemplo central: `let s2 = s1;` invalida `s1` em Rust (erro `E0382` em tempo de compilação) para evitar um *double free*. Em C a mesma atribuição é só uma cópia rasa de ponteiro — o `comparacao.c` **provoca de fato um double free** (`free(): double free detected in tcache 2`, processo abortado) para mostrar em tempo de execução o que o Rust barra em tempo de compilação. |
| [`clone_e_copy`](./clone_e_copy) | Variables and Data Interacting with Clone / Stack-Only Data: Copy | `clone()` é uma cópia profunda explícita — em C precisa ser escrita à mão (`clonar()`). Já `Copy` (tipos como `i32`) se comporta exatamente como a atribuição de um `int` em C: nenhuma diferença de comportamento entre as linguagens aqui. |
| [`ownership_em_funcoes`](./ownership_em_funcoes) | Ownership and Functions (Listing 4-3) | Passar uma `String` para uma função move a posse; usar a variável depois não compila em Rust. Em C, o `comparacao.c` mostra (comentado, por segurança) um *use-after-free* real: o código compilaria e rodaria mesmo com o bug. |
| [`retorno_e_escopo`](./retorno_e_escopo) | Return Values and Scope (Listing 4-4 e 4-5) | Devolver posse via `return` e o padrão de tupla `(String, usize)` para "usar e devolver" um parâmetro. Em C não há tuplas: o equivalente usa um parâmetro de saída (`char **s_devolvida`), e cabe inteiramente ao programador rastrear qual `free()` corresponde a qual `malloc()`. |

Planejado para as próximas seções do capítulo 4 (fora do escopo de 4.1):
`borrowing_checker` (referências e regras de borrowing, cap. 4.2) e
`lifetimes` (cap. 4.3).

## Como executar

Rust, dentro de cada pasta:

```bash
cargo run
```

C, dentro de cada pasta:

```bash
gcc -Wall -Wextra comparacao.c -o comparacao && ./comparacao
```

> **Aviso:** `move/comparacao.c` trava de propósito com um *double free*
> (código de saída 134) — é o ponto central do exemplo, não um bug.
