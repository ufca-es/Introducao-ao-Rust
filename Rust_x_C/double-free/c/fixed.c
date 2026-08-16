#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int *valor = malloc(sizeof(*valor));

    if (valor == NULL) {
        fprintf(stderr, "Falha ao alocar memoria.\n");
        return 1;
    }

    *valor = 42;
    printf("Valor: %d\n", *valor);

    free(valor);
    valor = NULL;
    return 0;
}
