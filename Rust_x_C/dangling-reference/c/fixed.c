#include <stdio.h>
#include <stdlib.h>

int *criar_valor(void) {
    int *valor = malloc(sizeof(*valor));

    if (valor == NULL) {
        return NULL;
    }

    *valor = 42;
    return valor;
}

int main(void) {
    int *valor = criar_valor();

    if (valor == NULL) {
        fprintf(stderr, "Falha ao alocar memoria.\n");
        return 1;
    }

    printf("Valor: %d\n", *valor);
    free(valor);
    return 0;
}
