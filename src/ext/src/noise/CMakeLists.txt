add_library(noise STATIC
    generator.h
    generator.c)

target_include_directories(noise PUBLIC
    ${CMAKE_CURRENT_LIST_DIR})
