# Entendendo Ownership
## Heap versus Stack
- Processos podem ter ==dois segmentos de expansão==:
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

### Notebooks
- Verifique mais exemplos em:
  - `strings` — [Literal (&str) vs. String no heap](./exemplos/strings/src/bin/exemplo1_literal_vs_string.rs), [String é crescível no heap](./exemplos/strings/src/bin/exemplo2_string_e_crescivel_no_heap.rs)
  - `move` — [Tipos Copy não movem (dado stack-only)](./exemplos/move/src/bin/exemplo3_tipos_copy_nao_movem.rs)

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

### Notebooks
- Verifique mais exemplos em:
  - `escopo_e_drop` — [Escopo de bloco](./exemplos/escopo_e_drop/src/bin/exemplo1_escopo_de_bloco.rs), [Drop automático no heap](./exemplos/escopo_e_drop/src/bin/exemplo2_drop_automatico_no_heap.rs), [Ordem de drop é LIFO](./exemplos/escopo_e_drop/src/bin/exemplo3_ordem_de_drop_e_lifo.rs)

- - - 

## Ownership
- Em Rust, a memória é gerenciada por meio de um sistema de *ownership*: um *conjunto de regras* que são verificadas em **tempo de compilação**.
- O principal objetivo devesse mecanismo é gerenciar os dados que estão armazenados na *Heap* do processo.
- **Regras de Ownership**:
	- ==Cada valor em Rust tem um owner==.
	- ==Só é possível ter um owner por vez==.
	- ==Quando o owner sai do escopo, o valor é excluído==.
- São regras relativamente simples, mas que modificam profundamente o modo ao qual utiliza-se a linguagem. Essas regras também ditam o comportamento de conceitos que vamos discutir a seguir.

### Notebooks
- Verifique mais exemplos em:
  - `move` — [Move invalida a variável original](./exemplos/move/src/bin/exemplo1_move_invalida_a_variavel_original.rs), [Reatribuição dispara drop imediato](./exemplos/move/src/bin/exemplo2_reatribuicao_dispara_drop_imediato.rs)
  - `strings` — [Clone é uma cópia profunda explícita](./exemplos/strings/src/bin/exemplo3_clone_e_copia_profunda_explicita.rs)
  - `ownership_em_funcoes` — [Passar por valor move a posse](./exemplos/ownership_em_funcoes/src/bin/exemplo1_passar_por_valor_move_a_posse.rs), [Tipos Copy não movem para a função](./exemplos/ownership_em_funcoes/src/bin/exemplo2_tipos_copy_nao_movem_para_a_funcao.rs)
  - `retorno_e_escopo` — [Devolve posse de um valor criado dentro](./exemplos/retorno_e_escopo/src/bin/exemplo1_devolve_posse_criada_dentro.rs), [Recebe e devolve a posse](./exemplos/retorno_e_escopo/src/bin/exemplo2_recebe_e_devolve_a_posse.rs), [Tupla para usar e devolver a posse](./exemplos/retorno_e_escopo/src/bin/exemplo3_tupla_para_usar_e_devolver.rs)

- - -

## Borrowing (Empréstimos)
- O mecanismo de passar um valor para uma função é semelhante ao de atribuir um valor a uma variável, ou seja, a *posse* daquele endereço/valor é transferida. Para resolver esse problema, Rust permite o uso de *referências*. Uma referência (`&`) é um tipo de dado que aponta para um valor armazenado na *Heap* (ponteiro). Veja o exemplo abaixo:
```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // `empresta` uma referência para calculate_length
    println!("The length of '{s1}' is {len}.");
}

// recebe uma referência para calculate_length
fn calculate_length(s: &String) -> usize { 
    s.len()
} // quando a referência sai do escopo, nada é descartado, porque ela não é dona do dado.
```
- **Regras de Borrowing**:
  - ==Referências imutáveis==: múltiplas partes do código podem ler o mesmo dado simultaneamente, sem problema, porque nenhuma delas pode modificá-lo. Por padrão, assim como as variáveis, as referências são imutáveis.
  - ==Referências mutáveis==: enquanto existir uma referência mutável ativa, nenhuma outra referência (nem imutável, nem mutável) pode existir ao mesmo tempo para o mesmo dado. Para criar uma referência mutável, use a palavra-chave `mut`.
  - ==Referências Pendentes (conceito intuitivo)==: uma referência nunca pode viver mais tempo que o dado ao qual ela aponta (não pode haver dangling references).

### Notebooks
- Verifique mais exemplos em:
  - `referencias_e_borrowing` — [Múltiplas referências imutáveis convivem](./exemplos/referencias_e_borrowing/src/bin/exemplo2_multiplas_referencias_imutaveis_convivem.rs)
  - `referencias_mutaveis` — [Referência mutável básica](./exemplos/referencias_mutaveis/src/bin/exemplo1_referencia_mutavel_basica.rs), [Apenas uma mutável por vez](./exemplos/referencias_mutaveis/src/bin/exemplo2_apenas_uma_mutavel_por_vez.rs), [Não misturar mutável com imutável](./exemplos/referencias_mutaveis/src/bin/exemplo3_nao_mistura_mutavel_com_imutavel.rs), [NLL: escopo termina no último uso](./exemplos/referencias_mutaveis/src/bin/exemplo4_nll_escopo_termina_no_ultimo_uso.rs)
  - `referencias_pendentes` — [Solução: devolver a posse](./exemplos/referencias_pendentes/src/bin/exemplo1_solucao_devolver_a_posse.rs), [Referência pendente não compila](./exemplos/referencias_pendentes/src/bin/exemplo2_referencia_pendente_nao_compila.rs)

- - - 

## Referências Bibliográficas
- [What is Ownership? — The Rust Programming Language](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
