#include "buffer/buffer.h"
#include "common/common.h"
#include "hamming/hamming.h"
#include "huffman/huffman.h"
#include "noise/generator.h"

#include <math.h>
#include <stdint.h>
#include <stdio.h>

int main() {
  buffered_reader reader;
  init_buffered_reader(&reader, "./todo.txt");

  buffered_writer writer;
  init_buffered_writer(&writer, "./todo.huf");

  bit_slice slice = read(&reader, 32);
  while (slice.base != NULL) {
    put_slice(&writer, slice);
    slice = read(&reader, 32);
  }

  free_buffered_reader(&reader);
  free_buffered_writer(&writer);

  return 0;
}
