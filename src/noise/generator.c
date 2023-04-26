#include "generator.h"

#include "../bitarr/bitarr.h"

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <time.h>

char* corrupt_i(FILE *fd, FILE *res, int block_size, uint32_t exponent){
    uint32_t  block_size_bytes = block_size / 8;
    uint64_t file_size, n_blocks;
    
    fread((void*)&n_blocks, sizeof(long), 1, fd);
    fread((void*)&file_size, sizeof(long), 1, fd);

    void *buffer = malloc(n_blocks * block_size_bytes);

    fread(buffer, 1, n_blocks * block_size_bytes, fd);

    srand(546514843103518461);

    int module_error = rand() % exponent;
    int position_error = rand() % block_size;

    flip_bit((void*)(buffer + module_error), position_error);

    fwrite((void*)&n_blocks, sizeof(long), 1, res);
    fwrite((void*)&file_size, sizeof(long), 1, res);

    fwrite(buffer, 1, n_blocks * block_size_bytes, res);
    
    free(buffer);
    return NULL;
}
