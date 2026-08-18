// Exemplo do problema histórico de gerenciamento manual de memória em C.
// Este programa COMPILA normalmente (no máximo com avisos), mas contém
// um bug clássico de "use-after-free": a função libera a memória do
// buffer e, em seguida, retorna um ponteiro para essa mesma memória já
// liberada (dangling pointer).
//
// O comportamento em tempo de execução é INDEFINIDO: pode imprimir a
// string corretamente, pode imprimir lixo de memória, ou pode até
// derrubar o programa, dependendo do estado do heap no momento.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *criar_saudacao(void) {
    char *buffer = malloc(20);
    if (buffer == NULL) {
        return NULL;
    }
    strcpy(buffer, "Olá, Rust!");
    free(buffer);   // memória liberada...
    return buffer;  // ...mas o ponteiro ainda é retornado (dangling pointer)
}

int main(void) {
    char *msg = criar_saudacao();
    printf("%s\n", msg); // comportamento indefinido: use-after-free
    return 0;
}
