/* Comparação com "Return Values and Scope" (Listing 4-4 e Listing 4-5) —
 * cap. 4.1 do Rust Book:
 * https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
 *
 * Compilar e rodar: gcc -Wall -Wextra comparacao.c -o comparacao && ./comparacao
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *da_posse(void) {
    char *alguma_string = malloc(6);
    strcpy(alguma_string, "yours");
    return alguma_string; /* a "posse" passa a ser do chamador só por
                            * CONVENÇÃO -- nada no compilador garante isso */
}

char *pega_e_devolve(char *uma_string) {
    return uma_string;
}

/* C não tem tuplas: o jeito idiomático de "devolver dois valores" é usar
 * um parâmetro de saída (out-parameter), como abaixo, ou um struct. */
size_t calcula_tamanho(char *s, char **s_devolvida) {
    *s_devolvida = s;
    return strlen(s);
}

int main(void) {
    /* Seção 1 (equivalente ao exemplo 1 em Rust): */
    char *s1 = da_posse();

    /* Seção 2 (equivalente ao exemplo 2 em Rust): */
    char *s2 = malloc(6);
    strcpy(s2, "hello");
    char *s3 = pega_e_devolve(s2); /* s3 aponta para o mesmo endereço de s2 */

    printf("s1 = %s, s3 = %s\n", s1, s3);

    /* Seção 3 (equivalente ao exemplo 3 em Rust): */
    char *s4 = malloc(6);
    strcpy(s4, "hello");
    char *s5;
    size_t tamanho = calcula_tamanho(s4, &s5); /* s5 aponta para o mesmo endereço de s4 */
    printf("o tamanho de '%s' e %zu\n", s5, tamanho);

    /* Cada malloc() precisa de EXATAMENTE um free() correspondente, e cabe
     * ao programador rastrear manualmente qual ponteiro ainda "possui" o
     * quê -- aqui, por exemplo, só s1, s3 e s5 são liberados, porque s2 e
     * s4 apontam para os mesmos endereços que s3 e s5. Nada no compilador
     * verifica esse raciocínio; é exatamente esse rastreamento manual que
     * o ownership do Rust substitui por checagem em tempo de compilação.
     * O próprio Rust Book usa esse exemplo para introduzir REFERÊNCIAS
     * (seção 4.2) como forma de evitar ter que devolver a posse o tempo
     * todo. */
    free(s1);
    free(s3);
    free(s5);

    return 0;
}
