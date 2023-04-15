add_library(coding STATIC
    coder.h
    encoder.c
    decoder.c)

target_include_directories(coding PUBLIC
    ${CMAKE_CURRENT_LIST_DIR})
