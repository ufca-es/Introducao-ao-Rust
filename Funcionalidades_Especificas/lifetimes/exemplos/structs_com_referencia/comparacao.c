// comparacao.c — equivalente em C dos cenários de structs_com_referencia
//
// Compilar: gcc -Wall -Wextra -Wpedantic comparacao.c -o comparacao
// Rodar:    ./comparacao
//
// Aviso: a seção 2 usa uma struct cujo ponteiro aponta para um buffer
// já fora de escopo — comportamento indefinido de propósito, é o
// ponto central do exemplo.

#include <stdio.h>

// Equivalente a `struct Trecho<'a> { texto: &'a str }`. Em C, o
// struct não tem NENHUM jeito de declarar "este ponteiro só é válido
// enquanto tal outra variável existir" — é só um ponteiro comum.
typedef struct {
    const char *texto;
} Trecho;

int main(void) {
    printf("--- Secao 1: struct valida (dado ainda vivo) ---\n");
    char romance[] = "Era uma vez...";
    Trecho primeiro = { .texto = romance };
    printf("%.4s\n", primeiro.texto);
    printf("(Igual ao exemplo1_struct_valida.rs — funciona nas duas\n");
    printf(" linguagens, porque o dado referenciado ainda existe.)\n\n");

    printf("--- Secao 2: struct com ponteiro pendurado ---\n");
    Trecho pendurado;
    {
        char romance_local[] = "Era uma vez...";
        pendurado.texto = romance_local;
    } // `romance_local` sai de escopo aqui — o endereço não é mais válido
    printf("Tentando usar a struct depois do escopo: %.4s\n", pendurado.texto);
    printf("(Comportamento indefinido — o Rust teria rejeitado isso\n");
    printf(" na compilacao, com E0597, igual ao exemplo2_struct_invalida.rs)\n");

    return 0;
}
