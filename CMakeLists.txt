cmake_minimum_required(VERSION 3.22)

project(hamming)

add_subdirectory(src/bitvec)
add_subdirectory(src/coding)
add_subdirectory(src/noise)
add_subdirectory(src/bitarr)

add_executable(hamming src/main.c)

target_link_libraries(hamming
    bitvec
    bitarr
    coding
    noise)
