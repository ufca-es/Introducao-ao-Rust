/* Comparação com "References and Borrowing" — cap. 4.2 do Rust Book:
 * https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
 *
 * Compilar e rodar: gcc -Wall -Wextra comparacao.c -o comparacao && ./comparacao
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Em C, passar um ponteiro para uma função também não transfere posse --
 * nesse sentido, é parecido com um "empréstimo" do Rust: quem chama
 * continua dono da memória e pode usá-la depois. A diferença é que isso é
 * só uma CONVENÇÃO, não uma regra verificada pelo compilador. */
size_t calcula_tamanho(const char *s) {
    return strlen(s);
}

/* `const` sinaliza a intenção de não modificar o dado apontado, mas não é
 * uma garantia real: pode ser descartado com um cast, sem nenhum aviso do
 * compilador (mesmo com -Wall -Wextra). Em Rust, mutar através de uma
 * referência `&String` nem compila -- não existe um "cast" que remova
 * essa proteção. */
void muda_apesar_do_const(const char *s) {
    char *ptr_mutavel = (char *) s; /* remove o const silenciosamente */
    ptr_mutavel[0] = 'H';
}

int main(void) {
    /* Seção 1 (equivalente ao exemplo 1 em Rust): */
    char *s1 = malloc(6);
    strcpy(s1, "hello");

    size_t tamanho = calcula_tamanho(s1);
    printf("O tamanho de '%s' é %zu.\n", s1, tamanho); /* s1 continua
                                                          * válido aqui,
                                                          * igual ao Rust */

    muda_apesar_do_const(s1); /* compila e roda normalmente em C, apesar
                                * do parâmetro ser `const char *` */
    printf("depois de 'const' ser ignorado: %s\n", s1);

    /* Seção 2 (equivalente ao exemplo 2 em Rust): nada impede múltiplos
     * ponteiros somente-leitura para o mesmo buffer em C -- assim como em
     * Rust, isso é seguro (leitura simultânea não é um data race). */
    const char *r1 = s1;
    const char *r2 = s1;
    printf("r1 = %s, r2 = %s\n", r1, r2);

    free(s1);
    return 0;
}
