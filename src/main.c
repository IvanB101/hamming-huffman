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

    char read_file[] = "../Primero.txt";
    char write_file[] = "../Intermedio.HA1";
    char error_file[] = "../ConError.HE1";
    char result[] = "../Final.txt";

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
    if(!(pepe = fopen(result, "wb"))) {
        printf("Error abriendo %s\n", result);
        perror(strerror(errno));
        return -1;
    }
    if(!(error = fopen(error_file, "wb+"))) {
        printf("Error abriendo %s\n", error_file);
        perror(strerror(errno));
        return -1;
    }

    encode(fd, res, 32, 5);

    rewind(fd);
    rewind(res);
    rewind(pepe);
    rewind(error);

    corrupt(res, error, 32, 5);
    
    rewind(fd);
    rewind(res);
    rewind(pepe);
    rewind(error);

    decode(error, pepe, 32, 5, 1);

    return 0;
}
