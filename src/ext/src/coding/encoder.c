#include "coder.h"

#include "../bitarr/bitarr.h"
#include "./masks.c"

#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <stdint.h>

int pack(void* buffer, void* block, uint32_t block_size, uint32_t buff_offset);

void protect(void* block, uint32_t block_size_bytes, uint32_t exponent, void *masks);

char* encode_i(FILE *fd, FILE *res, uint32_t block_size, unsigned int exponent) {
    void *masks = init_masks();

    uint32_t info_bits, buff_offset, block_size_bytes = block_size / 8;
    uint64_t file_size, n_blocks;

    fseek(fd, 0L, SEEK_END);
    file_size = ftell(fd);
    rewind(fd);

    info_bits = block_size - exponent - 1;
    n_blocks = file_size * 8 / info_bits;
    if(file_size * 8 % info_bits) {
        n_blocks++;
    }

    void *buffer = malloc(file_size + block_size_bytes),
         *result = malloc(n_blocks * block_size_bytes);

    fread(buffer, 1, file_size, fd);

    for(int i = 0, buff_offset = 0; i < n_blocks; i++) {
        void *block = (void*)(result + i * block_size_bytes);

        buff_offset = pack(
                buffer,
                block,
                block_size,
                buff_offset);

        protect(block, block_size_bytes, exponent, masks);
    }

    fwrite((void*)&n_blocks, sizeof(long), 1, res);
    fwrite((void*)&file_size, sizeof(long), 1, res);

    fwrite(result, 1, n_blocks * block_size_bytes, res);

    free(buffer);
    free(result);

    return NULL;
}

/**
 * Sets or resets control bits of the block to protect it
 * @param block pointer to the block to be protected
 * @param block_size size of the block to be protected, in bytes
 * @param exponent to which to elevate 2 to obtain block_size
 * @param masks array of masks to check parity of diferent groups of bits
 */
void protect(void* block, uint32_t block_size_bytes, uint32_t exponent, void *masks) {
    int i, j = 1;
    // Control bits for hamming
    for(i = 0; i < exponent; i++) {
        if(masked_parity(block, (void*)(masks + i * MAX_BLOCK_SIZE), block_size_bytes)) {
            flip_bit(block, j - 1);
        }

        j <<= 1;
    }

    // Parity check for entire block
    if(parity(block, block_size_bytes)) {
        flip_bit(block, j - 1);
    }
}

/**
 */
int pack(void* buffer, void* block, uint32_t block_size, uint32_t buff_offset) {
    int remaining = block_size - 2, start_from = buff_offset, start_to = 2, size = 1;

    while(remaining > 0) {
        move(buffer, block, start_from, start_to, size);

        remaining -= size + 1;
        start_from += size;
        start_to += size + 1;
        size = (size << 1) + 1;
    }
    
    return start_from;
}
