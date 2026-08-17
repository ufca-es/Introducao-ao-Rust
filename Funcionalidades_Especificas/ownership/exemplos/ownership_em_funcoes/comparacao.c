/* Comparação com "Ownership and Functions" (Listing 4-3) — cap. 4.1 do
 * Rust Book:
 * https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
 *
 * Compilar e rodar: gcc -Wall -Wextra comparacao.c -o comparacao && ./comparacao
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void toma_posse(char *alguma_string) {
    printf("%s\n", alguma_string);
    free(alguma_string); /* a função decide liberar a memória */
}

void faz_copia(int algum_inteiro) {
    printf("%d\n", algum_inteiro);
}

int main(void) {
    char *s = malloc(6);
    strcpy(s, "hello");

    toma_posse(s); /* o ponteiro é copiado para o parâmetro; a função
                     * acima já liberou essa memória. O compilador C não
                     * tem ideia disso e não impede o uso de `s` depois. */

    /* Isso compilaria e RODARIA em C -- e seria um use-after-free, porque
     * a memória apontada por `s` já foi liberada dentro de toma_posse().
     * Descomente a linha abaixo para observar (comportamento indefinido:
     * pode "funcionar por sorte", imprimir lixo ou corromper o heap
     * silenciosamente -- ao contrário do double free do exemplo `move`,
     * isso nem sempre trava de forma óbvia, o que o torna ainda mais
     * perigoso):
     *
     *     printf("uso apos free: %s\n", s);
     *
     * Em Rust, o equivalente -- usar `s` depois de `toma_posse(s)` -- é
     * REJEITADO NA COMPILAÇÃO (o mesmo erro E0382 do exemplo `move`).
     * Aqui em C, o bug só se manifestaria em tempo de execução, e só se
     * você tiver a sorte de perceber. */

    int x = 5;
    faz_copia(x);
    printf("x ainda pode ser usado depois de faz_copia: %d\n", x); /* igual ao Rust */

    return 0;
}
