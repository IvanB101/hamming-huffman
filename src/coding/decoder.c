#include "coder.h"

#include "../bitarr/bitarr.h"
#include <stdio.h>
#include <stdlib.h>

void correct(void* block, int block_size, unsigned int exponent, void *masks);

int unpack(void* buffer, void* block, int block_size, int counter);

int decode(FILE *fd, FILE *res, int block_size, unsigned int exponent, int correction) {
    void *masks = init_masks();

    unsigned int  block_size_bytes = block_size / 8;
    unsigned long file_size, n_blocks;
    int counter = 0;
    
    fread((void*)&n_blocks, sizeof(long), 1, fd);
    
    fread((void*)&file_size, sizeof(long), 1, fd);

    void *buffer = malloc(n_blocks * block_size_bytes),
         *result = malloc(file_size + block_size_bytes);

    fread(buffer, 1, n_blocks * block_size_bytes, fd);

    for(int i = 0; i < n_blocks; i++) {
        // correct((void*)(buffer + i * block_size_bytes), block_size, exponent, masks);

        counter = unpack((void*)(buffer + i * block_size_bytes), 
                        (void*)(result),
                        block_size,
                        counter);
    }

    fwrite(result, 1, counter, res);
    
    return 0;
}

void correct(void* block, int block_size, unsigned int exponent, void *masks) {
    int sindrome, i;

    // calculates the block syndrome
    for(i = 0; i < exponent; i++) {
        sindrome |= masked_parity(block, (void*)(masks + i * MAX_BLOCK_SIZE), block_size) << i;
    }

    // Checks if the parity of the blocks needs correction
    if(parity(block, block_size)) {
        if(sindrome != 0){
            flip_bit(block, sindrome);
        } 
    }
}


int unpack(void* buffer, void* block, int block_size, int counter) {
    int remaining = block_size - 2, start_from = 2, start_to = counter, size = 1;

    while(remaining > 0) {
        move((void*)buffer, (void*)block, start_from, start_to, size);

        remaining -= size + 1;
        start_from += size + 1;
        start_to += size;

        size = (size << 1) + 1;
    }

    return start_to;
}
