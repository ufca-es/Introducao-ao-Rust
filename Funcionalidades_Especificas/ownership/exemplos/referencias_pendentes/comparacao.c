/* Comparação com "Dangling References" — cap. 4.2 do Rust Book:
 * https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
 *
 * Compilar e rodar: gcc -Wall -Wextra comparacao.c -o comparacao && ./comparacao
 * AVISO: `pendente()` devolve de propósito um ponteiro para uma variável
 * local (stack) -- é o equivalente em C do que o Rust rejeita em tempo de
 * compilação (erro E0106). Comportamento indefinido: pode "funcionar por
 * sorte" ou imprimir lixo.
 */
#include <stdio.h>
#include <string.h>

/* O GCC de fato avisa sobre isso com -Wall (-Wreturn-local-addr), mas é
 * só um AVISO -- o código compila e gera um binário mesmo assim. Em
 * Rust, o equivalente é um ERRO de compilação: o binário nem chega a ser
 * gerado. */
char *pendente(void) {
    char s[6];
    strcpy(s, "hello"); /* buffer alocado na STACK deste frame */
    return s; /* devolve um ponteiro para memória que deixa de ser válida
               * assim que a função retorna e seu frame de stack é
               * desfeito */
}

int main(void) {
    /* Seção 1 (equivalente ao exemplo 1 em Rust): */
    char *s = pendente();
    printf("%s\n", s); /* comportamento indefinido: `s` aponta para um
                         * frame de stack que já não existe mais. Pode
                         * imprimir "hello" por coincidência (a memória
                         * ainda não foi sobrescrita), lixo, ou até
                         * segfaultar -- sem nenhuma garantia. */
    return 0;
}
