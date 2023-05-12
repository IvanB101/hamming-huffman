#ifndef MASKS
#define MASKS

#include "hamming.h"

#include "../bitarr/bitarr.h"

#include <stdlib.h>

void *masks;
int inicialized = 0;

void *init_masks() {
  if (!inicialized) {
    masks = malloc(EXPONENT * MAX_BLOCK_SIZE);

    int i, j, m = 1;
    for (i = 0; i < EXPONENT; i++) {
      for (j = 0; j < MAX_BLOCK_SIZE; j++) {
        if ((j + 1) & m) {
          set_bit((void *)(masks + i * MAX_BLOCK_SIZE), j);
        } else {
          reset_bit((void *)(masks + i * MAX_BLOCK_SIZE), j);
        }
      }
      m <<= 1;
    }

    inicialized = 1;
  }
  return masks;
}

#endif
