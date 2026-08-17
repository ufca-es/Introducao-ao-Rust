/* Comparação com "The String Type" e "Variables and Data Interacting with
 * Clone" — cap. 4.1 do Rust Book:
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
     * exatamente o bug de double free do notebook `move`. */
    size_t tamanho = strlen(original) + 1;
    char *copia = malloc(tamanho);
    memcpy(copia, original, tamanho);
    return copia;
}

int main(void) {
    /* Seção 1 (equivalente ao exemplo 1 em Rust): um literal em C também
     * fica embutido no binário, pensado para ser imutável -- igual ao
     * &str do Rust. */
    const char *literal = "hello";
    printf("literal: %s\n", literal);

    char *dinamica = malloc(6);
    strcpy(dinamica, "hello");
    printf("dinamica: %s\n", dinamica);

    /* Seção 2 (equivalente ao exemplo 2 em Rust): para ter o equivalente a
     * `String` (heap, crescível, mutável), é preciso alocar e controlar
     * tudo manualmente -- exatamente o que `String` faz por você em Rust,
     * inclusive a realocação quando o buffer atual não é suficiente. */
    const char *sufixo = ", world!";
    size_t novo_tamanho = strlen(dinamica) + strlen(sufixo) + 1;
    char *realocada = realloc(dinamica, novo_tamanho);
    if (realocada == NULL) {
        free(dinamica);
        return 1;
    }
    dinamica = realocada;
    strcat(dinamica, sufixo); /* equivalente manual de push_str() + push() */
    printf("dinamica: %s\n", dinamica);

    /* Diferença de segurança: em Rust, `literal` é `&str` e não tem NENHUM
     * método que permita escrita -- o código abaixo nem compila. Em C, a
     * mesma tentativa (com um ponteiro não constante) compila limpo, mesmo
     * com -Wall -Wextra, e só falha em tempo de execução. Testado neste
     * ambiente: o programa compila sem nenhum aviso e termina com "Falha
     * de segmentação" (SIGSEGV, código de saída 139). Descomente para
     * reproduzir:
     *
     *     char *literal_mut = "hello";
     *     literal_mut[0] = 'H';
     *     printf("%s\n", literal_mut);
     */

    /* Seção 3 (equivalente ao exemplo 3 em Rust): clonar() é uma cópia
     * profunda manual -- dois buffers distintos, cada um com seu próprio
     * free(). */
    char *s1 = malloc(6);
    strcpy(s1, "hello");
    char *s2 = clonar(s1);
    printf("s1 = %s, s2 = %s\n", s1, s2);

    free(s1);
    free(s2); /* seguro: cada ponteiro aponta para um endereço diferente */
    free(dinamica);

    return 0;
}
