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
* `'static` é o lifetime mais longo possível (dura o programa inteiro) —
  não é um "desliga a verificação".
* São regras que existem só na compilação, mas que garantem segurança de
  memória em runtime sem depender de garbage collector.

## Notebooks

* Verifique mais exemplos em:
  * `anotacao_explicita` — [Sem anotação, o compilador rejeita](./exemplos/anotacao_explicita/sem_anotacao.rs), [Anotação amarra o retorno ao menor lifetime de entrada](./exemplos/anotacao_explicita/com_anotacao.rs)
  * `structs_com_referencia` — [Struct não sobrevive ao dado que referencia](./exemplos/structs_com_referencia/trecho.rs)
  * `lifetime_static` — [`'static` é o lifetime mais longo, não desliga a verificação](./exemplos/lifetime_static/static_example.rs)

O exemplo canônico de **dangling reference**, que ilustra diretamente
esse mecanismo comparado com C, está em
[`Rust_x_C/dangling-reference/`](../../Rust_x_C/dangling-reference).
