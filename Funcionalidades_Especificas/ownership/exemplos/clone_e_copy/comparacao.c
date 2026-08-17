/* Comparação com "Variables and Data Interacting with Clone" e
 * "Stack-Only Data: Copy" — cap. 4.1 do Rust Book:
 * https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
 *
 * Compilar e rodar: gcc -Wall -Wextra comparacao.c -o comparacao && ./comparacao
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *clonar(const char *original) {
    /* C não tem clone() embutido: uma cópia profunda precisa ser escrita
     * à mão. Esquecer disso -- e simplesmente copiar o ponteiro -- é
     * exatamente o bug de double free do exemplo `move`. */
    size_t tamanho = strlen(original) + 1;
    char *copia = malloc(tamanho);
    memcpy(copia, original, tamanho);
    return copia;
}

int main(void) {
    char *s1 = malloc(6);
    strcpy(s1, "hello");

    char *s2 = clonar(s1); /* cópia profunda manual: dois buffers distintos */

    printf("s1 = %s, s2 = %s\n", s1, s2);

    free(s1);
    free(s2); /* seguro: cada ponteiro aponta para um endereço diferente */

    /* Já um tipo escalar simples (int) se comporta em C exatamente como
     * um tipo Copy em Rust: a atribuição sempre copia o valor pela
     * stack. Aqui não existe NENHUMA diferença de comportamento entre as
     * duas linguagens. */
    int x = 5;
    int y = x;
    printf("x = %d, y = %d\n", x, y);

    return 0;
}
