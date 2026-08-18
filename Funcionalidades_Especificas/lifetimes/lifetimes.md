# Lifetimes

* Lifetime é a região do código durante a qual uma referência é válida —
  uma ferramenta do *borrow checker*, usada apenas em **tempo de
  compilação**, sem nenhum custo em runtime.
* O objetivo desse mecanismo é garantir que nenhuma referência sobrevive
  ao dado que ela aponta (o problema do *dangling reference*).
* Definições importantes:
  * *Na maioria dos casos, o compilador infere o lifetime sozinho
    (lifetime elision).*
  * *Anotação explícita (`'a`) só é obrigatória quando o compilador não
    consegue decidir sozinho — tipicamente com mais de uma referência
    de entrada e uma de saída.*
  * *A anotação não muda o tempo de vida de nada; ela só descreve, pro
    compilador, uma relação que já existe.*
  * *`'static` é o lifetime mais longo possível (dura o programa
    inteiro) — não é um "desliga a verificação".*
* São regras que existem só na compilação, mas que garantem segurança de
  memória em runtime sem depender de garbage collector — continuação
  natural do que Ownership e Borrowing já garantem.

## Notebooks

Verifique mais exemplos em:

* `anotacao_explicita` — [Sem anotação, o compilador rejeita](./exemplos/anotacao_explicita/src/bin/exemplo1_sem_anotacao.rs), [Anotação amarra o retorno ao menor lifetime de entrada](./exemplos/anotacao_explicita/src/bin/exemplo2_com_anotacao.rs), [Uma entrada só: elision resolve sozinha](./exemplos/anotacao_explicita/src/bin/exemplo3_uma_entrada_elision.rs)
* `structs_com_referencia` — [Struct válida enquanto o dado referenciado existe](./exemplos/structs_com_referencia/src/bin/exemplo1_struct_valida.rs), [Struct usada após o dado sair de escopo não compila](./exemplos/structs_com_referencia/src/bin/exemplo2_struct_invalida.rs)
* `lifetime_static` — [String literal é `'static` de verdade](./exemplos/lifetime_static/src/bin/exemplo1_string_literal.rs), [Forçar `'static` num dado que não é permanente não compila](./exemplos/lifetime_static/src/bin/exemplo2_static_incorreto.rs)

## Referências Bibliográficas

Validating References with Lifetimes — The Rust Programming Language
