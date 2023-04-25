cmake_minimum_required(VERSION 3.22)

set(This hamming)
project(${This} C CXX)

set(CMAKE_C_STANDARD 99)
set(CMAKE_CXX_STANDARD 14)

set(CMAKE_BUILD_TYPE Debug)

add_subdirectory(src/coding)
add_subdirectory(src/noise)
add_subdirectory(src/bitarr)

add_executable(${This} src/main.c)

target_link_libraries(${This}
    bitarr
    coding
    noise)

add_library(c_procedures STATIC
    src/c_procedures.h
    src/c_procedures.c)

target_include_directories(c_procedures  PUBLIC
    ${CMAKE_CURRENT_LIST_DIR})

target_link_libraries(${This}
    bitarr
    coding
    noise)

# For Testing
