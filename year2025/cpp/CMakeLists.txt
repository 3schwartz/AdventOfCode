cmake_minimum_required(VERSION 3.10.0)
project(aoc2025 VERSION 0.1.0 LANGUAGES C CXX)

add_executable(day0 
    day0/main.cpp
    day0/aoc.cpp
    common/common.cpp
)

include(CTest)
enable_testing()

add_executable(test_day0
    day0/test_aoc.cpp
    day0/aoc.cpp
)
    
add_test(
    NAME test_day0
    COMMAND test_day0
)
