/* Comparação com "Mutable References" — cap. 4.2 do Rust Book:
 * https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
 *
 * Compilar e rodar: gcc -Wall -Wextra comparacao.c -o comparacao && ./comparacao
 * AVISO: a segunda parte deste exemplo lê de propósito um ponteiro
 * potencialmente invalidado por `realloc` -- é o ponto central da
 * comparação (comportamento indefinido em C x erro de compilação em
 * Rust), não um bug.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void muda(char *algo, size_t capacidade) {
    strncat(algo, ", world", capacidade - strlen(algo) - 1);
}

int main(void) {
    /* Seção 1 (equivalente ao exemplo 1 em Rust): */
    size_t capacidade = 32;
    char *s = malloc(capacidade);
    strcpy(s, "hello");

    muda(s, capacidade); /* equivalente ao `&mut s` do Rust: a função
                           * recebe o ponteiro e modifica o buffer */
    printf("%s\n", s);

    /* Seções 2 e 3 (equivalente aos exemplos 2 e 3 em Rust -- a regra de
     * só uma referência mutável por vez): em C, nada impede ter dois
     * ponteiros "mutáveis" para o MESMO buffer
     * ao mesmo tempo -- o compilador não distingue "só leitura" de
     * "leitura e escrita" além da convenção `const`. Isso abre espaço
     * para bugs de aliasing que o Rust rejeita na compilação (regra do
     * `referencias_mutaveis.rs`: só uma referência mutável por vez). */
    char *r1 = s;
    char *r2 = s;

    /* Se uma das duas "referências" realocar o buffer (por exemplo, para
     * caber uma string maior), a OUTRA passa a apontar para memória já
     * liberada -- sem nenhum aviso do compilador. */
    r1 = realloc(r1, capacidade * 2);
    strcat(r1, "!");

    printf("r1 = %s\n", r1); /* válido: r1 foi atualizado pelo realloc */
    printf("r2 = %s\n", r2); /* comportamento indefinido: se realloc moveu
                               * o bloco, r2 aponta para memória já
                               * liberada -- um use-after-free silencioso.
                               * Em Rust, ter r1 e r2 como `&mut` ao mesmo
                               * tempo nem compilaria (erro E0499). */

    free(r1);
    return 0;
}
