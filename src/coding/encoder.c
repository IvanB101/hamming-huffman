#include "coder.h"

#include "../bitarr/bitarr.h"
#include "./masks.c"
#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <math.h>

#define BUFFER_SIZE 65536

int make_space(void* buff_old, void* buff_new, void* block, int block_size, int rem_old);
void protect(void* block, int block_size, unsigned int exponent);

void test(FILE *fd, FILE *res) {
    void* buffer = malloc(sizeof(int));
    fseek(fd, 0, SEEK_END);

    *(int*)buffer = (int)ftell(fd);
    fwrite(buffer, 4, 1, res);

    printf("Read size: %d\n", *(int*)buffer);
    
    *(int*)buffer = 0;
    printf("Buffer: %d\n", *(int*)buffer);

    fread(buffer, 4, 1, res);
    printf("Written size: %d\n", *(int*)buffer);
}

int encode(FILE *fd, FILE *res, unsigned int block_size, unsigned int exponent) {
    if(!inicialized) {
        init_masks();
    }
    unsigned int file_size, n_blocks, info_bits;


    fseek(fd, 0L, SEEK_END);
    file_size = ftell(fd);

    rewind(fd);

    info_bits = block_size - exponent - 1;
    n_blocks = ceil(file_size * 8.0 / info_bits);

    void *buffer = malloc(file_size),
         *result = malloc((n_blocks + 1) * block_size / 8);

    unsigned int buff_index = 0,
                 res_index = 0,
                 remaining = block_size;

    fread(buffer, 1, file_size, fd);

    while(res_index < n_blocks + 1) {
        remaining = make_space(
                (void*)(buffer + buff_index),
                (void*)(buffer + buff_index + 1),
                (void*)(result + res_index),
                block_size,
                remaining);

        protect((void*)(result + res_index), block_size, exponent);

        if (remaining < 0) {
            remaining = -remaining;
        } else {
            buff_index++;
        }

        res_index++;
    }

    fwrite((void*)&n_blocks, 4, 1, res);

    fwrite(result, 1, n_blocks * block_size / 8, res);

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
void protect(void* block, int block_size, unsigned int exponent) {
    int i, j = 1;
    // Control bits for hamming
    for(i = 0; i < exponent; i++) {
        if(masked_parity(block, (void*)masks[i], block_size)) {
            flip_bit(block, j - 1);
        }

        j <<= 1;
    }

    // Parity check for entire block
    if(parity(block, block_size)) {
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
    int remaining = block_size * 8 - 2, start_from = block_size * 8 - rem_old, start_to = 2, size = 1;

    while(rem_old >= size) {
        move((void*)buff_old, (void*)block, start_from, start_to, size);

        remaining -= size + 1;
        rem_old -= size;
        start_from += size;
        start_to += size + 1;
        size = (size << 1) + 1;

        if(start_to == size - 1) {
            return -rem_old;
        }
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

    return block_size * 8 - start_from;
}

