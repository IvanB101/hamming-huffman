#ifndef HUFFMAN
#define HUFFMAN

#include <stdint.h>

typedef struct {
  char orig;
  char *code;
  uint8_t code_length;
  double prob;
} char_info;

char* compress(char *path, char *dest);

char* decompress(char *path, char *dest);

#endif // !HUFFMAN
