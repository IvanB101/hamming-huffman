#include "coder.h"

#include "../bitarr/bitarr.h"

char masks[EXPONENT][MAX_BLOCK_SIZE];
int inicialized = 0;

void init_masks() {
    int i, j, m = 1;
    for(i = 0; i < EXPONENT; i++) {
        for(j = 0; j < MAX_BLOCK_SIZE; j++) {
            if((j + 1) & m) {
                set_bit((void*)masks[i], j);
            } else {
                reset_bit((void*)masks[i], j);
            }
        }
        m <<= 1;
    }

    inicialized = 1;
}
