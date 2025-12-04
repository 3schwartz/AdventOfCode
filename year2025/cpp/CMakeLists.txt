cmake_minimum_required(VERSION 3.10.0)
project(aoc2025 VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(CTest)
enable_testing()

add_executable(day1
    day1/main.cpp
    day1/aoc.cpp
    common/common.cpp
)

add_executable(test_day1
    day1/test_aoc.cpp
    day1/aoc.cpp
    common/common.cpp
)
    
add_test(
    NAME test_day1
    COMMAND test_day1
)

add_executable(day2
    day2/main.cpp
    day2/aoc.cpp
    common/common.cpp
)

add_executable(day3
    day3/main.cpp
    day3/aoc.cpp
    common/common.cpp
)

add_executable(day4
    day4/main.cpp
    day4/aoc.cpp
    common/common.cpp
)

# add_executable(day0 
#     day0/main.cpp
#     day0/aoc.cpp
#     common/common.cpp
# )

# add_executable(test_day0
#     day0/test_aoc.cpp
#     day0/aoc.cpp
# )
    
# add_test(
#     NAME test_day0
#     COMMAND test_day0
# )
