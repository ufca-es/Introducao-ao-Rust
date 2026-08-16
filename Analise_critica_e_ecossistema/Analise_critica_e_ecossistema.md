# Análise crítica e ecossistema do Rust

## Ecossistema

O Rust possui um ecossistema integrado em torno do **Cargo**, ferramenta usada
para criar projetos, instalar dependências, compilar, testar e gerar
documentação. As bibliotecas reutilizáveis são chamadas de *crates* e podem
ser encontradas no [crates.io](https://crates.io/), com documentação publicada
no [docs.rs](https://docs.rs/).

| Área | Crates | Uso |
|---|---|---|
| Dados | `serde`, `serde_json` | Trabalhar com JSON |
| Rede | `reqwest` | Consumir APIs HTTP |
| Assincronismo | `tokio` | Executar tarefas assíncronas |
| Back-end | `axum`, `actix-web` | Construir APIs web |
| Linha de comando | `clap` | Criar programas de terminal |

Os projetos em [exemplos](./exemplos/) demonstram o uso de `serde`, `clap` e
da biblioteca padrão para concorrência.

## Maturidade e adoção

Rust é utilizado em ferramentas de terminal, infraestrutura, serviços web,
sistemas embarcados e WebAssembly. Projetos como `ripgrep`, `fd`, `bat`,
`Deno` e `uv` mostram seu uso em software real. O projeto Rust for Linux
permite introduzir Rust gradualmente em drivers e módulos do kernel, sem exigir
a reescrita imediata do código C existente. Em 2025, os mantenedores do kernel
consideraram concluído o experimento com Rust, fortalecendo sua posição como
parte central do projeto. A adoção, porém, continua gradual: Rust convive com
C, e parte de sua infraestrutura ainda evolui para reduzir dependências de
recursos instáveis do compilador.

## Comunidade e perspectivas de adoção

A comunidade de Rust apresenta alto nível de satisfação com a linguagem. Na
pesquisa Stack Overflow de 2025, Rust foi a linguagem mais admirada, com 72%
dos respondentes que a utilizaram desejando continuar a usá-la. Esse dado não
mede diretamente a adoção empresarial, mas indica uma perspectiva positiva de
crescimento. Ao mesmo tempo, a presença de Rust em empresas ainda é menor que
a de linguagens tradicionais, o que evidencia custos reais de migração,
capacitação das equipes e maturidade desigual de bibliotecas em algumas áreas.

O próprio Cargo foi apontado pela mesma pesquisa como a ferramenta de
infraestrutura e desenvolvimento em nuvem mais admirada, com 71%. Isso reforça
que a adoção de Rust está relacionada não apenas à linguagem, mas também a uma
experiência de desenvolvimento integrada.

## Análise crítica

As principais vantagens são segurança de memória e concorrência verificadas
antes da execução, desempenho sem garbage collector e ferramentas padronizadas
para desenvolvimento. Isso torna Rust especialmente interessante em sistemas
nos quais falhas de memória, desempenho ou confiabilidade são pontos críticos.

Por outro lado, ownership, borrowing e lifetimes aumentam a curva de
aprendizagem. Projetos grandes podem ter compilação mais lenta, e algumas áreas
ainda possuem menos bibliotecas maduras que ecossistemas como Python, Java e
JavaScript. Além disso, o uso de `unsafe` e integrações com C exigem revisão
cuidadosa, pois parte das garantias automáticas deixa de se aplicar.

Assim, Rust é uma escolha forte quando segurança, desempenho e manutenção de
longo prazo justificam o investimento inicial de aprendizado.

## Referências

- [Ferramentas do Rust](https://www.rust-lang.org/tools). Acesso em: 16 ago. 2026.
- [Documentação do Cargo](https://doc.rust-lang.org/cargo/). Acesso em: 16 ago. 2026.
- [crates.io](https://crates.io/). Acesso em: 16 ago. 2026.
- [docs.rs](https://docs.rs/). Acesso em: 16 ago. 2026.
- [Pesquisa Stack Overflow Developer Survey 2025](https://survey.stackoverflow.co/2025/technology/). Acesso em: 16 ago. 2026.
- [Rust for Linux no Linux Plumbers Conference 2025](https://lpc.events/event/19/contributions/2068/attachments/1859/3981/2025-12-11%2520-%2520LPC%25202025%2520-%2520Rust%2520for%2520Linux.pdf). Acesso em: 16 ago. 2026.
- [Roadmap oficial do projeto Rust para Rust for Linux](https://rust-lang.github.io/rust-project-goals/2026/roadmap-rust-for-linux.html). Acesso em: 16 ago. 2026.
