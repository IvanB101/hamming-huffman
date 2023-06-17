#include "buffer.h"

#include "../bitarr/bitarr.h"
#include "../common/common.h"

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

void init_buffered_writer(buffered_writer *bw, char *path) {
  bw->file = fopen(path, "wb");
  bw->buffer = (uint8_t *)malloc(BUFFER_SIZE);
  bw->index = 0;
}

void free_buffered_writer(buffered_writer *bw) {
  uint64_t bytes_in_buffer = bw->index / 8 + bw->index % 8;
  fwrite(bw->buffer, 1, bytes_in_buffer, bw->file);

  fclose(bw->file);
  free((void *)bw->buffer);
}

byte_slice take_slice(buffered_writer *bw, uint64_t size) {
  byte_slice slice = {NULL, size};
  uint64_t available = BUFFER_SIZE - bw->index / 8;

  if (available < size) {
    fwrite(bw->buffer, 1, bw->index / 8, bw->file);
    bw->index = 0;
  }

  slice.base = bw->buffer + bw->index;

  bw->index += size * 8;

  return slice;
}

void put_slice(buffered_writer *bw, bit_slice slice) {
  uint64_t available = (uint64_t)BUFFER_SIZE - bw->index;

  if (available < slice.size) {
    move(slice.base, bw->buffer, slice.bit_offset, bw->index, available);

    slice.base += available / 8;
    slice.bit_offset = (slice.bit_offset + available) % 8;
    slice.size -= available;

    fwrite(bw->buffer, 1, BUFFER_SIZE, bw->file);
    bw->index = 0;
  }

  move(slice.base, bw->buffer, slice.bit_offset, bw->index, slice.size);
  bw->index += slice.size;
}
