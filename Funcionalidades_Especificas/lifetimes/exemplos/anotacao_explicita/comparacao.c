// comparacao.c — equivalente em C dos cenários de anotacao_explicita
//
// Compilar: gcc -Wall -Wextra -Wpedantic comparacao.c -o comparacao
// Rodar:    ./comparacao
//
// Aviso: a seção 2 devolve um ponteiro para uma variável local e o
// desreferencia de propósito — comportamento indefinido, pode
// imprimir lixo de memória ou travar, dependendo do compilador e das
// otimizações. É o ponto central do exemplo, não um bug.

#include <stdio.h>
#include <string.h>

// ---- Seção 1: equivalente ao exemplo2_com_anotacao (caso correto) ----
//
// Em Rust, `maior<'a>` exige a anotação porque tem duas entradas e uma
// saída. Em C, a mesma função compila sem exigir NADA — o compilador
// não tem nenhum conceito de lifetime para checar.
const char *maior_c(const char *x, const char *y) {
    return strlen(x) > strlen(y) ? x : y;
}

// ---- Seção 2: equivalente ao exemplo1_sem_anotacao, mas em C isso
// não vira erro de compilação em nenhuma das duas versões ----
//
// Aqui devolvemos ponteiro para uma variável local. Em Rust, o
// compilador rejeita (E0106/E0515). Em C, compila igual à seção 1 —
// só um aviso, se os flags de warning estiverem ligados.
const char *variavel_local(void) {
    char buffer[] = "dado temporario";
    return buffer; // endereço de `buffer` não é mais válido ao retornar
}

int main(void) {
    printf("--- Secao 1: duas entradas, uma saida (caso correto) ---\n");
    const char *a = "abcd";
    const char *b = "xyz";
    printf("Maior string: %s\n", maior_c(a, b));
    printf("(C aceitou sem exigir nenhuma anotacao — nao existe o\n");
    printf(" conceito de lifetime para o compilador checar aqui.)\n\n");

    printf("--- Secao 2: retorno de ponteiro para variavel local ---\n");
    const char *pendurado = variavel_local();
    printf("Tentando usar o ponteiro pendurado: %s\n", pendurado);
    printf("(Comportamento indefinido — o Rust teria rejeitado isso\n");
    printf(" na compilacao, com E0106 seguido de E0515.)\n");

    return 0;
}
