#ifndef CODER
#define CODER

#include <stdio.h>

#define MAX_BLOCK_SIZE 65536
#define EXPONENT 16

/**
 * Uses hamming to encode text contained in fd and writes it in res, it uses
 * exponent bits for hamming plus 1 bit for parity in each block
 * @param fd file descriptor of file to encode
 * @param res file descriptor of file to write encoded data
 * @param block_size size of the block used for encoding
 * @param exponent to which you have to elevate 2 to obtain block_size
 *
 * For correct functioning, block_size has to be a power of 2 
*/
int encode(FILE *fd, FILE *res, unsigned int block_size, unsigned int exponent);

int decode(FILE fd, int block_size, int correct);

void test();

#endif 
