#include <stdio.h>

const int *criar_referencia_pendurada(void) {
    int valor = 42;

    return &valor;
}

int main(void) {
    const int *referencia = criar_referencia_pendurada();

    /* Comportamento indefinido: "valor" ja deixou de existir. */
    printf("Valor: %d\n", *referencia);
    return 0;
}
