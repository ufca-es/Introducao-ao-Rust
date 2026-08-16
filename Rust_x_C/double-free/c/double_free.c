#include <stdlib.h>

int main(void) {
    int *valor = malloc(sizeof(*valor));

    if (valor == NULL) {
        return 1;
    }

    *valor = 42;
    free(valor);

    /* Comportamento indefinido: a alocacao ja foi liberada. */
    free(valor);
    return 0;
}
