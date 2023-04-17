#include "coder.h"

#include "../bitarr/bitarr.h"
#include "./masks.c"
#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>

#include <unistd.h>

int make_space(void* buff_old, void* buff_new, void* block, int block_size, int rem_old);
void protect(void* block, int block_size, unsigned int exponent);

void test() {
    FILE *fd, *res;
    
    char buff[200];
    if(!getcwd(buff, sizeof(buff))) {
        perror(strerror(errno));
        return;
    }

    printf("CWD: %s\n", buff);
    if(!(fd = fopen("/home/ivan/repositories/teoria-de-la-informacion/hamming/Prueba.txt", "rb"))) {
        printf("Error al abrir el archivo de lectura\n");
        perror(strerror(errno));
        return;
    }
    if(!(res = fopen("/home/ivan/repositories/teoria-de-la-informacion/hamming/Prueba.HA1", "rb"))) {
        printf("Error al abrir el archivo de escritura\n");
        perror(strerror(errno));
        return;
    }

    encode(fd, res, 32, 5);
}

int encode(FILE *fd, FILE *res, unsigned int block_size, unsigned int exponent) {
    if(!inicialized) {
        init_masks();
    }

    void* buffer = malloc(block_size);

    unsigned int read, total = 0;
    while((read = fread(buffer, 1, block_size, fd)) > 0) {
        total += read;

        fwrite(buffer, 1, block_size, fd);
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

