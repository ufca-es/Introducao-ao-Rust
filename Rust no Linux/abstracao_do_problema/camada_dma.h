#ifndef CAMADA_DMA_H
#define CAMADA_DMA_H

#include <stddef.h>

/*
 * API mantida pela equipe C.
 * O sufixo "v2" representa uma mudanca feita nessa API.
 */
int dma_transferir_v2(const int *dados, size_t quantidade);

#endif
