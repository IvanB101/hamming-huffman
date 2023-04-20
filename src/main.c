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
    FILE *fd, *res, *pepe;

    char read_file[] = "/home/ivan/repositories/teoria-de-la-informacion/hamming/Prueba.txt";
    char write_file[] = "/home/ivan/repositories/teoria-de-la-informacion/hamming/Prueba.HA1";
    char result[] = "/home/ivan/repositories/teoria-de-la-informacion/hamming/Pepe.txt";

    if(!(fd = fopen(read_file, "rb"))) {
        printf("Error abriendo %s\n", read_file);
        perror(strerror(errno));
        return -1;
    }
    if(!(res = fopen(write_file, "wb+"))) {
        printf("Error abriendo %s\n", write_file);
        perror(strerror(errno));
        return -1;
    }
    if(!(pepe = fopen(result, "wb+"))) {
        printf("Error abriendo %s\n", result);
        perror(strerror(errno));
        return -1;
    }

    encode(fd, res, 32, 5);
    decode(res, pepe, 32, 5, 1);

    return 0;
}
