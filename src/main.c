#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>
#include <math.h>
#include <error.h>
#include <errno.h>

#include "./c_procedures.h"

#include "coding/coder.h"
#include "noise/generator.h"
#include "bitarr/bitarr.h"

int main() {
    FILE *fd, *res, *pepe, *error;
    uint32_t block_size = 2048;

    char file[] = "../Primero.txt";
    char* err;

    err = encode(file, block_size);
    if(err) {
        printf("Error: %s\n", err);
        return -1;
    }

    err = corrupt(file);
    if(err) {
        printf("Error: %s\n", err);
        return -1;
    }

    err = decode(file, 0);
    if(err) {
        printf("Error: %s\n", err);
        return -1;
    }

    return 0;
}
