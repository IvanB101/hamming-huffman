#include "huffman.h"

#include "../bitarr/bitarr.h"

#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/**
 * Clones a string
 * @param str pointer to the beginig of the string to clone
 * @return a clone of str
 */
char *str_clone(char *str);

/**
 * Adds a bit to the new code of a character
 * May have to realocate the string in the heap if it has grown past the string
 * capacity
 * @param node pointer to the node containing the code
 * @param bit value of the bit to append
 */
void push_code_bit(char_info *node, uint8_t bit);

char *compress(char *path, char *dest) {
  FILE *fd, *res;

  fd = fopen(path, "rb");
  if (!fd) {
    return strerror(errno);
  }
  res = fopen(dest, "wb");
  if (!res) {
    return strerror(errno);
  }

  uint16_t card_orig = 128;
  uint16_t ocurrencies[card_orig];
  for (int i = 0; i < card_orig; i++) {
    ocurrencies[i] = 0;
  }

  uint16_t distinct = 0;
  uint64_t file_size;

  fseek(fd, 0L, SEEK_END);
  file_size = ftell(fd);
  rewind(fd);

  void *result = malloc(file_size);
  void *buffer = malloc(file_size);
  fread(buffer, 1, file_size, fd);

  // Counting ocurrencies of each character
  for (int i = 0; i < file_size; i++) {
    if (!ocurrencies[((char *)buffer)[i]]++) {
      distinct++;
    }
  }

  // Colecting ocurrencies for each character
  char_info *tree = (char_info *)malloc(sizeof(char_info) *
                                        (distinct * (distinct + 1) / 2 - 1));
  uint8_t j = 0;
  for (int i = 0; i < card_orig; i++) {
    // Passing ocurrencies to a better format for code generation
    if (ocurrencies[i]) {
      tree[j] = (char_info){.orig = i,
                            .code = NULL,
                            .code_length = 0,
                            .prob = ocurrencies[i] / (double)file_size};
      j++;
    }
  }

  // Generation of tree, merging two character with less ocurrencies
  uint32_t base = 0, new_base = distinct;
  for (int i = distinct - 1; i >= 2; i--) {
    int k = 0;
    char_info a = tree[base], b = tree[base + 1];
    char_info new = (char_info){
        .orig = 0, .code = NULL, .code_length = 0, .prob = a.prob + b.prob};

    for (; k - 1 < i; k++) {
      if (tree[base + k + 2].prob < new.prob) {
        break;
      }
      tree[new_base + k].prob = tree[base + k + 2].prob;
      tree[new_base + k].orig = 1;
    }
    tree[new_base + k] = new;
    k++;
    for (; k < i; k++) {
      tree[new_base + k].prob = tree[base + k + 1].prob;
      tree[new_base + k].orig = 1;
    }

    base = new_base;
    new_base += i;
  }

  // Reduction of the tree, generating new encoding
  tree[base].code = (char *)malloc(1);
  tree[base].code[0] = 0;
  tree[base].code_length = 1;
  tree[base + 1].code = (char *)malloc(1);
  tree[base + 1].code[0] = 1;
  tree[base + 1].code_length = 1;
  for (int i = 2; i < distinct; i++) {
    new_base = base - i - 1;
    char_info node;

    int k = 0;
    while (node.orig) {
      node = tree[base + k];
      tree[new_base + k + 2].code = node.code;
      tree[new_base + k + 2].code_length = node.code_length;
      k++;
    }
    node = tree[base + k];

    tree[new_base].code = str_clone(node.code);
    tree[new_base].code_length = node.code_length;
    push_code_bit(&tree[new_base], 0);

    tree[new_base + 1].code = node.code;
    tree[new_base + 1].code_length = node.code_length;
    push_code_bit(&tree[new_base], 1);

    for (; k + 1 < i; k++) {
      node = tree[base + k + 1];
      tree[new_base + k + 2].code = node.code;
      tree[new_base + k + 2].code_length = node.code_length;
    }

    base = new_base;
  }

  // Puting resulting codes in arrays for faster access
  char_info *table[card_orig];
  for (int i = 0; i < distinct; i++) {
    table[tree[i].orig] = &tree[i];
  }

  // Coding of information previosly read in buffer
  uint64_t buff_offset = 0;
  for (int i = 0; i < file_size; i++) {
    char temp = ((char *)buffer)[i];

    move((void *)table[temp]->code, result, 0, buff_offset,
         table[temp]->code_length);

    buff_offset += table[temp]->code_length;
  }

  // Writing the results in the file
  // Number of table entries
  fwrite((void *)&distinct, 1, 1, res);
  for (int i = 0; i < distinct; i++) {
    char_info entry = tree[i];
    // Character for the table entry
    fwrite((void *)&entry.orig, 1, 1, res);
    // Length of character code
    fwrite((void *)&entry.code_length, 1, 1, res);
    // New code for character
    fwrite((void *)&entry.code, entry.code_length, 1, res);
    // Fraction of original files characters equal to this one
    fwrite((void *)&entry.prob, sizeof(double), 1, res);
  }

  uint64_t info_bytes = buff_offset / 8;
  if (buff_offset % 8) {
    info_bytes++;
  }

  // Size in bytes of compressed information
  fwrite((void *)&info_bytes, 1, sizeof(uint64_t), res);
  // Size in bites of compressed information
  fwrite((void *)&buff_offset, 1, sizeof(uint64_t), res);
  // Compresed information
  fwrite(result, 1, info_bytes, res);

  free(buffer);
  free((void *)tree);
  free(result);

  fclose(fd);
  fclose(res);

  return NULL;
}

void push_code_bit(char_info *node, uint8_t bit) {
  node->code_length++;

  if ((node->code_length % 8) == 1) {
    char *temp = (char *)malloc(node->code_length / 8 + 1);
    if (node->code) {
      move((void *)node->code, temp, 0, 0, node->code_length - 1);
      free((void *)node->code);
    }
    node->code = temp;
  }

  if (bit) {
    set_bit((void *)node->code, node->code_length - 1);
  } else {
    reset_bit((void *)node->code, node->code_length - 1);
  }
}

char *str_clone(char *str) {
  char *ret = (char *)malloc(sizeof(str));
  for (int i = 0; i < sizeof(str); i++) {
    ret[i] = str[i];
  }

  return ret;
}
