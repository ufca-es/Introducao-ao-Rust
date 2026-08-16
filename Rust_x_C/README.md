# Rust x C: segurança de memória na prática

Material de apoio para o subtema **"Comparação prática entre Rust e C"** do
trabalho *Introdução ao Rust - Segurança de Memória sem Garbage Collector*, da
disciplina de Paradigmas de Programação da UFCA.

Esta seção compara as duas linguagens com foco em programação de sistemas,
gerenciamento de memória e erros que o compilador do Rust consegue impedir em
tempo de compilação. Os exemplos serão apresentados em pares equivalentes e
terão instruções para compilação e execução.

## Objetivo da comparação

C oferece controle direto sobre memória e hardware, tem ampla portabilidade e
uma base de código consolidada durante décadas. Esse controle também atribui ao
programador a responsabilidade de garantir que ponteiros sejam válidos, que
recursos sejam liberados uma única vez e que acessos concorrentes estejam
corretamente sincronizados.

Rust procura manter o controle e o desempenho esperados de uma linguagem de
sistemas, acrescentando verificações de segurança em tempo de compilação. Em
*safe Rust*, o modelo de *ownership*, as regras de empréstimo (*borrowing*) e os
tempos de vida (*lifetimes*) impedem várias classes de uso inválido da memória
antes que o programa seja executado.

> **Tese central:** Rust busca oferecer controle de baixo nível comparável ao C,
> transferindo parte da responsabilidade pela segurança de memória do
> programador para o compilador, sem depender de um Garbage Collector.

## Gerenciamento de memória

### C: responsabilidade manual

Em C, memória dinâmica normalmente é obtida por funções como `malloc` e
`calloc` e devolvida por `free`. A linguagem permite aritmética de ponteiros e
não acompanha automaticamente quem é responsável por cada alocação. Isso dá
flexibilidade ao programador, mas torna possível:

- acessar uma região depois de sua liberação (*use-after-free*);
- liberar a mesma região mais de uma vez (*double free*);
- manter um ponteiro para um objeto que já deixou de existir (*dangling
  pointer*);
- acessar posições fora dos limites de um objeto;
- compartilhar dados entre threads sem sincronização adequada.

Essas operações podem produzir **comportamento indefinido**. Isso não significa
apenas que o programa encerrará com erro: ele pode aparentar funcionar, produzir
resultados incorretos ou apresentar comportamentos diferentes conforme o
compilador, as otimizações e o ambiente de execução.

### Rust: ownership e empréstimos

Em Rust, cada valor possui um responsável chamado de dono (*owner*). Quando o
dono sai de escopo, seus recursos são liberados de maneira determinística. A
posse também pode ser transferida por uma operação de movimento (*move*), após
a qual o vínculo anterior não pode mais usar o valor.

Um valor pode ser acessado temporariamente por referências, seguindo duas regras
centrais:

- podem existir várias referências imutáveis para o mesmo valor;
- ou pode existir uma única referência mutável, mas não as duas formas ao mesmo
  tempo.

O *borrow checker* verifica essas regras e utiliza os *lifetimes* para garantir
que uma referência não sobreviva ao valor ao qual aponta. Grande parte dessas
verificações ocorre durante a compilação e não exige um coletor executado em
segundo plano.

## Comparação resumida

| Critério | C | Rust |
|---|---|---|
| Execução | Código nativo | Código nativo |
| Gerenciamento de memória | Manual, normalmente com `malloc` e `free` | Ownership e liberação por escopo |
| Coleta de lixo | Não possui | Não possui |
| Referências | Ponteiros sob responsabilidade do programador | Empréstimos verificados pelo compilador |
| Segurança de memória | Não é garantida pela linguagem | Garantida dentro das regras de *safe Rust* |
| Concorrência | Compartilhamento e sincronização manuais | Regras de ownership e traits `Send`/`Sync` |
| Tratamento de erros | Códigos de retorno, valores sentinela e `errno` | Tipos `Result` e `Option`, além de `panic!` |
| Ferramentas de projeto | Variam conforme compilador e sistema de build | Cargo integra build, dependências e testes |
| Compatibilidade | ABI consolidada e suporte muito amplo | Pode interoperar com C por FFI |

## Limites das garantias do Rust

As garantias descritas neste material se aplicam a *safe Rust*. A palavra-chave
`unsafe` habilita operações que o compilador não consegue validar completamente,
como desreferenciar ponteiros crus ou acessar determinados recursos externos.
Nesses trechos, cabe ao programador preservar manualmente os invariantes de
segurança exigidos pela linguagem.

Isso não torna `unsafe` equivalente a desativar todas as verificações do Rust:
as regras normais continuam valendo fora das operações explicitamente
permitidas. Entretanto, um erro dentro de uma abstração insegura pode afetar o
código seguro que depende dela, razão pela qual esses blocos devem ser pequenos,
justificados e revisados cuidadosamente.

Rust também não impede todos os vazamentos de memória. Por exemplo, ciclos
construídos com contagem de referências podem manter valores vivos
indefinidamente. Vazamentos são considerados seguros do ponto de vista de
acesso à memória, embora continuem sendo um problema de consumo de recursos.

## Exemplos planejados

Os exemplos práticos serão adicionados em blocos independentes:

1. referência pendurada e *use-after-free*;
2. liberação dupla (*double free*);
3. condição de corrida (*data race*).

Cada bloco terá uma versão em C, uma tentativa equivalente em *safe Rust*, a
saída observada e uma versão corrigida quando necessário. Programas que falham
de propósito serão identificados claramente.

## Exemplos disponíveis

### Referência pendurada e use-after-free

O primeiro exemplo mostra uma função que tenta devolver uma referência para um
valor local. Em C, o compilador pode emitir um aviso, mas ainda gerar um
executável cujo acesso ao ponteiro possui comportamento indefinido. Em safe
Rust, o borrow checker rejeita a tentativa antes da execução.

- [Código, explicação e instruções de execução](./dangling-reference/README.md)

### Double free

O segundo exemplo tenta liberar a mesma alocação duas vezes. Em C, isso produz
comportamento indefinido e pode ser identificado por avisos ou ferramentas
dinâmicas em alguns casos. Em safe Rust, a primeira chamada a `drop` consome o
valor, e uma segunda chamada é rejeitada como uso de valor movido.

- [Código, explicação e instruções de execução](./double-free/README.md)

## Referências iniciais

- [The Rust Programming Language: Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [The Rust Programming Language: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [The Rustonomicon: Meet Safe and Unsafe](https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html)
- [Rust Reference: Behavior considered undefined](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
