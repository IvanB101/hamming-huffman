add_library(noise STATIC
    generator.h
    generator.cpp)

target_include_directories(noise PUBLIC
    ${CMAKE_CURRENT_LIST_DIR})
