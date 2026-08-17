/* Comparação com "The String Type" — cap. 4.1 do Rust Book:
 * https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
 *
 * Compilar e rodar: gcc -Wall -Wextra comparacao.c -o comparacao && ./comparacao
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    /* Um literal em C também fica embutido no binário, pensado para ser
     * imutável -- igual ao &str do Rust. */
    const char *literal = "hello";
    printf("literal: %s\n", literal);

    /* Para ter o equivalente a `String` (heap, crescível, mutável), é
     * preciso alocar e controlar tudo manualmente -- exatamente o que
     * `String` faz por você em Rust. */
    char *dinamica = malloc(6);
    strcpy(dinamica, "hello");

    const char *sufixo = ", world!";
    size_t novo_tamanho = strlen(dinamica) + strlen(sufixo) + 1;
    char *realocada = realloc(dinamica, novo_tamanho);
    if (realocada == NULL) {
        free(dinamica);
        return 1;
    }
    dinamica = realocada;
    strcat(dinamica, sufixo); /* equivalente manual de push_str() */

    printf("dinamica: %s\n", dinamica);
    free(dinamica);

    /* Diferença de segurança: em Rust, `literal` é `&str` e não tem
     * NENHUM método que permita escrita -- o código abaixo nem compila.
     * Em C, a mesma tentativa (com um ponteiro não constante) compila
     * limpo, mesmo com -Wall -Wextra, e só falha em tempo de execução.
     * Testado neste ambiente: o programa compila sem nenhum aviso e
     * termina com "Falha de segmentação" (SIGSEGV, código de saída 139).
     * Descomente para reproduzir:
     *
     *     char *literal_mut = "hello";
     *     literal_mut[0] = 'H';
     *     printf("%s\n", literal_mut);
     */

    printf("fim do main\n");
    return 0;
}
