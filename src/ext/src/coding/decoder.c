#include "coder.h"

#include "../bitarr/bitarr.h"

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <errno.h>

void correct(void* block, uint32_t block_size, uint32_t exponent, void *masks);

int unpack(void* buffer, void* result, uint32_t block_size, uint32_t buff_offset);

char* decode(char *path, char *dest, uint64_t block_size, uint64_t  exponent, int correction) {
    FILE *fd, *res;

    fd = fopen(path, "rb");
    if(!fd) {
        return strerror(errno);
    }
    res = fopen(dest, "wb");
    if(!res) {
        return strerror(errno);
    }

    void *masks = init_masks();

    uint32_t  block_size_bytes = block_size / 8;
    uint64_t file_size, n_blocks;
    int buff_offset = 0;
    
    fread((void*)&n_blocks, sizeof(long), 1, fd);
    fread((void*)&file_size, sizeof(long), 1, fd);

    void *buffer = malloc(n_blocks * block_size_bytes),
         *result = malloc(file_size + block_size_bytes);

    fread(buffer, 1, n_blocks * block_size_bytes, fd);

    for(int i = 0; i < n_blocks; i++) {
        void *block = (void*)(buffer + i * block_size_bytes);

        if(correction) {
            correct(block, block_size_bytes, exponent, masks);
        }

        buff_offset = unpack(block, 
                        result,
                        block_size,
                        buff_offset);
    }

    fwrite(result, 1, file_size, res);
    
    free(buffer);
    free(result);

    fclose(fd);
    fclose(res);
    
    return NULL;
}

void correct(void* block, uint32_t block_size_bytes, uint32_t exponent, void *masks) {
    uint32_t sindrome = 0, i;

    // calculates the block syndrome
    for(i = 0; i < exponent; i++) {
        sindrome |= (masked_parity(block, (void*)(masks + i * MAX_BLOCK_SIZE), block_size_bytes) << i);
    }

    // Checks if the parity of the blocks needs correction
    if(parity(block, block_size_bytes)) {
        if(sindrome != 0){
            flip_bit(block, sindrome-1);
        } 
    }
}


int unpack(void* buffer, void* result, uint32_t block_size, uint32_t buff_offset) {
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

