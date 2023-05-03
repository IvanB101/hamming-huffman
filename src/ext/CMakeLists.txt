cmake_minimum_required(VERSION 3.22)

set(This hamming)
project(${This} C CXX)

set(CMAKE_C_STANDARD 99)
set(CMAKE_CXX_STANDARD 14)

set(CMAKE_BUILD_TYPE Debug)

add_library(c_procedures STATIC
    src/bitarr/bitarr.h
    src/bitarr/bitarr.c
    src/hamming/hamming.h
    src/hamming/encoder.c
    src/hamming/decoder.c
    src/hamming/masks.c
    src/huffman/huffman.h
    src/huffman/compress.c
    src/huffman/decompress.c
    src/noise/generator.h
    src/noise/generator.c)
target_include_directories(c_procedures  PUBLIC
    ${CMAKE_CURRENT_LIST_DIR})
