cmake_minimum_required(VERSION 3.10.0)
project(aoc2025 VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(CTest)
enable_testing()

add_executable(test_day0
    day0/test_aoc.cpp
    day0/aoc.cpp
    common/common.cpp
)
    
add_test(
    NAME test_day0
    COMMAND test_day0
)

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

add_executable(day5
    day5/main.cpp
    day5/aoc.cpp
    common/common.cpp
)

add_executable(test_day5
    day5/test_aoc.cpp
    day5/aoc.cpp
    common/common.cpp
)
    
add_test(
    NAME test_day5
    COMMAND test_day5
)

add_executable(day6
    day6/main.cpp
    day6/aoc.cpp
    common/common.cpp
)
add_executable(test_day6
    day6/test_aoc.cpp
    day6/aoc.cpp
    common/common.cpp
)
add_test(NAME test_day6 COMMAND test_day6)

add_executable(day7 day7/main.cpp day7/aoc.cpp common/common.cpp)
add_executable(test_day7 day7/test_aoc.cpp day7/aoc.cpp common/common.cpp)
add_test(NAME test_day7 COMMAND test_day7)

add_executable(day8 day8/main.cpp day8/aoc.cpp common/common.cpp)
add_executable(test_day8 day8/test_aoc.cpp day8/aoc.cpp common/common.cpp)
add_test(NAME test_day8 COMMAND test_day8)

add_executable(day9 day9/main.cpp day9/aoc.cpp common/common.cpp)
add_executable(test_day9 day9/test_aoc.cpp day8/aoc.cpp common/common.cpp)
add_test(NAME test_day9 COMMAND test_day9)

# add_executable(day0 
#     day0/main.cpp
#     day0/aoc.cpp
#     common/common.cpp
# )

# add_executable(test_day0
#     day0/test_aoc.cpp
#     day0/aoc.cpp
#     common/common.cpp
# )
    
# add_test(
#     NAME test_day0
#     COMMAND test_day0
# )
