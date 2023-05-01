#include <math.h>
#include <stdio.h>
#include <stdlib.h>

#include "bitarr.h"

int min(int a, int b) { return (a < b) ? a : b; }

void set_bit(void *arr, int position) {
  char mask = 1 << (7 - position % 8);

  ((char *)arr)[position / 8] |= mask;
}

void reset_bit(void *arr, int position) {
  char mask = 1 << (7 - position % 8);
  mask = ~mask;

  ((char *)arr)[position / 8] &= mask;
}

void flip_bit(void *arr, int position) {
  char mask = 1 << (7 - position % 8);

  if (((char *)arr)[position / 8] & mask) {
    reset_bit(arr, position);
  } else {
    set_bit(arr, position);
  }
}

void bit_and(void *arr1, void *arr2, int size1) {
  int i;
  for (i = 0; i < size1 / sizeof(int); i++) {
    ((int *)arr1)[i] &= ((int *)arr2)[i];
  }
  for (i = i * sizeof(int); i < size1; i++) {
    ((char *)arr1)[i] &= ((char *)arr2)[i];
  }
}

void bit_or(void *arr1, void *arr2, int size1) {
  int i;
  for (i = 0; i < size1 / sizeof(int); i++) {
    ((int *)arr1)[i] |= ((int *)arr2)[i];
  }
  for (i = i * sizeof(int); i < size1; i++) {
    ((char *)arr1)[i] |= ((char *)arr2)[i];
  }
}

void bit_not(void *arr1, int size) {
  int i;
  for (i = 0; i < size / sizeof(int); i++) {
    ((int *)arr1)[i] = ~((int *)arr1)[i];
  }
  for (i = i * sizeof(int); i < size; i++) {
    ((char *)arr1)[i] = ~((char *)arr1)[i];
  }
}

void move(void *from, void *to, int start_from, int start_to, int size) {
  int passed = 0, current_from = start_from, current_to = start_to;

  while (passed < size) {
    int dist_from = 8 - (current_from % 8);
    int dist_to = 8 - (current_to % 8);
    int to_move = min(min(dist_from, dist_to), size - passed);

    unsigned char mask = ((int)1 << to_move) - 1;

    unsigned char char_from = ((unsigned char *)from)[current_from / 8];
    unsigned char temp = char_from & (mask << (dist_from - to_move));

    mask <<= dist_to - to_move;
    int diference = dist_from - dist_to;
    if (diference < 0) {
      temp <<= -diference;
    } else {
      temp >>= diference;
    }

    unsigned char *char_to = &((unsigned char *)to)[current_to / 8];
    *char_to &= ~mask;
    *char_to |= temp;

    passed += to_move;
    current_from += to_move;
    current_to += to_move;
  }
}

short parity(void *arr, unsigned int size) {
  unsigned int i;
  unsigned char temp = ((unsigned char *)arr)[0], res = 0;

  for (i = 1; i < size; i++) {
    temp ^= ((unsigned char *)arr)[i];
  }

  for (i = 0; i < 8; i++) {
    res ^= temp & 1;

    temp >>= 1;
  }

  return res;
}

short masked_parity(void *arr, void *mask, unsigned int size) {
  unsigned int i;
  unsigned char temp = ((unsigned char *)arr)[0] & ((unsigned char *)mask)[0],
                res = 0;

  for (i = 1; i < size; i++) {
    temp ^= ((unsigned char *)arr)[i] & ((unsigned char *)mask)[i];
  }

  for (i = 0; i < 8; i++) {
    res ^= temp & 1;

    temp >>= 1;
  }

  return res;
}

char *to_bit_string(void *arr, int size) {
  char *ret = (char *)malloc(size * 9 + 1);

  int i, j, m;
  for (i = 0; i < size; i++) {
    char temp = ((char *)arr)[i];

    for (j = 0, m = 1 << 7; j < 8; j++) {
      ret[i * 9 + j] = ((temp & m) != 0) + '0';

      m >>= 1;
    }
    ret[i * 9 + j] = ' ';
  }
  ret[size * 9] = '\0';

  return ret;
}
