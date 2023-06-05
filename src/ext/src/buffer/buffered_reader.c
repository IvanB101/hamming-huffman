#include "buffer.h"

#include "../common/common.h"

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

void init_buffered_reader(buffered_reader *br, char *path) {
  br->file = fopen(path, "rb");
  br->buffer = (uint8_t *)malloc(BUFFER_SIZE);

  fseek(br->file, 0L, SEEK_END);
  br->file_size = ftell(br->file) * 8;
  rewind(br->file);

  br->index = 0;
  br->last = 0;
  br->read_bits = 0;
}

void free_buffered_reader(buffered_reader *br) {
  fclose(br->file);
  free((void *)br->buffer);
}

bit_slice read(buffered_reader *br, uint64_t amount) {
  bit_slice slice = {NULL, 0, 0};
  uint64_t remaining = br->file_size - br->read_bits;
  uint64_t buf_remaining = br->last - br->index;

  if (!remaining && !buf_remaining) {
    return slice;
  }
  if (!buf_remaining) {
    br->last = fread(br->buffer, 1, BUFFER_SIZE, br->file) * 8;
    br->index = 0;
    br->read_bits += br->last;

    buf_remaining = br->last;
  }

  slice.base = br->buffer + br->index / 8;
  slice.size = min(buf_remaining, amount);
  slice.bit_offset = br->index % 8;

  br->index += slice.size;

  return slice;
}
