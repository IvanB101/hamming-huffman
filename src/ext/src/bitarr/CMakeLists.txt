add_library(bitarr STATIC
    bitarr.h
    bitarr.c)

target_include_directories(bitarr PUBLIC
    ${CMAKE_CURRENT_LIST_DIR})
