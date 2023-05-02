#ifndef HUFFMAN
#define HUFFMAN

#include <stdint.h>

typedef struct {
  char orig;
  char *code;
  uint8_t code_length;
  double prob;
} char_info;

/**
 * Compresses a file a writes de result to another file
 * @param path of the file to compress
 * @param dest path of the file to which to write the compressed information
 * @return NULL if succesfull or a string containing a brief description of the
 * error otherwise
 */
char* compress(char *path, char *dest);

char* decompress(char *path, char *dest);

#endif // !HUFFMAN
