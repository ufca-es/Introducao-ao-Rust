# Referência pendurada: C x Rust

Uma referência fica pendurada quando continua apontando para um objeto que já
deixou de existir. Acessá-la não é válido, mesmo que o endereço ainda contenha
aparentemente o valor antigo.

## Versão incorreta em C

O arquivo [`c/dangling_pointer.c`](./c/dangling_pointer.c) devolve o endereço de
uma variável automática local. Essa variável deixa de existir quando a função
retorna. O ponteiro recebido por `main` não aponta mais para um objeto válido e
sua desreferenciação produz comportamento indefinido.

Compile com avisos habilitados:

```bash
gcc -std=c11 -Wall -Wextra -Wpedantic c/dangling_pointer.c -o dangling_pointer
```

Com GCC 15.2.0, o compilador aceita gerar o executável, mas apresenta o aviso:

```text
warning: function returns address of local variable [-Wreturn-local-addr]
```

Um aviso não torna o programa válido. Também não é seguro usar uma execução
aparentemente bem-sucedida como evidência de correção, pois o padrão de C não
determina o resultado desse acesso.

## Versão corrigida em C

O arquivo [`c/fixed.c`](./c/fixed.c) reserva o inteiro na memória dinâmica e
transfere ao chamador o ponteiro para a alocação. O chamador verifica se
`malloc` teve sucesso, utiliza o valor e chama `free` exatamente uma vez.

```bash
gcc -std=c11 -Wall -Wextra -Wpedantic c/fixed.c -o fixed_c
./fixed_c
```

Saída esperada:

```text
Valor: 42
```

## Tentativa equivalente em Rust

O arquivo [`rust/dangling_reference.rs`](./rust/dangling_reference.rs) tenta
devolver uma referência para uma variável local. O borrow checker identifica
que `valor` será destruído ao final da função e rejeita o programa:

```bash
rustc rust/dangling_reference.rs
```

O diagnóstico esperado é o erro `E0515`, que informa que a função está
devolvendo uma referência para dados pertencentes à própria função.

## Versão corrigida em Rust

Em [`rust/fixed.rs`](./rust/fixed.rs), a função devolve um `Box<i32>`. A posse da
alocação é transferida ao chamador, em vez de devolver uma referência para um
valor que será destruído. Quando `valor` sai de escopo em `main`, a memória é
liberada automaticamente.

```bash
rustc rust/fixed.rs -o fixed_rust
./fixed_rust
```

Saída esperada:

```text
Valor: 42
```

## O que o exemplo demonstra

- C permite gerar o programa, embora um compilador com avisos habilitados possa
  apontar o problema;
- o acesso pelo ponteiro pendurado possui comportamento indefinido;
- safe Rust trata o mesmo problema como erro de compilação;
- as versões corrigidas tornam explícito quem possui o recurso;
- em C, o chamador precisa executar `free`; em Rust, `Box` é destruído quando
  seu dono sai de escopo.
