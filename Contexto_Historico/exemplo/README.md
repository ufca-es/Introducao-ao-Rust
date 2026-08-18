# Exemplo Prático: Segurança de Memória em Tempo de Compilação

> Este exemplo complementa o documento [`contexto-historico.md`](../contexto-historico.md), ilustrando na prática o problema histórico que motivou a criação do Rust: bugs de gerenciamento manual de memória em linguagens como C.

## O que este exemplo demonstra

Três programas equivalentes:

| Pasta | O que faz | Resultado esperado |
|---|---|---|
| [`c/`](./c) | Versão em C com um bug clássico de *use-after-free* | Compila (com aviso), mas **imprime lixo de memória** em vez do texto correto |
| [`rust/ownership_ok/`](./rust/ownership_ok) | O mesmo cenário em Rust, escrito de forma correta | Compila e **imprime o texto correto**, sem `free()` manual e sem garbage collector |
| [`rust/borrow_error/`](./rust/borrow_error) | Uma versão em Rust que tenta reproduzir o mesmo tipo de bug do C | **Não compila** - o *borrow checker* recusa o programa antes mesmo de gerar um binário |

## Pré-requisitos

- **GCC** (ou outro compilador C compatível), para o exemplo em C.
- **Rust e Cargo**, para os exemplos em Rust.

Se você ainda não tem o Rust instalado, a forma recomendada é via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Em distribuições baseadas em Debian/Ubuntu também é possível instalar via `apt`:

```bash
sudo apt-get update
sudo apt-get install -y gcc rustc cargo
```

Verifique as instalações com:

```bash
gcc --version
rustc --version
cargo --version
```

## 1. Rodando o exemplo em C (com o bug)

```bash
cd c
gcc -Wall -Wextra use_after_free.c -o use_after_free
./use_after_free
```

**O que observar:**

- O `gcc` compila o programa, mas emite um aviso (`warning: pointer 'buffer' used after 'free'`) - ou seja, **o compilador percebe que algo está errado, mas não impede a geração do binário**.
- Ao rodar `./use_after_free`, a saída **não é** `Olá, Rust!`. Em nosso teste, o programa imprimiu bytes de lixo de memória (algo como caracteres de controle ilegíveis), pois a memória do `buffer` já havia sido liberada com `free()` antes de ser usada em `printf`. O resultado exato pode variar entre execuções e sistemas, o que é justamente o problema: **é um comportamento indefinido**.

## 2. Rodando o exemplo em Rust correto (ownership)

```bash
cd rust/ownership_ok
cargo run
```

**O que observar:**

- O programa compila sem nenhum aviso relacionado a memória.
- A saída é exatamente `Olá, Rust!`, de forma consistente e previsível.
- Não há `free()` manual em nenhum lugar do código: o compilador sabe, através do sistema de **ownership (posse)**, que a posse da `String` foi transferida da função `criar_saudacao` para a variável `msg` em `main`, e libera a memória automaticamente apenas quando `msg` sai de escopo, ao final do programa.

## 3. Rodando o exemplo em Rust com erro proposital (borrow checker)

```bash
cd rust/borrow_error
cargo build
```

**O que observar:**

- A compilação **falha** com o erro `E0597: 'x' does not live long enough`.
- O compilador está impedindo, em tempo de compilação, a criação de uma referência (`&x`) que ficaria "pendurada" (*dangling*) depois que `x` sai de escopo - exatamente o tipo de erro que, em C, gerou o `use-after-free` do exemplo 1.
- Para ver a explicação detalhada do erro, rode:

  ```bash
  rustc --explain E0597
  ```

## Resumo comparativo

| Aspecto | C (`use_after_free.c`) | Rust (`borrow_error`) |
|---|---|---|
| Quando o bug é detectado | Em tempo de execução (nem sempre de forma óbvia) | Em **tempo de compilação** |
| Ferramenta extra necessária | Valgrind, AddressSanitizer, ou sorte | Nenhuma - é o próprio compilador (`rustc`) |
| Resultado ao rodar com o bug | Saída incorreta / comportamento indefinido | O binário **nem chega a ser gerado** |
| Garbage collector envolvido | Não | Não |

## Estrutura de pastas

```
exemplo/
├── README.md
├── c/
│   └── use_after_free.c             # exemplo em C com bug de memória
└── rust/
    ├── ownership_ok/                # projeto Cargo - versão correta
    │   ├── Cargo.toml
    │   └── src/main.rs
    └── borrow_error/                # projeto Cargo - versão que não compila (proposital)
        ├── Cargo.toml
        └── src/main.rs
```

## Conclusão

Esse comparativo resume, na prática, o argumento histórico apresentado em [`contexto-historico.md`](../contexto-historico.md): ao mover a verificação de segurança de memória do *runtime* (como fazem linguagens com garbage collector) para o *compile time* (via ownership, borrowing e lifetimes), o Rust elimina classes inteiras de bugs que historicamente causaram falhas e vulnerabilidades graves em C e C++, sem pagar o preço de desempenho de um garbage collector.