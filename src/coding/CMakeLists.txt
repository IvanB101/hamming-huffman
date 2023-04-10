add_library(coding STATIC
    coder.h
    coder.cpp)

target_include_directories(coding PUBLIC
    ${CMAKE_CURRENT_LIST_DIR})
