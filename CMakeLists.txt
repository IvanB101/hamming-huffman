cmake_minimum_required(VERSION 3.22)

set(This hamming)
project(${This} C CXX)

set(CMAKE_C_STANDARD 99)
set(CMAKE_CXX_STANDARD 14)

set(CMAKE_BUILD_TYPE Debug)

add_subdirectory(src/bitvec)
add_subdirectory(src/coding)
add_subdirectory(src/noise)
add_subdirectory(src/bitarr)

add_executable(${This} src/main.c)

target_link_libraries(${This}
    bitvec
    bitarr
    coding
    noise)

# For Testing
