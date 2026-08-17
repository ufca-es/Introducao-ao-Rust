# Rust x C: segurança de memória na prática

Material de apoio para o subtema **"Comparação prática entre Rust e C"** do
trabalho *Introdução ao Rust - Segurança de Memória sem Garbage Collector*, da
disciplina de Paradigmas de Programação da UFCA.

Esta seção compara as duas linguagens com foco em programação de sistemas,
gerenciamento de memória e erros que o compilador do Rust consegue impedir em
tempo de compilação. Os exemplos são apresentados em pares equivalentes e têm
instruções para compilação e execução.

Os códigos foram verificados no Windows com GCC 15.2.0 e Rust 1.97.1, usando o
toolchain `stable-x86_64-pc-windows-gnu`. As versões corretas foram compiladas e
executadas. As versões Rust propositalmente inválidas tiveram seus diagnósticos
registrados, enquanto os programas C com comportamento indefinido não foram
executados.

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

## Vantagens e limitações

### Pontos fortes do C

C continua relevante em programação de sistemas por características que não
devem ser ignoradas na comparação:

- possui uma especificação relativamente pequena e implementações disponíveis
  para uma grande variedade de processadores e sistemas operacionais;
- tem uma ABI amplamente utilizada como fronteira de interoperabilidade entre
  linguagens e bibliotecas;
- permite controle direto sobre representação de dados, memória e dispositivos;
- conta com décadas de bibliotecas, ferramentas, documentação e código legado;
- pode ser a única opção viável em plataformas antigas ou muito restritas.

Uma linguagem menor não significa que seja simples produzir software seguro em
C. O programador precisa controlar manualmente tempos de vida, aliases,
liberação de recursos e sincronização. Revisões, testes e analisadores ajudam,
mas não transformam essas propriedades em garantias gerais da linguagem.

### Pontos fortes do Rust

Rust torna parte dessas obrigações verificável pelo compilador:

- ownership representa quem deve liberar cada recurso;
- borrowing permite acesso temporário sem transferir a posse;
- lifetimes impedem que referências seguras sobrevivam aos valores referidos;
- tipos como `Option` e `Result` representam ausência e falha explicitamente;
- `Send` e `Sync` restringem como tipos atravessam fronteiras entre threads;
- Cargo padroniza compilação, testes, documentação e dependências.

Essas garantias não implicam que todo programa Rust esteja correto. O sistema de
tipos não impede erros de lógica, deadlocks, consumo excessivo de memória ou
algoritmos incorretos. Código `unsafe`, interfaces externas e implementações de
baixo nível também exigem invariantes que o compilador não consegue verificar
sozinho.

### Custos da abordagem de Rust

O rigor adicional possui custos de adoção:

- ownership, borrowing e lifetimes aumentam a curva de aprendizagem;
- alguns padrões de estruturas de dados exigem reformulação ou abstrações mais
  complexas;
- mensagens do borrow checker podem exigir conhecimento do modelo de posse;
- compilações e tipos genéricos podem aumentar o tempo de build e o tamanho dos
  artefatos;
- bibliotecas ou plataformas específicas podem oferecer suporte mais maduro em
  C.

O custo ocorre principalmente durante projeto e compilação. As verificações de
empréstimos não exigem um Garbage Collector nem uma tabela de referências
consultada a cada acesso durante a execução.

## Cenários de uso

| Cenário | Opção que tende a favorecer | Motivo principal |
|---|---|---|
| Novo componente de sistema exposto a entradas não confiáveis | Rust | Redução de vulnerabilidades causadas por uso inválido de memória |
| Firmware para plataforma sem suporte maduro a Rust | C | Toolchain, bibliotecas e documentação já disponíveis |
| Integração com biblioteca ou sistema legado em C | C ou adoção gradual de Rust | Reescrita completa pode ter custo e risco elevados |
| Serviço concorrente com requisitos de segurança de memória | Rust | Ownership e traits de concorrência eliminam várias condições inseguras |
| Código extremamente dependente de uma ABI estável e difundida | C | ABI consolidada entre compiladores, sistemas e linguagens |
| Biblioteca nova chamada por aplicações C | Rust com interface C | Implementação segura internamente e compatibilidade na fronteira |
| Prototipação de baixo nível por equipe experiente em C | Depende do contexto | Prazo, risco, plataforma e experiência podem pesar mais que a linguagem |

