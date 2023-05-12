#ifndef HUFFMAN
#define HUFFMAN

#include <stdint.h>

/*
  @param orig original character
  @param code is a pointer to the decoding code
  @param code_length is the length of the code
  @param prob is the probability of the character 

*/
typedef struct {
  uint8_t orig;
  uint8_t *code;
  uint8_t code_length;
  double prob;
} char_info;

typedef struct {
    uint32_t card_orig;
    uint32_t distinct;
    char_info *nodes;
} encoding_tree;

/*
 @param caract cotains the caracter that node represents
 @param izq represents the code 0, also being the left child
 @param der represents the code 1, also being the right child
*/
typedef struct node {
  char caract;
  struct node *childs[2];
} decoding_tree;

/**
 * Compresses a file a writes de result to another file
 * @param path of the file to compress
 * @param dest path of the file to which to write the compressed information
 * @return NULL if succesfull or a string containing a brief description of the
 * error otherwise
 */
char* compress(char *path, char *dest);

/**
 */
char* decompress(char *path, char *dest);

/**
 * Print the characters with their respective new code
 */
void print_coding(encoding_tree);

#endif // !HUFFMAN
