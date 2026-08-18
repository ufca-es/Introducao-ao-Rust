# Contexto Histórico do Rust

## 1. Introdução

Antes de falar sobre Rust em si, é preciso entender **os problemas que motivaram sua criação**. Rust não surgiu como um experimento acadêmico isolado, mas como uma resposta direta a décadas de bugs, falhas de segurança e acidentes de engenharia causados por linguagens de baixo nível como C e C++.

## 2. O problema: décadas de bugs de memória em C e C++

C e C++ dominam o desenvolvimento de sistemas operacionais, navegadores, bancos de dados e software embarcado há mais de 40 anos, principalmente por oferecerem controle total sobre a memória e desempenho próximo ao hardware. Esse controle, porém, tem um preço alto: **o gerenciamento manual de memória é uma das maiores fontes de bugs graves de software da história da computação.**

Os problemas mais recorrentes incluem:

- **Buffer overflow** - escrever além dos limites de um array/buffer, sobrescrevendo memória adjacente.
- **Use-after-free** - usar um ponteiro para memória que já foi liberada (`free`/`delete`).
- **Dangling pointers** - ponteiros que apontam para memória inválida ou já reaproveitada.
- **Double free** - liberar a mesma região de memória duas vezes, corrompendo o heap.
- **Data races** - duas threads acessando a mesma região de memória simultaneamente, ao menos uma delas escrevendo, sem sincronização adequada.
- **Null pointer dereference** - acessar um ponteiro nulo, causando falhas (segmentation fault).

Esses não são bugs raros ou teóricos. Estudos internos divulgados pela **Microsoft (2019)** e pelo time de segurança do **Google Chrome/Chromium (2020)** mostraram que aproximadamente **70% das vulnerabilidades críticas de segurança** corrigidas nesses projetos ao longo dos anos estavam relacionadas a erros de gerenciamento de memória - o tipo exato de erro que compiladores de C/C++ não conseguem detectar em tempo de compilação.

Isso gerou um dilema histórico na engenharia de software:

| Abordagem | Desempenho | Segurança de memória |
|---|---|---|
| C / C++ | Alto (controle manual) | Baixa (responsabilidade do programador) |
| Java, Python, Go, C# | Mais baixo (overhead de runtime) | Alta (garbage collector) |

Linguagens com **garbage collector (GC)** resolvem boa parte dos problemas de memória, mas introduzem pausas de coleta de lixo e overhead de runtime que as tornam inadequadas para sistemas operacionais, drivers, motores de jogos ou software embarcado com restrições de tempo real - justamente os domínios onde C e C++ continuavam (e continuam) insubstituíveis.

Rust nasceu para tentar **quebrar esse dilema**: oferecer segurança de memória no nível de linguagens com GC, mas com desempenho e controle no nível de C/C++, **sem usar garbage collector**.

## 3. As origens do Rust

- **2006** - O projeto é iniciado como iniciativa pessoal do engenheiro **Graydon Hoare**, então funcionário da Mozilla. A motivação inicial teria surgido, segundo relatos do próprio Hoare, após incidentes relacionados a falhas de software causadas por bugs de memória.
- **2009** - A **Mozilla Research** passa a patrocinar oficialmente o projeto, reconhecendo o potencial da linguagem para reescrever componentes críticos do motor de renderização do Firefox.
- **2010** - Rust é anunciado publicamente pela Mozilla.
- **2010–2011** - O compilador, originalmente escrito em OCaml, é reescrito na própria linguagem Rust (*self-hosting*), um marco importante de maturidade.
- **2012–2014** - A linguagem passa por diversas revisões significativas de sintaxe e do sistema de tipos enquanto a Mozilla desenvolve o **Servo**, um motor de navegador experimental escrito em Rust, usado como "prova de conceito" em escala real para os conceitos da linguagem (muitos componentes do Servo foram depois incorporados ao motor de renderização do Firefox, o Quantum).
- **15 de maio de 2015** - Lançamento da **versão 1.0**, com garantias formais de estabilidade e compatibilidade retroativa.
- **2015–2020** - Adoção crescente fora da Mozilla: Dropbox, npm, Cloudflare e outras empresas passam a usar Rust em componentes críticos de infraestrutura.
- **2020** - A Mozilla passa por uma grande reestruturação e demite parte significativa da equipe do time Rust; para garantir a continuidade do projeto de forma independente da empresa, é fundada a **Rust Foundation**, com apoio de empresas como AWS, Google, Microsoft e Huawei como membros fundadores.
- **2021 em diante** - Adoção em projetos de altíssima visibilidade: o **kernel do Linux** passa a aceitar código Rust (2022), a AWS usa Rust em serviços de infraestrutura crítica (como o hypervisor Firecracker), a Microsoft reescreve componentes do Windows em Rust, e a linguagem é eleita "linguagem mais amada" por 8 anos consecutivos na pesquisa *Stack Overflow Developer Survey*.

## 4. Objetivos de design que guiaram a criação da linguagem

Desde o início, o projeto foi guiado por três pilares que resumem bem sua proposta de valor:

1. **Segurança (safety)** - eliminar classes inteiras de bugs de memória e data races **em tempo de compilação**, antes mesmo do programa rodar.
2. **Concorrência (concurrency)** - permitir programação concorrente sem os riscos tradicionais de condições de corrida, através do próprio sistema de tipos.
3. **Desempenho (performance)** - sem máquina virtual e sem garbage collector, gerando código nativo comparável a C/C++.

O mecanismo central que a linguagem criou para atingir esses objetivos simultaneamente é o sistema de **ownership (posse), borrowing (empréstimo) e lifetimes (tempos de vida)**, verificado estaticamente pelo compilador através do chamado *borrow checker*. É esse mecanismo que permite ao Rust "saber", em tempo de compilação, quando cada trecho de memória pode ser liberado - dispensando tanto a coleta de lixo em tempo de execução quanto o gerenciamento manual propenso a erros.

## 5. Referências

- [Rust](https://www.dio.me/articles/rust-a57976da6612)
- [Rust vs C: Segurança e Performance](https://rustlang.com.br/artigos/rust-vs-c/)
- [Explorando a Linguagem de Programação Rust](https://skillstecnologicas.com/linguagem-de-programacao-rust/)
- [Rust: o que é, características e como instalar](https://www.alura.com.br/artigos/rust-o-que)
- [We need a safer systems programming language](https://www.microsoft.com/en-us/msrc/blog/2019/07/we-need-a-safer-systems-programming-language/)
- [Rust Foundation](https://foundation.rust-lang.org/)
- [Rust Blog](https://blog.rust-lang.org/)