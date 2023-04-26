#ifndef C_PROCEDURES
#define C_PROCEDURES

#include <stdint.h>

char* encode(char* path, uint32_t block_size);

char* decode(char* path, int correct);

char* corrupt(char* path);

char* compress(char* path);

char* decompress(char* path);

#endif // !LIB
