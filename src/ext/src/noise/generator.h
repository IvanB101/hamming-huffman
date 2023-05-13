#ifndef GENERATOR 
#define GENERATOR 

#include <stdio.h>
#include <stdint.h>

char* corrupt(char *path, char *dest, uint64_t block_size, uint64_t  exponent, double probability);

#endif 
