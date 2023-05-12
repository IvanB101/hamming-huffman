add_library(huffman STATIC
    huffman.h
    huffman.c)

target_include_directories(coding PUBLIC
    ${CMAKE_CURRENT_LIST_DIR})

target_link_libraries(coding
    bitarr)
