#include "coder.h"

#include "../bitarr/bitarr.h"
#include "./masks.c"
#include <stdio.h>

int make_space(void* buff_old, void* buff_new, void* block, int block_size, int rem_old);
void protect(void* block, int block_size, unsigned int exponent);

void test() {
    char buff_new[] = {0xee, 0xf1, 0, 0};
    char buff_old[] = {0, 0, 0, 0};
    char block[] = {0xff, 0xff, 0, 0};

    init_masks();

    printf("Buff_new: %s\n", to_bit_string((void*)buff_new, sizeof(buff_new)));

    int i;
    for(i = 0; i < 5; i++) {
        short p = masked_parity((void*)buff_new, (void*)masks[i], sizeof(buff_new));
        printf("Mask %d: %s", i, to_bit_string((void*)masks[i], sizeof(buff_new)));
        printf("Parity %d: %d\n", i, p);
    }

    protect((void*)buff_new, sizeof(buff_new), 5);
    printf("Protected: %s\n", to_bit_string((void*)buff_new, sizeof(buff_new)));
}

int encode(FILE fd, unsigned int block_size, unsigned int exponent) {
    if(!inicialized) {
        init_masks();
    }

    return -1;
}

/**
 * Sets or resets control bits of the block to protect it
 * @param block pointer to the block to be protected
 * @param block_size size of the block to be protected, in bytes
 * @param exponent to which to elevate 2 to obtain block_size
*/
void protect(void* block, int block_size, unsigned int exponent) {
    int i, j = 1;
    for(i = 0; i < exponent; i++) {
        if(masked_parity(block, (void*)masks[i], block_size)) {
            flip_bit(block, j - 1);
        }
        
        j <<= 1;
    }

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

