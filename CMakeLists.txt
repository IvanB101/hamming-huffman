cmake_minimum_required(VERSION 3.22)

project(hamming)

add_subdirectory(src/coding)
add_subdirectory(src/noise)

add_executable(hamming src/main.cpp)

target_link_libraries(hamming
    coding
    noise)
