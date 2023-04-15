#ifndef CODER
#define CODER

#include <stdio.h>

int encode(FILE fd, int block_size);

int decode(FILE fd, int block_size, int correct);

#endif 
