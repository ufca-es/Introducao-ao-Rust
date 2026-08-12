<h1 align="center">Projeto Rust for Linux</h1>

<p align="center">
  <strong>Uma alternativa segura e moderna para a criação de módulos e drivers</strong>
</p>

---

## Visão geral

| Iniciativa | Adoção | Suporte |
|:---:|:---:|:---:|
| Criada em **2020** | Rust adotado permanentemente no kernel em **2025** | Aceito a partir do **kernel 6.1** |

---

> ## “O Rust ainda não é o queridinho do kernel”

Quem estuda ou atua na área de desenvolvimento por tempo suficiente sabe que o *moderno nem sempre é bem recebido pelos desenvolvedores*. No caso da implementação de Rust no kernel Linux, não foi muito diferente.

Quem defende a implementação da linguagem Rust no kernel Linux utiliza o argumento de que a linguagem oferece segurança e modernidade ao kernel.

Já a *“velha guarda”*, que atuou por mais de 20 anos no desenvolvimento do kernel Linux utilizando C, via dois problemas principais na implementação:

### 1. Aprender uma nova linguagem

> Mesmo considerando os benefícios da linguagem Rust, **por que deveriam aprender uma nova linguagem simplesmente por causa de um “movimento” para uma linguagem mais moderna?**

### 2. Integrar diferentes linguagens

> Talvez mais relevante para a resistência dos desenvolvedores, levando em consideração os conceitos da Engenharia de Software, **era problemático pensar em transformar o kernel em um projeto no qual diferentes partes estivessem interligadas por meio de várias linguagens.**

---

## A resistência de Christoph Hellwig

Para exemplificar a resistência dos desenvolvedores à linguagem, podemos citar o caso de **Christoph Hellwig**, um veterano mantenedor da infraestrutura DMA (*Direct Memory Access*).

Hellwig era contrário à integração. Em sua visão, sistemas assim ficariam difíceis de manter, especialmente em seu caso com o DMA.

### Comunicação entre o DMA e um driver em C

```text
    DMA
     │
     │ C
     ▼
 Driver C
```

Resumindo, o responsável pelo DMA pode modificar uma API e, sendo tudo em C, trabalhar dentro de um ecossistema que conhece muito bem.

### Comunicação entre o DMA e um driver em Rust

Já com Rust, ficaria assim:

```text
             DMA em C
            /        \
           /          \
    Driver C       Binding Rust
                        │
                        ▼
                   Driver Rust
```

---

## E o que seria um *binding* em Rust?

Um *binding* em Rust é, de forma simples, uma ponte que permite que um código Rust use funções ou estruturas escritas em outra linguagem, como C. Por exemplo, se em C foi implementada a seguinte função:

### Exemplo em C

```c
int somar(int a, int b);
```

### Representação em Rust

Em Rust, isso seria representado como:

```rust
extern "C" {
    fn somar(a: i32, b: i32) -> i32;
}
```

---

## O problema da manutenção

Por isso, caso um mantenedor C alterasse uma API, o *binding* Rust poderia quebrar. Aqui morava o problema:

> ### Se quebrar, quem será responsável por corrigir o Rust?

Hellwig não queria que manter uma API C significasse:

> *“entender todas as abstrações Rust que dependem dela”.*

Esse problema, inclusive, foi reconhecido até mesmo pelos defensores de Rust.

[Para aprofundar a discussão](https://arstechnica.com/gadgets/2025/02/linux-leaders-pave-a-path-for-rust-in-kernel-while-supporting-c-veterans/ "Rust no kernel enquanto há suporte a C")

---

## Solução proposta por Torvalds

### O mantenedor C não é responsável pelo Rust

A posição de Linus é a de que um desenvolvedor como Hellwig **não precisa aprender Rust**.

Ele pode continuar cuidando apenas do DMA em C. Se uma alteração nessa API quebrar o *binding* Rust, a correção cabe aos mantenedores Rust:

```text
API C é alterada
        ↓
Binding Rust quebra
        ↓
Equipe Rust corrige
```

> **Não é responsabilidade de Hellwig consertar o Rust.**

Torvalds descreveu isso como uma espécie de **“wall of protection”**, ou parede de proteção, para os desenvolvedores C: ninguém seria obrigado a lidar com Rust só porque um código Rust utiliza suas interfaces.

Até aqui, provavelmente Hellwig e Linus poderiam concordar. O conflito estava no outro lado da regra.

---

### Hellwig queria impedir que o Rust utilizasse sua API

O *patch* que provocou a discussão **não modificava o código DMA** mantido por Hellwig. Ele apenas criava, em outro subdiretório, uma abstração Rust que utilizava suas interfaces.

> **Esse detalhe é fundamental.**

```text
kernel/
├── dma/
│   └── código C existente
│
└── rust/
    └── abstração que usa DMA
```

Por isso, Linus escreveu, em maiúsculas, que a solicitação **“DID NOT TOUCH THE DMA LAYER AT ALL”** — não tocava na camada DMA. Segundo ele, o código era apenas **mais um usuário** das interfaces DMA, localizado em outro subdiretório.

Essa diferença explica praticamente toda a briga.

---

### A “parede” funciona para os dois lados

Essa “parede” protege o desenvolvedor C: ele não precisa aprender, revisar ou corrigir Rust e pode continuar modificando sua API. Entretanto, essa proteção tem uma consequência:

```text
“Eu não quero responsabilidade sobre Rust”
                 ↓
“Então você também não controla o Rust”
```

Torvalds descreveu exatamente esse princípio: se o mantenedor optar por ignorar o lado Rust, então ele também não terá autoridade sobre o lado Rust.

| Equipe C | Equipe Rust |
|:---|:---|
| Mantém o DMA em C | Mantém as abstrações Rust |
| Modifica e corrige as APIs C | Acompanha as mudanças em C |
| Não precisa corrigir Rust | Corrige os *bindings* Rust |

Isso é bem diferente de obrigar o mantenedor C a trabalhar com Rust.

---

## Referências

- [Documentação oficial do Rust](https://docs.kernel.org/rust/index.html "Documentação do Rust")
- [Torvalds: código Rust no kernel não é imposto apesar das objeções dos mantenedores — Slashdot Linux](https://linux.slashdot.org/story/25/02/22/0524210/torvalds-rust-kernel-code-isnt-forced-in-over-maintainers-objections "Torvalds: Rust Kernel Code Isn't Forced In Over Maintainers' Objections")
- [Lideranças do Linux apoiam a adoção de Rust no código do kernel — The Register](https://www.theregister.com/software/2025/02/21/linux-royalty-backs-adoption-of-rust-for-kernel-code/1359453 "Linux royalty backs adoption of Rust for kernel code")
- [Uma mudança na manutenção da camada de mapeamento DMA do kernel — LWN.net](https://lwn.net/Articles/1011819/ "A change in maintenance for the kernel's DMA-mapping layer")
