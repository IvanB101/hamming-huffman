#ifndef CODER
#define CODER

#include <stdio.h>

#define MAX_BLOCK_SIZE 65536
#define EXPONENT 16

int encode(FILE fd, unsigned int block_size, unsigned int exponent);

int decode(FILE fd, int block_size, int correct);

void test();

#endif 
