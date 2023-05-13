#include "generator.h"

#include "../bitarr/bitarr.h"

#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

char *corrupt(char *path, char *dest, uint64_t block_size, uint64_t exponent, double probability) {
  FILE *fd, *res;

  fd = fopen(path, "rb");
  if (!fd) {
    return strerror(errno);
  }
  res = fopen(dest, "wb");
  if (!res) {
    return strerror(errno);
  }

  uint32_t block_size_bytes = block_size / 8;
  uint64_t file_size, n_blocks;

  fread((void *)&n_blocks, sizeof(long), 1, fd);
  fread((void *)&file_size, sizeof(long), 1, fd);

  void *buffer = malloc(n_blocks * block_size_bytes);

  fread(buffer, 1, n_blocks * block_size_bytes, fd);

  time_t utc_now = time(NULL);
  srand(utc_now);

  int block_index = rand() % exponent;
  int block_offset = rand() % block_size;

  flip_bit((void *)(buffer + block_index), block_offset);

  fwrite((void *)&n_blocks, sizeof(long), 1, res);
  fwrite((void *)&file_size, sizeof(long), 1, res);

  fwrite(buffer, 1, n_blocks * block_size_bytes, res);

  free(buffer);

  fclose(fd);
  fclose(res);

  return NULL;
}
