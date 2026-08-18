# Lifetimes em Rust

> Parte de: **Funcionalidades Específicas** (Alan, Abner)
> Autor desta seção: Abner

## 1. O problema que lifetimes resolvem

Ownership garante que cada valor tem um único dono e que o valor é liberado
quando o dono sai de escopo. Borrowing permite emprestar referências (`&T`,
`&mut T`) sem transferir a posse. O problema que falta resolver é: **como o
compilador sabe que uma referência emprestada continua válida no momento em
que ela é usada?**

Em C, nada impede isto:

```c
int *dangling_pointer(void) {
    int local = 42;
    return &local; // 'local' morre ao sair da função
} // ponteiro retornado aponta para memória já desalocada (stack)
```

O programa compila, e o comportamento é indefinido (undefined behavior) —
pode "funcionar" por acaso, ou corromper dados silenciosamente. Em Rust, o
mesmo padrão **não compila**. É esse mecanismo de verificação — os
**lifetimes** — que esta seção documenta.

## 2. O que é um lifetime, na prática

Um lifetime é a região do código-fonte durante a qual uma referência é
válida. Não é um conceito de runtime (não existe custo em tempo de execução);
é puramente uma ferramenta do **borrow checker**, usada em tempo de
compilação para provar que nenhuma referência sobrevive ao dado que ela
aponta.

Na maioria dos casos o compilador infere os lifetimes sozinho (chamado de
*lifetime elision*). Anotações explícitas (`'a`, `'b`, ...) só são
necessárias quando o compilador não consegue decidir sozinho qual é a
relação entre os lifetimes de entrada e saída de uma função — tipicamente
quando há mais de uma referência de entrada e uma referência de saída.

```rust
// Sem anotação, o compilador não sabe se o retorno "pertence"
// a `x` ou a `y` — é obrigatório anotar:
fn maior<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

A leitura correta de `fn maior<'a>(x: &'a str, y: &'a str) -> &'a str` é:
*"existe um lifetime `'a` tal que `x`, `y` e o valor de retorno vivem, no
mínimo, durante `'a`"*. A anotação **não muda** por quanto tempo os dados
vivem; ela apenas descreve uma relação que já existe, para que o borrow
checker consiga verificá-la.

## 3. Regras de elision (por que quase nunca precisamos anotar)

O compilador aplica três regras antes de exigir anotação manual:

1. Cada parâmetro de referência recebe seu próprio lifetime.
2. Se há exatamente **um** parâmetro de entrada por referência, esse
   lifetime é atribuído a todas as referências de saída.
3. Se um dos parâmetros é `&self` ou `&mut self` (método), o lifetime de
   `self` é atribuído a todas as referências de saída.

É por isso que `fn primeira_palavra(s: &str) -> &str` compila sem anotação
(regra 2), mas a função `maior` acima precisa de anotação — ela tem dois
parâmetros de referência e nenhuma regra decide sozinha qual dos dois está
ligado ao retorno.

## 4. Structs com referências

Structs também podem guardar referências, mas precisam declarar o lifetime
dessa referência — isso garante que uma instância da struct não pode
sobreviver ao dado que ela referencia:

```rust
struct Trecho<'a> {
    texto: &'a str,
}

fn main() {
    let romance = String::from("Era uma vez...");
    let primeiro = Trecho { texto: &romance[0..4] };
    println!("{}", primeiro.texto); // válido: `romance` ainda vive aqui
}
```

Se tentássemos usar `primeiro` depois que `romance` saísse de escopo, o
borrow checker rejeitaria a compilação — exatamente o mecanismo que evita o
`dangling reference` do exemplo em C.

## 5. `'static`: o caso especial

`'static` indica que a referência é válida durante toda a execução do
programa — por exemplo, string literals (`&'static str`), que são
embutidas no binário. Vale destacar na apresentação: `'static` **não** é
"desliga o borrow checker"; é apenas o lifetime mais longo possível, e o
compilador continua verificando normalmente.

## 6. O que o lifetime NÃO faz

Ponto importante para a análise crítica do grupo (seção de Neto): lifetimes
não têm nenhum custo em tempo de execução — são inteiramente apagados após
a compilação (*erased*, semelhante a type erasure de generics em outras
linguagens). Isso reforça o argumento central de Rust: segurança de memória
**sem** garbage collector e **sem** overhead de runtime, ao custo de uma
curva de aprendizado maior e de, ocasionalmente, o programador precisar
reestruturar código para satisfazer o borrow checker.

## 7. Ligação com os exemplos práticos

Os exemplos ficam em `exemplos/`, um por conceito, no mesmo padrão usado na
pasta `ownership/` do repositório:

- `exemplos/dangling_reference/` — o par comparativo Rust/C de **dangling
  reference** (item da lista de exemplos sugeridos no enunciado):
  `dangling.rs` (rejeitado em tempo de compilação, com a mensagem de erro
  do `rustc` documentada), `dangling_fixed.rs` (versão corrigida) e
  `dangling.c` (compila, mas é undefined behavior).
- `exemplos/anotacao_explicita/` — a função `maior()`, caso em que a
  elision não resolve sozinha e a anotação `'a` é obrigatória.
- `exemplos/structs_com_referencia/` — a struct `Trecho<'a>`, mostrando
  que uma struct não pode sobreviver ao dado que referencia.
- `exemplos/lifetime_static/` — contraste entre `&'static str` (string
  literal) e uma referência comum, para deixar claro que `'static` não
  desliga a verificação do borrow checker.

## Referências

- The Rust Programming Language, cap. 10.3 — *Validating References with
  Lifetimes* (documentação oficial, doc.rust-lang.org/book)
- Rust Reference — *Lifetime elision* (doc.rust-lang.org/reference)
- Jung, R. et al. *RustBelt: Securing the Foundations of the Rust
  Programming Language* (POPL 2018) — formalização do modelo de ownership/
  borrowing/lifetimes
