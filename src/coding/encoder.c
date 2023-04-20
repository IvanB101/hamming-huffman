#include "coder.h"

#include "../bitarr/bitarr.h"
#include "./masks.c"

#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>

int make_space(void* buff_old, void* buff_new, void* block, int block_size, int rem_old);
void protect(void* block, int block_size_bytes, unsigned int exponent, void *masks);

void test(FILE *fd, FILE *res) {
    encode(fd, res, 32, 5);
}

int encode(FILE *fd, FILE *res, unsigned int block_size, unsigned int exponent) {
    void *masks = init_masks();

    unsigned int info_bits, buff_index, remaining, block_size_bytes = block_size / 8;
    unsigned long file_size, n_blocks, res_size;

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

    for(int i = 0, buff_index = 0, remaining = block_size; i < n_blocks; i++) {
        remaining = make_space(
                (void*)(buffer + buff_index * block_size_bytes),
                (void*)(buffer + (buff_index + 1) * block_size_bytes),
                (void*)(result + i * block_size_bytes),
                block_size,
                remaining);

        protect((void*)(result + i * block_size_bytes), block_size_bytes, exponent, masks);

        if (remaining < 0) {
            remaining = -remaining;
        } else {
            buff_index++;
        }
    }

    fwrite((void*)&n_blocks, sizeof(long), 1, res);
    fwrite((void*)&file_size, sizeof(long), 1, res);

    fwrite(result, 1, n_blocks * block_size_bytes, res);

    free(buffer);
    free(result);

    return 0;
}

/**
 * Sets or resets control bits of the block to protect it
 * @param block pointer to the block to be protected
 * @param block_size size of the block to be protected, in bytes
 * @param exponent to which to elevate 2 to obtain block_size
 */
void protect(void* block, int block_size_bytes, unsigned int exponent, void *masks) {
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
 * Copies bits starting with remaining bits from buff_old and continuing with buff_new and 
 * puts them in block, leaving space for control bits for hamming.
 * @param buff_old first buffer to copy bits from 
 * @param buff_old second buffer to copy bits from 
 * @param block to which bits are copied
 * @param block_size size of the block, in bytes
 * @param rem_old remaining bits to be copied from buff_old
 *
 * buff_old, buff_new and block must be of the same size, otherwise behavior is undefined
 */
int make_space(void* buff_old, void* buff_new, void* block, int block_size, int rem_old) {
    int remaining = block_size - 2, start_from = block_size - rem_old, start_to = 2, size = 1;

    while(rem_old >= size) {
        move((void*)buff_old, (void*)block, start_from, start_to, size);

        remaining -= size + 1;
        start_to += size + 1;
        if(start_to == size - 1) {
            return -rem_old;
        }
        rem_old -= size;
        start_from += size;
        size = (size << 1) + 1;
    }

    move((void*)buff_old, (void*)block, start_from, start_to, rem_old);
    remaining -= rem_old;
    start_from = 0;
    start_to += rem_old;

    move((void*)buff_new, (void*)block, start_from, start_to, size - rem_old);
    remaining -= size - rem_old + 1;
    start_from += size - rem_old;
    start_to += size - rem_old + 1;
    size = (size << 1) + 1;

    while(remaining > 0) {
        move((void*)buff_new, (void*)block, start_from, start_to, size);

        remaining -= size + 1;
        start_from += size;
        start_to += size + 1;
        size = (size << 1) + 1;
    }

    return block_size  - start_from;
}
