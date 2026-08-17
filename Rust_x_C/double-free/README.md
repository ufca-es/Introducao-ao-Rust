# Double free: C x Rust

Um *double free* acontece quando um programa tenta liberar duas vezes a mesma
alocação. Depois da primeira liberação, o ponteiro não identifica mais um
objeto válido. Entregá-lo novamente a `free` produz comportamento indefinido.

## Versão incorreta em C

O arquivo [`c/double_free.c`](./c/double_free.c) chama `free(valor)` duas vezes.
Compiladores normalmente não conseguem provar que os dois ponteiros usados em
um programa real representam a mesma alocação.

```bash
gcc -std=c11 -Wall -Wextra -Wpedantic c/double_free.c -o double_free
```

Com GCC 15.2.0, esse exemplo gera um aviso `-Wuse-after-free`. Outros
compiladores, versões ou construções mais complexas podem gerar diagnósticos
diferentes ou nenhum aviso. Ainda que um executável seja produzido, ele não
deve ser executado como se fosse um programa válido.

Ferramentas como AddressSanitizer podem ajudar a detectar o erro durante uma
execução de teste, mas essa detecção dinâmica cobre apenas os caminhos realmente
executados:

```bash
gcc -std=c11 -g -fsanitize=address c/double_free.c -o double_free_asan
```

## Versão corrigida em C

Em [`c/fixed.c`](./c/fixed.c), há uma única chamada a `free`. O ponteiro é
definido como `NULL` depois da liberação para reduzir o risco de reutilização
acidental naquele escopo. Isso é uma prática defensiva, não um mecanismo geral
de ownership: outras cópias do mesmo endereço continuariam penduradas.

```bash
gcc -std=c11 -Wall -Wextra -Wpedantic c/fixed.c -o fixed_c
./fixed_c
```

Saída esperada:

```text
Valor: 42
```

## Tentativa equivalente em Rust

No arquivo [`rust/double_free.rs`](./rust/double_free.rs), a primeira chamada a
`drop(valor)` recebe a posse do `Box<i32>` e encerra seu tempo de vida. A segunda
chamada tenta usar um valor que já foi movido, por isso o compilador rejeita o
programa:

```bash
rustc rust/double_free.rs
```

O diagnóstico verificado com Rust 1.97.1 é `E0382`, uso de valor movido. Em safe
Rust, não é possível chamar o destrutor do mesmo `Box` uma segunda vez dessa
forma. A [saída registrada](./rust/erro_compilacao.txt) foi produzida pelo código
deste exemplo.

## Versão corrigida em Rust

O arquivo [`rust/fixed.rs`](./rust/fixed.rs) apenas utiliza o valor. Não é
necessário chamar `drop` explicitamente: quando o dono sai de escopo, o destrutor
de `Box` libera a alocação uma única vez.

```bash
rustc rust/fixed.rs -o fixed_rust
./fixed_rust
```

Saída verificada:

```text
Valor: 42
```

## O que o exemplo demonstra

- C depende do programador e de ferramentas auxiliares para manter o ciclo de
  vida da alocação correto;
- avisos do compilador C são úteis, mas não constituem uma garantia da
  linguagem;
- `drop` consome o valor em Rust, impedindo um segundo uso em código seguro;
- a destruição automática por escopo torna explícito qual valor controla o
  recurso.
