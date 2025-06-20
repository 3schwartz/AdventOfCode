cmake_minimum_required(VERSION 3.31)
project(cpp)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(day1 day1/day1.cpp)

add_executable(day2 day2/day2.cpp)

add_executable(day11
        day11/day11.cpp
        day11/day11_logic.cpp
        day11/day11_bfs.cpp
)

# Unit tests
include(FetchContent)
FetchContent_Declare(
        googletest
        URL https://github.com/google/googletest/archive/refs/heads/main.zip
)
FetchContent_MakeAvailable(googletest)

enable_testing()

add_executable(day11_test
        day11/day11_logic.cpp
        day11/day11_test.cpp
        day11/day11_bfs.cpp
)

target_link_libraries(
        day11_test
        GTest::gtest_main
)

include(GoogleTest)
gtest_discover_tests(day11_test)