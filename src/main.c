#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>
#include <math.h>
#include <error.h>
#include <errno.h>

#include "coding/coder.h"
#include "noise/generator.h"
#include "bitarr/bitarr.h"

int main() {
    FILE *fd, *res;

    char read_file[] = "Prueba.txt";
    char write_file[] = "Prueba.HA1";

    if(!(fd = fopen(read_file, "rb"))) {
        printf("Error abriendo %s\n", read_file);
        perror(strerror(errno));
    }
    if(!(fd = fopen(write_file, "wb+"))) {
        printf("Error abriendo %s\n", write_file);
        perror(strerror(errno));
    }

    test(fd, res);

    return 0;
}
