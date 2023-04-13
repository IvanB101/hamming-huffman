add_library(bitvec STATIC
    bitvec.h
    bitvec.c)

target_include_directories(bitvec PUBLIC
    ${CMAKE_CURRENT_LIST_DIR})
