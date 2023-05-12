#include "huffman.h"

#include "../bitarr/bitarr.h"

#include <errno.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

decoding_tree *init_node();

decoding_tree *build_tree(encoding_tree tree);

char *decompress(char *path, char *dest) {
  FILE *fd, *res;
  uint64_t info_bytes, file_size, info_bits;

  fd = fopen(path, "rb");
  if (!fd) {
    return strerror(errno);
  }

  res = fopen(dest, "wb");
  if (!res) {
    return strerror(errno);
  }

  encoding_tree tree;
  fread((void *)&tree.distinct, sizeof(uint32_t), 1, fd);
  tree.nodes = (char_info *)malloc(tree.distinct * sizeof(char_info));
  for (int i = 0; i < tree.distinct; i++) {
    char_info *entry = &tree.nodes[i];
    // Character for the table entry
    fread((void *)&entry->orig, 1, 1, fd);
    // Length of character code
    fread((void *)&entry->code_length, 1, 1, fd);
    uint8_t len = ceil(entry->code_length / 8.0);
    // New code for character
    entry->code = (uint8_t *)malloc(len);
    fread((void *)entry->code, 1, len, fd);
    // Fraction of original files characters equal to this one
    fread((void *)&entry->prob, sizeof(double), 1, fd);
  }

  // Size in bytes of uncompressed information
  fread((void *)&file_size, 1, sizeof(uint64_t), fd);
  // Size in bites of compressed information
  fread((void *)&info_bits, 1, sizeof(uint64_t), fd);

  info_bytes = ceil(info_bits / 8.0);

  void *result = malloc(file_size);
  void *buffer = malloc(info_bytes);

  // Compresed information
  fread(buffer, 1, info_bytes, fd);

  decoding_tree *root = build_tree(tree);

  uint8_t bit = 0;
  uint32_t buff_index = 0, i = 0;
  for (; buff_index < file_size; buff_index++) {
    decoding_tree *aux = root;

    while (!aux->caract) {
      move(buffer, (void *)&bit, i++, 7, 1);

      aux = aux->childs[bit];
    }

    move((void *)&aux->caract, result, 0, buff_index * 8, 8);
  }

  printf("File size: %ld\n", file_size);

  fwrite(result, 1, file_size, res);

  return NULL;
}

decoding_tree *init_node() {
  decoding_tree *node = (decoding_tree *)malloc(sizeof(decoding_tree));

  node->caract = 0;
  node->childs[0] = NULL;
  node->childs[1] = NULL;

  return node;
}

decoding_tree *build_tree(encoding_tree tree) {
  decoding_tree *root = init_node();

  for (int i = 0; i < tree.distinct; i++) {
    decoding_tree *aux = root;
    char_info info = tree.nodes[i];

    for (int j = 0; j < info.code_length; j++) {
      uint8_t bit;
      move((void *)info.code, (void *)&bit, j, 7, 1);

      if (!aux->childs[bit]) {
        aux->childs[bit] = init_node();
      }
      aux = aux->childs[bit];
    }

    aux->caract = info.orig;
  }

  return root;
}
