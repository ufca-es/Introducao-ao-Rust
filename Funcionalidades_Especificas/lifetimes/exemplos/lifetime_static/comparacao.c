// comparacao.c — equivalente em C dos cenários de lifetime_static
//
// Compilar: gcc -Wall -Wextra -Wpedantic comparacao.c -o comparacao
// Rodar:    ./comparacao
//
// Aviso: a seção 2 devolve um ponteiro para uma variável local de
// propósito — comportamento indefinido, é o ponto central do exemplo.

#include <stdio.h>

// C tem a noção de "storage duration" (static, automatic, dynamic),
// mas — diferente do Rust — o compilador NÃO verifica se um ponteiro
// devolvido respeita a storage duration de quem o criou.

// Equivalente ao exemplo1_string_literal.rs: string literals em C
// também têm storage duration estática (ficam no binário).
const char *string_literal_c(void) {
    return "isto e um string literal, tambem fica gravado no binario";
}

// Equivalente ao exemplo2_static_incorreto.rs: aqui devolvemos um
// ponteiro pra variável local (storage duration automática) como se
// fosse permanente. Em Rust isso é rejeitado (E0515). Em C, compila
// com só um aviso.
const char *nao_e_static_c(void) {
    char texto_local[] = "nao da pra ser permanente";
    return texto_local;
}

int main(void) {
    printf("--- Secao 1: string literal (storage duration estatica) ---\n");
    printf("%s\n", string_literal_c());
    printf("(Igual ao exemplo1_string_literal.rs — funciona nas duas\n");
    printf(" linguagens, o dado realmente é permanente.)\n\n");

    printf("--- Secao 2: ponteiro para variavel local, tratado como permanente ---\n");
    printf("%s\n", nao_e_static_c());
    printf("(Comportamento indefinido — o Rust teria rejeitado isso\n");
    printf(" na compilacao, com E0515, igual ao exemplo2_static_incorreto.rs)\n");

    return 0;
}
