# Exemplos de Ownership (cap. 4.1 e 4.2 do Rust Book)

Réplica dos exemplos de
["What is Ownership?"](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
e
["References and Borrowing"](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
(caps. 4.1 e 4.2 do Rust Book), cada um comparado com o comportamento
equivalente em C.

Cada pasta é um **notebook**: um projeto Cargo independente que reúne
cinco exemplos sobre o mesmo conceito, um por arquivo em `src/bin/`,
cada um com sua própria `fn main()` — como células independentes de um
notebook, cada exemplo roda sozinho, sem depender dos outros. A pasta
também contém um único `comparacao.c`, com o equivalente em C dos
cenários daquele notebook que têm contraparte direta na linguagem,
dividido em seções comentadas.

Todo exemplo começa com um link para o [Rust
Playground](https://play.rust-lang.org/) já com o código carregado, para
rodar direto no navegador sem instalar nada.

| Pasta (notebook) | Conceito (Rust Book) | O que compara com C |
|---|---|---|
| [`escopo_e_drop`](./escopo_e_drop) | Variable Scope / Memory and Allocation | Escopo de bloco é igual nas duas linguagens; `drop` automático do Rust ao final do escopo (inclusive a ordem LIFO entre vários valores) vs. `free()` manual em C — esquecê-lo vaza memória sem nenhum aviso do compilador. |
| [`move`](./move) | Ways Variables and Data Interact: Move / Stack-Only Data: Copy | O exemplo central: `let s2 = s1;` invalida `s1` em Rust (erro `E0382` em tempo de compilação) para evitar um *double free*. Em C a mesma atribuição é só uma cópia rasa de ponteiro — o `comparacao.c` **provoca de fato um double free** (`free(): double free detected in tcache 2`, processo abortado) para mostrar em tempo de execução o que o Rust barra em tempo de compilação. Também contrasta com tipos `Copy` (como `i32`), que se comportam igual nas duas linguagens. |
| [`strings`](./strings) | The String Type / Variables and Data Interacting with Clone | `String` é o exemplo canônico de dado de tamanho desconhecido e expansível: literal (`&str`, embutido no binário) vs. `String` (heap, crescível via `push_str`/`push`, com realocação de `capacity()`) vs. buffer manual com `malloc`/`realloc`/`strcat` em C. Também mostra `clone()` como cópia profunda explícita — em C precisa ser escrita à mão (`clonar()`). |
| [`ownership_em_funcoes`](./ownership_em_funcoes) | Ownership and Functions (Listing 4-3) | Passar uma `String` para uma função move a posse; usar a variável depois não compila em Rust. Em C, o `comparacao.c` mostra (comentado, por segurança) um *use-after-free* real: o código compilaria e rodaria mesmo com o bug. |
| [`retorno_e_escopo`](./retorno_e_escopo) | Return Values and Scope (Listing 4-4 e 4-5) | Devolver posse via `return` e o padrão de tupla `(String, usize)` para "usar e devolver" um parâmetro. Em C não há tuplas: o equivalente usa um parâmetro de saída (`char **s_devolvida`), e cabe inteiramente ao programador rastrear qual `free()` corresponde a qual `malloc()`. |
| [`referencias_e_borrowing`](./referencias_e_borrowing) | References and Borrowing — cap. 4.2 | `&s1` empresta uma referência sem tomar posse, então `s1` continua válida após a chamada (diferente de `ownership_em_funcoes`); múltiplas referências imutáveis convivem sem problema. Em C, passar um ponteiro também não transfere posse, mas a imutabilidade de `&String` é uma regra do compilador; o `const` de C é só convenção e pode ser descartado com um cast, sem nenhum aviso. |
| [`referencias_mutaveis`](./referencias_mutaveis) | Mutable References — cap. 4.2 | `&mut s` permite mutar através de uma referência, mas só pode existir uma referência mutável (ou várias imutáveis, nunca as duas ao mesmo tempo) por escopo — regra que evita data races em tempo de compilação (`E0499`/`E0502`), e cujo escopo termina no último uso (NLL). Em C nada impede dois ponteiros mutáveis para o mesmo buffer: o `comparacao.c` mostra um `realloc()` através de um deles invalidando o outro, um use-after-free silencioso. |
| [`referencias_pendentes`](./referencias_pendentes) | Dangling References — cap. 4.2 | O compilador do Rust rejeita (`E0106`) uma função que devolve `&String` para uma variável local, porque o dado seria liberado antes da referência ser usada. Em C o `comparacao.c` faz exatamente isso — devolve um ponteiro para um buffer da stack — e o GCC só emite um aviso (`-Wreturn-local-addr`); o binário compila e o comportamento em tempo de execução é indefinido. |

Planejado para a próxima seção do capítulo 4 (fora do escopo de 4.1 e 4.2):
`lifetimes` (cap. 4.3).

## Como executar

Rust, dentro de cada pasta: liste os exemplos disponíveis e rode um por
vez pelo nome do arquivo em `src/bin/`.

```bash
ls src/bin
cargo run --bin <nome_do_exemplo>
```

Por exemplo, dentro de `strings/`:

```bash
cargo run --bin exemplo1_literal_vs_string
```

C, dentro de cada pasta (um único arquivo cobre os cenários do notebook
que têm contraparte em C, em seções comentadas):

```bash
gcc -Wall -Wextra comparacao.c -o comparacao && ./comparacao
```

> **Aviso:** a última seção de `move/comparacao.c` trava de propósito com
> um *double free* (código de saída 134) — é o ponto central do exemplo,
> não um bug.
>
> **Aviso:** `referencias_pendentes/comparacao.c` também trava de
> propósito, com uma *segmentation fault* (código de saída 139) ao
> desreferenciar um ponteiro para um frame de stack já desfeito — mesmo
> ponto central, mesmo motivo.
