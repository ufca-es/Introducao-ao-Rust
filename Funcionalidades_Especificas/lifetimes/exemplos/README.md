# Exemplos de Lifetimes (cap. 10.3 do Rust Book)

Réplica dos exemplos de "Validating References with Lifetimes" (cap.
10.3 do Rust Book), cada um comparado com o comportamento equivalente
em C. Continuação natural dos notebooks de Ownership e Borrowing
(caps. 4.1 e 4.2).

Cada pasta é um notebook: um projeto Cargo independente que reúne
diversos exemplos sobre o mesmo conceito, um por arquivo em `src/bin/`,
cada um com sua própria `fn main()` — como células independentes de um
notebook, cada exemplo roda sozinho, sem depender dos outros. A pasta
também contém um único `comparacao.c`, com o equivalente em C de todos
os cenários daquele notebook, dividido em seções comentadas.

| Pasta (notebook) | Conceito (Rust Book) | O que compara com C |
|---|---|---|
| `anotacao_explicita` | Generic Lifetimes in Functions | O exemplo central: uma função com duas referências de entrada e uma de saída não compila sem anotação (`E0106`) — nenhuma das regras de elision resolve sozinha. O `comparacao.c` mostra a mesma função em C compilando sem exigir nada, mesmo quando devolve ponteiro para uma variável local (`-Wreturn-local-addr`, comportamento indefinido em runtime). Também contrasta com o caso de uma entrada só, em que a elision resolve sozinha nas duas linguagens. |
| `structs_com_referencia` | Lifetime Annotations in Struct Definitions | Uma struct que guarda referência precisa declarar `<'a>`; usá-la depois que o dado referenciado sai de escopo não compila (`E0597`). O `comparacao.c` reproduz o mesmo cenário com um struct em C guardando um ponteiro — sem NENHUM aviso do compilador, o programa "parece funcionar" por acaso (a memória da stack ainda não foi reaproveitada), o que é ainda mais perigoso que um erro visível. |
| `lifetime_static` | The Static Lifetime | String literals são `'static` de verdade (ficam gravadas no binário) nas duas linguagens. Já tentar devolver `&'static str` de uma `String` criada em runtime não compila em Rust (`E0515`); em C, o `comparacao.c` faz o equivalente e trava com segmentation fault (código de saída 139) ao tentar usar o ponteiro pendurado. |

## Como executar

Rust, dentro de cada pasta: liste os exemplos disponíveis e rode um por
vez pelo nome do arquivo em `src/bin/`.

```
ls src/bin
cargo run --bin <nome_do_exemplo>
```

Por exemplo, dentro de `anotacao_explicita/`:

```
cargo run --bin exemplo2_com_anotacao
```

C, dentro de cada pasta (um único arquivo cobre todos os exemplos do
notebook, em seções comentadas):

```
gcc -Wall -Wextra -Wpedantic comparacao.c -o comparacao && ./comparacao
```

**Aviso:** `anotacao_explicita/exemplo1_sem_anotacao` e
`structs_com_referencia/exemplo2_struct_invalida` e
`lifetime_static/exemplo2_static_incorreto` **não compilam de
propósito** — são o ponto central de cada notebook, não um bug.

**Aviso:** `lifetime_static/comparacao.c` trava de propósito com uma
segmentation fault (código de saída 139) ao desreferenciar um ponteiro
para um frame de stack já desfeito — mesmo ponto central do exemplo
`exemplo2_static_incorreto.rs`, só que em C o programa nem chega a
avisar antes de travar.
