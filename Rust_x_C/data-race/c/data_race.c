#include <stdio.h>
#include <pthread.h>

enum { INCREMENTOS = 1000000 };

static int contador = 0;

void *incrementar(void *arg) {
    (void)arg;

    for (int i = 0; i < INCREMENTOS; i++) {
        contador++;
    }

    return NULL;
}

int main(void) {
    pthread_t primeira;
    pthread_t segunda;

    if (pthread_create(&primeira, NULL, incrementar, NULL) != 0) {
        fprintf(stderr, "Falha ao criar a primeira thread.\n");
        return 1;
    }

    if (pthread_create(&segunda, NULL, incrementar, NULL) != 0) {
        fprintf(stderr, "Falha ao criar a segunda thread.\n");
        pthread_join(primeira, NULL);
        return 1;
    }

    pthread_join(primeira, NULL);
    pthread_join(segunda, NULL);

    /* O resultado nao e confiavel: as threads disputam o mesmo objeto. */
    printf("Contador: %d\n", contador);
    return 0;
}
