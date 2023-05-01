#include "huffman.h"

#include "../bitarr/bitarr.h"

#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *clone(char *str);

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
  for (int i; i < card_orig; i++) {
    ocurrencies[i] = 0;
  }

  uint8_t distinct = 0;
  uint64_t file_size;

  fseek(fd, 0L, SEEK_END);
  file_size = ftell(fd);
  rewind(fd);

  void *result = malloc(file_size);
  void *buffer = malloc(file_size);
  fread(buffer, 1, file_size, fd);

  for (int i; i < file_size; i++) {
    if (!ocurrencies[((char *)buffer)[i]]++) {
      distinct++;
    }
  }

  char_info *tree = (char_info *)malloc(sizeof(char_info) *
                                        (distinct * (distinct - 1) / 2 - 1));
  uint8_t j = 0;
  for (int i = 0; i < card_orig; i++) {
    if (ocurrencies[i]) {
      tree[j++] = (char_info){.orig = i,
                              .code = NULL,
                              .code_length = 0,
                              .prob = ocurrencies[i] / (double)file_size};
    }
  }

  uint8_t base = 0, new_base = distinct;
  for (int i = distinct; i > 2; i--) {
    int k = 2;
    char_info a = tree[base], b = tree[base + 1];
    char_info new = (char_info){
        .orig = 0, .code = NULL, .code_length = 1, .prob = a.prob + b.prob};

    while (tree[base + k].prob < new.prob) {
      tree[new_base + k - 2] = tree[base + k];
      tree[new_base + k - 2].orig = 1;
      k++;
    }
    tree[new_base + k - 2] = new;
    while (k - 1 < new_base) {
      tree[new_base + k - 1] = tree[base + k];
      tree[new_base + k - 1].orig = 1;
      k++;
    }

    base = new_base;
    new_base += i - 1;
  }

  tree[base].code = (char *)malloc(1);
  tree[base].code[0] = 0;
  tree[base + 1].code = (char *)malloc(1);
  tree[base + 1].code[0] = 1;
  for (int i = 2; i < distinct; i++) {
    new_base = base - i - 1;

    for (int k = 0; k < i - 1; k++) {
      char_info node = tree[base + k];
      if (node.orig) {
        tree[new_base + k].code = node.code;
        tree[new_base + k].code_length = node.code_length;
      } else {
        tree[new_base].code = clone(node.code);
        tree[new_base].code_length = node.code_length;
        push_code_bit(&tree[new_base], 0);

        tree[new_base + 1].code = node.code;
        tree[new_base + 1].code_length = node.code_length;
        push_code_bit(&tree[new_base], 1);

        k--;
      }
    }
  }

  char_info *table[card_orig];
  for (int i = 0; i < distinct; i++) {
    table[tree[i].orig] = &tree[i];
  }

  uint64_t buff_offset = 0;
  for (int i = 0; i < file_size; i++) {
    char temp = ((char *)buffer)[i];

    move((void *)tree[temp].code, result, 0, buff_offset,
         tree[temp].code_length);

    buff_offset += tree[temp].code_length;
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
  if(buff_offset % 8) {
      info_bytes++;
  }

  // Size in bytes of compressed information
  fwrite((void*)&info_bytes, 1, sizeof(uint64_t), res);
  // Size in bites of compressed information
  fwrite((void*)&buff_offset, 1, sizeof(uint64_t), res);
  // Compresed information
  fwrite(result, 1, info_bytes, res);

  free(buffer);
  free((void *)tree);

  fclose(fd);
  fclose(res);

  return NULL;
}

void push_code_bit(char_info *node, uint8_t bit) {
  node->code_length++;

  if ((node->code_length % 8) == 1) {
    char *temp = (char *)malloc(node->code_length / 8 + 1);
    if ((void *)node->code) {
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

char *clone(char *str) {
  char *ret = (char *)malloc(sizeof(str));
  for (int i = 0; i < sizeof(str); i++) {
    ret[i] = str[i];
  }

  return ret;
}
