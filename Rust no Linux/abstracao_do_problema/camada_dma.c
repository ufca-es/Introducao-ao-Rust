#include "camada_dma.h"

#include <stdio.h>

int dma_transferir_v2(const int *dados, size_t quantidade) {
    if (dados == NULL || quantidade == 0) {
        return -1;
    }

    printf("[C / DMA] Recebi %zu valores: ", quantidade);
    for (size_t i = 0; i < quantidade; i++) {
        printf("%d%s", dados[i], i + 1 == quantidade ? "" : ", ");
    }
    printf("\n[C / DMA] Transferencia concluida.\n");
    fflush(stdout);

    return 0;
}
