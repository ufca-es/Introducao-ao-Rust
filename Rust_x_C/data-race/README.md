# Data race: C x Rust

Uma *data race* ocorre quando threads acessam simultaneamente o mesmo objeto,
pelo menos um dos acessos realiza uma escrita e não há sincronização adequada.
Em C, uma data race produz comportamento indefinido. O resultado numérico
observado é apenas um possível sintoma, não a definição do erro.

## Versão incorreta em C

Em [`c/data_race.c`](./c/data_race.c), duas threads incrementam o mesmo contador
global sem mutex ou operação atômica:

```bash
gcc -std=c11 -Wall -Wextra -Wpedantic -pthread c/data_race.c -o data_race
```

O compilador pode gerar o executável sem identificar o conflito. O programa não
deve ser considerado um teste confiável, mesmo que alguma execução imprima o
valor esperado. As threads realizam leituras e escritas conflitantes, e a
linguagem não define o comportamento resultante.

Adicionar `volatile` ao contador não corrigiria o problema. `volatile` não
fornece atomicidade nem estabelece sincronização entre threads.

## Versão corrigida em C

O arquivo [`c/fixed.c`](./c/fixed.c) protege cada incremento com um
`pthread_mutex_t` da biblioteca POSIX Threads. Assim, somente uma thread
modifica o contador por vez.

```bash
gcc -std=c11 -Wall -Wextra -Wpedantic -pthread c/fixed.c -o fixed_c
./fixed_c
```

Saída esperada:

```text
Contador: 2000000
```

Os exemplos usam POSIX Threads porque o suporte a `<threads.h>` do C11 é
opcional e não está presente em todos os toolchains. No Windows, eles podem ser
compilados pelo GCC do MSYS2/MinGW com a biblioteca winpthreads instalada.

## Tentativa equivalente em Rust

O arquivo [`rust/data_race.rs`](./rust/data_race.rs) utiliza threads com escopo
para tentar emprestar `contador` de forma mutável a duas closures. Os tempos de
vida das duas threads podem se sobrepor, portanto ambas exigiriam acesso
exclusivo ao mesmo valor ao mesmo tempo:

```bash
rustc rust/data_race.rs
```

O compilador rejeita o programa porque as closures exigem acessos mutáveis
simultâneos. Diferentemente de um teste dinâmico, essa verificação não depende
de uma disputa ocorrer durante uma execução específica.

## Versão corrigida em Rust

Em [`rust/fixed.rs`](./rust/fixed.rs), `Arc` mantém propriedade compartilhada do
contador entre as threads, e `Mutex` permite acesso mutável exclusivo. Cada
thread recebe um clone do `Arc`, não uma cópia do contador.

```bash
rustc rust/fixed.rs -o fixed_rust
./fixed_rust
```

Saída esperada:

```text
Contador: 2000000
```

O método `lock` devolve um guarda. Enquanto esse guarda existir, outra thread
não pode entrar na região protegida. Ao sair de escopo, o guarda libera o mutex
automaticamente.

## Send e Sync

As traits marcadoras `Send` e `Sync` participam das garantias de concorrência:

- um tipo `Send` pode ter sua posse transferida para outra thread;
- um tipo `Sync` pode ser referenciado por múltiplas threads com segurança.

O compilador deriva essas propriedades quando os componentes do tipo permitem.
Tipos que não são seguros para determinado compartilhamento não atravessam a
fronteira entre threads em safe Rust.

## O que o exemplo demonstra

- C permite expressar o acesso concorrente sem sincronização;
- a ausência de um resultado errado visível não prova que o programa C é
  correto;
- safe Rust impede referências mutáveis simultâneas ao mesmo valor;
- `Arc` resolve a propriedade compartilhada, enquanto `Mutex` controla a
  mutação;
- sincronização continua tendo custo e pode causar problemas lógicos, como
  deadlocks, mesmo quando o acesso à memória é seguro.
