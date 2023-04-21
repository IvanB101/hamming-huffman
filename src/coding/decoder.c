#include "coder.h"

#include "../bitarr/bitarr.h"
#include <stdio.h>
#include <stdlib.h>

void correct(void* block, int block_size, unsigned int exponent, void *masks);

int unpack(void* buffer, void* block, int block_size, int buff_offset);

int decode(FILE *fd, FILE *res, int block_size, unsigned int exponent, int correction) {
    void *masks = init_masks();

    unsigned int  block_size_bytes = block_size / 8;
    unsigned long file_size, n_blocks;
    int buff_offset = 0;
    
    fread((void*)&n_blocks, sizeof(long), 1, fd);
    fread((void*)&file_size, sizeof(long), 1, fd);

    void *buffer = malloc(n_blocks * block_size_bytes),
         *result = malloc(file_size + block_size_bytes);

    fread(buffer, 1, n_blocks * block_size_bytes, fd);

    for(int i = 0; i < n_blocks; i++) {
        void *block = (void*)(buffer + i * block_size_bytes);

        correct(block, block_size_bytes, exponent, masks);

        buff_offset = unpack(block, 
                        result,
                        block_size,
                        buff_offset);
    }

    fwrite(result, 1, file_size, res);
    
    return 0;
}

void correct(void* block, int block_size_bytes, unsigned int exponent, void *masks) {
    int sindrome, i;

    // calculates the block syndrome
    for(i = 0; i < exponent; i++) {
        sindrome |= masked_parity(block, (void*)(masks + i * MAX_BLOCK_SIZE), block_size_bytes) << i;
    }

    // Checks if the parity of the blocks needs correction
    if(parity(block, block_size_bytes)) {
        if(sindrome != 0){
            flip_bit(block, sindrome);
        } 
    }
}


int unpack(void* buffer, void* result, int block_size, int buff_offset) {
    int remaining = block_size - 2, start_from = 2, start_to = buff_offset, size = 1;

    while(remaining > 0) {
        move(buffer, result, start_from, start_to, size);

        remaining -= size + 1;
        start_from += size + 1;
        start_to += size;

        size = (size << 1) + 1;
    }

    return start_to;
}

int introduce_error(FILE *fd, FILE *res, int block_size, unsigned int exponent){
    unsigned int  block_size_bytes = block_size / 8;
    unsigned long file_size, n_blocks;
    
    fread((void*)&n_blocks, sizeof(long), 1, fd);
    fread((void*)&file_size, sizeof(long), 1, fd);

    void *buffer = malloc(n_blocks * block_size_bytes);

    fread(buffer, 1, n_blocks * block_size_bytes, fd);

    srand(0);

    int module_error = rand() % exponent;
    int position_error = rand() % block_size;

    flip_bit((void*)(buffer + module_error), position_error);

    fwrite((void*)&n_blocks, sizeof(long), 1, res);
    fwrite((void*)&file_size, sizeof(long), 1, res);

    fwrite(buffer, 1, n_blocks * block_size_bytes, res);
    
    free(buffer);
    return 0;
}
