#include "huffman.h"

#include "../bitarr/bitarr.h"

#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void sort(char_info *arr, uint64_t size);
/**
 * Clones a string
 * @param str pointer to the beginig of the string to clone
 * @return a clone of str
 */
char *str_clone(char *str, uint64_t size);

/**
 * Adds a bit to the new code of a character
 * May have to realocate the string in the heap if it has grown past the string
 * capacity
 * @param node pointer to the node containing the code
 * @param bit value of the bit to append
 */
void push_code_bit(char_info *node, uint8_t bit);

/**
 * Generates an auxiliary structure for generating a new encoding for characters
 * for compressing a file
 * @return a tree with its leaves inicialized
 */
encoding_tree init_tree(void *buffer, uint32_t card_orig, uint64_t file_size);

/**
 * Constructs the tree from the leaves inicialized in init tree
 */
void build_tree(encoding_tree tree);

/**
 * Print the characters with their respective new code
 */
void print_coding(encoding_tree);

/**
 * Uses the built tree in build_tree for generating the new encoding_tree
 * @return a table in which each entry contains the new code for the value of
 * the index
 */
char_info **reduce_tree(encoding_tree tree);

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

  uint32_t card_orig = 128;
  uint64_t file_size;

  fseek(fd, 0L, SEEK_END);
  file_size = ftell(fd);
  rewind(fd);

  void *result = malloc(file_size);
  void *buffer = malloc(file_size);
  fread(buffer, 1, file_size, fd);

  encoding_tree tree = init_tree(buffer, card_orig, file_size);

  build_tree(tree);

  char_info **table = reduce_tree(tree);

  print_coding(tree);

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
  fwrite((void *)&tree.distinct, sizeof(uint32_t), 1, res);
  for (int i = 0; i < tree.distinct; i++) {
    char_info entry = tree.nodes[i];
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

  // Size in bytes of uncompressed information
  fwrite((void *)&file_size, 1, sizeof(uint64_t), res);
  // Size in bytes of compressed information
  fwrite((void *)&info_bytes, 1, sizeof(uint64_t), res);
  // Size in bites of compressed information
  fwrite((void *)&buff_offset, 1, sizeof(uint64_t), res);
  // Compresed information
  fwrite(result, 1, info_bytes, res);

  free(buffer);
  free((void *)tree.nodes);
  free((void *)table);
  free(result);

  fclose(fd);
  fclose(res);

  return NULL;
}

encoding_tree init_tree(void *buffer, uint32_t card_orig, uint64_t file_size) {
  encoding_tree tree = (encoding_tree){
      .card_orig = card_orig,
      .distinct = 0,
      .nodes = NULL,
  };

  // Array inicialization
  uint32_t *ocurrencies = (uint32_t *)malloc(card_orig * sizeof(uint32_t));
  for (int i = 0; i < card_orig; i++) {
    ocurrencies[i] = 0;
  }

  uint32_t distinct = 0;
  // Counting ocurrencies of each character
  for (int i = 0; i < file_size; i++) {
    if (!ocurrencies[((char *)buffer)[i]]++) {
      tree.distinct++;
    }
  }

  // Colecting ocurrencies for each character
  tree.nodes = (char_info *)malloc(
      sizeof(char_info) * (tree.distinct * (tree.distinct + 1) / 2 - 1));
  uint8_t j = 0;
  for (int i = 0; i < card_orig; i++) {
    // Passing ocurrencies to a better format for code generation
    if (ocurrencies[i]) {
      tree.nodes[j++] = (char_info){.orig = i,
                                    .code = NULL,
                                    .code_length = 0,
                                    .prob = ocurrencies[i] / (double)file_size};
    }
  }

  sort(tree.nodes, tree.distinct);

  return tree;
}

void build_tree(encoding_tree tree) {
  uint32_t base = 0, new_base = tree.distinct;
  char_info *nodes = tree.nodes;
  for (int i = tree.distinct - 1; i >= 2; i--) {
    int new_index = 0, index = 2;
    char_info a = nodes[base], b = nodes[base + 1];
    char_info new = (char_info){
        .orig = 0, .code = NULL, .code_length = 0, .prob = a.prob + b.prob};

    for (; (new_index + 1 < i) && (nodes[base + index].prob < new.prob);
         new_index++, index++) {
      nodes[new_base + new_index].prob = nodes[base + index].prob;
      nodes[new_base + new_index].orig = 1;
    }

    nodes[new_base + new_index] = new;
    new_index++;

    for (; new_index < i; new_index++, index++) {
      nodes[new_base + new_index].prob = nodes[base + index].prob;
      nodes[new_base + new_index].orig = 1;
    }

    base = new_base;
    new_base += i;
  }
}

char_info **reduce_tree(encoding_tree tree) {
  uint32_t base = (tree.distinct * (tree.distinct + 1) / 2 - 1) - 2, new_base;
  char_info *nodes = tree.nodes;

  nodes[base].code_length = nodes[base + 1].code_length = 1;
  nodes[base].code = (char *)malloc(1);
  nodes[base + 1].code = (char *)malloc(1);
  nodes[base].code[0] = 0;
  nodes[base + 1].code[0] = 1;
  for (int i = 2; i < tree.distinct; i++) {
    new_base = base - i - 1;
    char_info node = nodes[base];

    int index = 0, new_index = 2;
    while (node.orig) {
      nodes[new_base + new_index].code = node.code;
      nodes[new_base + new_index].code_length = node.code_length;
      index++;
      new_index++;
      node = nodes[base + index];
    }

    nodes[new_base].code = str_clone(node.code, node.code_length / 8 + 1);
    nodes[new_base].code_length = node.code_length;
    push_code_bit(&nodes[new_base], 0);

    nodes[new_base + 1].code = node.code;
    nodes[new_base + 1].code_length = node.code_length;
    push_code_bit(&nodes[new_base + 1], 1);

    index++;

    for (; index < i; index++, new_index++) {
      node = nodes[base + index];
      nodes[new_base + new_index].code = node.code;
      nodes[new_base + new_index].code_length = node.code_length;
    }

    base = new_base;
  }

  // Puting resulting codes in arrays for faster access
  char_info **table = malloc(tree.card_orig);
  for (int i = 0; i < tree.distinct; i++) {
    table[nodes[i].orig] = &nodes[i];
  }

  return table;
}

void sort(char_info *arr, uint64_t size) {
  for (int i = 0; i < size - 1; i++) {
    int min = i;

    for (int j = i + 1; j < size; j++) {
      if (arr[j].prob < arr[min].prob) {
        min = j;
      }
    }

    char_info temp = arr[i];
    arr[i] = arr[min];
    arr[min] = temp;
  }
}

void push_code_bit(char_info *node, uint8_t bit) {
  node->code_length++;

  if ((node->code_length % 8) == 1) {
    char *temp = (char *)malloc(node->code_length / 8 + 1);
    if (node->code_length > 1) {
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

char *str_clone(char *str, uint64_t size) {
  char *ret = (char *)malloc(size);
  for (int i = 0; i < size; i++) {
    ret[i] = str[i];
  }

  return ret;
}

void print_coding(encoding_tree tree) {
  for (int i = 0; i < tree.distinct; i++) {
    int len = tree.nodes[i].code_length / 8;
    if (tree.nodes[i].code_length % 8) {
      len++;
    }
    printf("Char: %d\t", tree.nodes[i].orig);
    printf("Code_length: %d\t", tree.nodes[i].code_length);
    printf("Prob: %.4lf\t", tree.nodes[i].prob);
    printf("Code: %s\n", to_bit_string((void *)tree.nodes[i].code, len));
  }
}
