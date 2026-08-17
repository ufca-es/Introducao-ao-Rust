/* Comparação com "Ways Variables and Data Interact: Move" e "Stack-Only
 * Data: Copy" — cap. 4.1 do Rust Book:
 * https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
 *
 * Compilar e rodar: gcc -Wall -Wextra comparacao.c -o comparacao && ./comparacao
 * AVISO: a ÚLTIMA seção deste programa trava DE PROPÓSITO (double free) --
 * é o ponto central da comparação: o que o Rust rejeita em tempo de
 * COMPILAÇÃO, o C deixa passar e só quebra em tempo de EXECUÇÃO. Por isso
 * ela fica por último: nada depois dela chega a executar.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    /* Seção 1 (equivalente ao exemplo 3 em Rust): um tipo escalar simples
     * (int) se comporta em C exatamente como um tipo Copy em Rust -- a
     * atribuição sempre copia o valor pela stack. Nenhuma diferença de
     * comportamento entre as duas linguagens aqui. */
    int x = 5;
    int y = x;
    printf("x = %d, y = %d\n", x, y);

    /* Seção 2 (equivalente ao exemplo 2 em Rust): reatribuir um ponteiro
     * NÃO libera automaticamente o valor antigo -- diferente do Rust, onde
     * `drop` roda imediatamente na reatribuição. Esquecer o free() abaixo
     * vazaria memória sem nenhum aviso do compilador. */
    char *s3 = malloc(6);
    strcpy(s3, "hello");
    printf("s3 antes = %s\n", s3);
    free(s3); /* preciso lembrar disso manualmente antes de reatribuir */
    s3 = malloc(6);
    strcpy(s3, "ahoy");
    printf("s3 depois = %s\n", s3);
    free(s3);

    /* Seção 3 (equivalente ao exemplo 1 em Rust) -- a última, porque
     * derruba o processo de propósito: */
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
