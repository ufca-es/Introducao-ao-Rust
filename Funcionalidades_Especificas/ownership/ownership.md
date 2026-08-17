# Entendendo Ownership
## Heap versus Stack
- Processos podem ter dois ==segmentos de expansão==:
	- *Heap*: uma área temporária para variáveis que são dinamicamente alocadas e liberadas.
		- Dados com *tamanho desconhecido ou flexível* devem ser armazenados na heap.
		- [[pointers]], [[file pointers]]
		- [[Valgrind]]
	- *Stack*: um área para variáveis locais normais e endereços de retorno.
		- Todos os dados armazenados na stack devem ter um *tamanho conhecido e fixo*.
		- [[stack|LIFO (Last-in First-out)]]
		- [[Stack Overflow]]
		- [[Call Stacks]]

![process-memory-organization](../../imgs/process-memory-organization.png)


- - -
## Escopo de uma Variável
- refere-se ao período ao qual uma variável é válida dentro de um programa, ela permanece válida enquanto o "ponteiro de execução do programa" não sair desse escopo.
```rust
fn main() {
    {                      // s is not valid here, since it's not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
}
```

- - - 

## Ownership
- Em Rust, a memória é gerenciada por meio de um sistema de *ownership*: um *conjunto de regras* que são verificadas em **tempo de compilação**.
- O principal objetivo devesse mecanismo é gerenciar os dados que estão armazenados na *Heap* do processo.
- Definições importantes:
	- *Cada valor em Rust tem um owner.*.
	- *Só é possível ter um owner por vez*.
	- *Quando o owner sai do escopo, o valor é excluído*.
- São regras relativamente simples, mas que modificam profundamente o modo ao qual utiliza-se a linguagem.

- - -

## Referência
- [What is Ownership? — The Rust Programming Language](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
