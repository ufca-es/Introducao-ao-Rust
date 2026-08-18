#include "camada_dma.h"

#include <stdio.h>

int main(void) {
    int dados[] = {10, 20, 30, 40};
    size_t quantidade = sizeof(dados) / sizeof(dados[0]);

    printf("[Driver C] Chamando diretamente a API C...\n");
    int resultado = dma_transferir_v2(dados, quantidade);

    if (resultado != 0) {
        fprintf(stderr, "[Driver C] A transferencia falhou.\n");
        return 1;
    }

    return 0;
}
