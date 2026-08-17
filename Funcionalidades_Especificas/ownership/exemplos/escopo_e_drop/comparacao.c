/* Comparação com "Memory and Allocation" — cap. 4.1 do Rust Book:
 * https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
 *
 * Compilar e rodar: gcc -Wall -Wextra comparacao.c -o comparacao && ./comparacao
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    /* Escopo de bloco existe em C também, e funciona igual ao Rust: uma
     * variável só é visível dentro das chaves onde foi declarada. */
    {
        const char *s = "hello";
        printf("dentro do escopo: %s\n", s);
    } /* s deixa de ser visível aqui -- mas isso é só uma questão de nome,
       * resolvida em tempo de compilação. */

    /* A diferença real está na memória alocada no heap. C não tem `drop`
     * automático: se o programador esquecer o free(), a memória continua
     * reservada (vazamento) mesmo depois que o ponteiro sai de escopo e
     * se torna inacessível -- e o compilador não avisa nada sobre isso. */
    {
        char *heap_s = malloc(6);
        strcpy(heap_s, "hello");
        printf("dentro do escopo (heap): %s\n", heap_s);
        free(heap_s); /* preciso lembrar de liberar manualmente */
    } /* se o free() acima fosse removido, o programa compilaria e rodaria
       * exatamente igual, só que vazando memória a cada execução deste
       * bloco -- em Rust isso não acontece silenciosamente: o `drop` é
       * parte do comportamento garantido do tipo `String`. */

    printf("fim do main\n");
    return 0;
}