A escolha não deve ser apresentada como uma disputa em que uma linguagem vence
em todos os critérios. Plataforma, equipe, bibliotecas existentes, requisitos
de certificação, interoperabilidade e custo de migração influenciam a decisão.

## Interoperabilidade e adoção gradual

Rust pode declarar funções e dados fornecidos por bibliotecas C por meio de FFI
(*Foreign Function Interface*). No sentido contrário, uma biblioteca Rust pode
expor funções com uma interface binária compatível com C. Isso permite substituir
ou acrescentar componentes gradualmente, sem reescrever todo um sistema.

A fronteira FFI exige cuidado. O compilador não conhece automaticamente os
contratos de ponteiros recebidos do C, como validade, alinhamento, tamanho e
tempo de vida. Por isso, chamadas externas normalmente envolvem `unsafe`. Uma
prática recomendada é manter essa camada pequena e convertê-la imediatamente em
tipos e operações seguros para o restante do programa.

FFI não é um exemplo obrigatório desta seção. Ele é apresentado como cenário de
uso porque demonstra que adotar Rust não exige abandonar imediatamente uma base
existente em C.

## Síntese da comparação

Rust e C podem produzir código nativo sem Garbage Collector e atender domínios
semelhantes. A diferença central está em onde parte da responsabilidade é
verificada. C fornece mecanismos diretos e permite que o programador estabeleça
os contratos necessários. Rust representa muitos desses contratos no sistema
de tipos e rejeita programas que não consegue provar seguros dentro de seu
subconjunto seguro.

Rust é especialmente atraente quando falhas de memória e concorrência têm alto
impacto. C permanece importante quando compatibilidade, disponibilidade de
toolchain, código legado ou restrições de plataforma são determinantes. Uma
análise responsável considera os requisitos concretos do projeto em vez de
afirmar que uma linguagem é universalmente superior.

## Estrutura dos exemplos

Os exemplos práticos estão organizados em três blocos independentes:

1. referência pendurada e *use-after-free*;
2. liberação dupla (*double free*);
3. condição de corrida (*data race*).

Cada bloco possui uma versão em C, uma tentativa equivalente em *safe Rust*, a
saída observada e uma versão corrigida quando necessário. Programas que falham
de propósito estão identificados claramente.

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

### Data race

O terceiro exemplo compartilha um contador entre duas threads. C permite o
acesso conflitante sem sincronização, o que produz comportamento indefinido. Em
safe Rust, o compartilhamento mutável é rejeitado, e a versão correta combina
`Arc` para propriedade compartilhada com `Mutex` para exclusão mútua.

- [Código, explicação e instruções de execução](./data-race/README.md)

## Referências bibliográficas

- [The Rust Programming Language: Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [The Rust Programming Language: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [The Rustonomicon: Meet Safe and Unsafe](https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html)
- [The Rustonomicon: FFI](https://doc.rust-lang.org/nomicon/ffi.html)
- [Rust Reference: Behavior considered undefined](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
- [Rust Standard Library: `Send`](https://doc.rust-lang.org/std/marker/trait.Send.html)
- [Rust Standard Library: `Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html)
- [Rust Standard Library: `Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)
- [Rust Standard Library: `Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
- [ISO/IEC 9899:201x Committee Draft N1570](https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf)
- [The Open Group Base Specifications: `pthread_create`](https://pubs.opengroup.org/onlinepubs/9799919799/functions/pthread_create.html)
- [The Open Group Base Specifications: `pthread_mutex_lock`](https://pubs.opengroup.org/onlinepubs/9799919799/functions/pthread_mutex_lock.html)
- [GCC: Options to Request or Suppress Warnings](https://gcc.gnu.org/onlinedocs/gcc/Warning-Options.html)
- [GCC: Program Instrumentation Options](https://gcc.gnu.org/onlinedocs/gcc/Instrumentation-Options.html)
