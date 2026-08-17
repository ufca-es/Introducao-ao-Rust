/* Comparação com "Ways Variables and Data Interact: Move" — cap. 4.1 do
 * Rust Book:
 * https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
 *
 * Compilar e rodar: gcc -Wall -Wextra comparacao.c -o comparacao && ./comparacao
 * AVISO: este programa trava DE PROPÓSITO (double free) -- é o ponto
 * central da comparação: o que o Rust rejeita em tempo de COMPILAÇÃO, o C
 * deixa passar e só quebra em tempo de EXECUÇÃO.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    char *s1 = malloc(6);
    strcpy(s1, "hello");

    char *s2 = s1; /* em C, "atribuição" é SEMPRE uma cópia rasa: s1 e s2
                     * passam a apontar para o mesmo endereço no heap. O
                     * compilador não tem noção de posse -- para ele, os
                     * dois ponteiros são igualmente válidos ao mesmo
                     * tempo, sem nenhum aviso. */

    printf("s1 = %s\n", s1);
    printf("s2 = %s\n", s2); /* isso compila e roda normalmente em C; o
                               * equivalente em Rust (usar s1 depois do
                               * `let s2 = s1;`) nem compilaria. */

    free(s1);
    free(s2); /* double free: o mesmo endereço é liberado duas vezes.
               * Comportamento indefinido.
               *
               * Testado neste ambiente (glibc): o processo aborta com
               *   free(): double free detected in tcache 2
               * e código de saída 134 (SIGABRT).
               *
               * Nota sobre ferramentas: o GCC 15 tem um aviso estático
               * (-Wuse-after-free) capaz de flagrar ESTE caso trivial na
               * compilação. Mas basta mover os dois free() para dentro de
               * uma função auxiliar (algo tão comum quanto uma função
               * `liberar(char *p) { free(p); }`) para o aviso desaparecer
               * por completo -- testado, sem nenhum aviso na compilação.
               * É uma heurística local, não uma garantia. A checagem de
               * ownership do Rust não é uma heurística: é uma regra
               * estrutural do sistema de tipos, então não se perde com
               * indireção. */

    return 0;
}
