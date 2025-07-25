cmake_minimum_required(VERSION 3.31)
project(cpp)

set(OPENSSL_ROOT_DIR "/opt/homebrew/opt/openssl@3")

find_package(OpenSSL REQUIRED)

# Set C++ standard globally
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# ----------------------------------------------------------------------------
# Executables
# ----------------------------------------------------------------------------

add_executable(day1 day1/day1.cpp)
add_executable(day2 day2/day2.cpp)
add_executable(day13 day13/day13.cpp)

add_executable(day11
        day11/day11.cpp
        day11/day11_logic.cpp
        day11/day11_bfs.cpp
        day11/day11_v2.cpp
        common/common.cpp
)

add_executable(day12
        day12/day12.cpp
        common/common.cpp
)

add_executable(day14
        day14/day14_exec.cpp
        day14/day14.cpp
)
target_include_directories(day14 PRIVATE ${OPENSSL_INCLUDE_DIR})
target_link_libraries(day14 PRIVATE OpenSSL::Crypto)

add_executable(day15
        day15/day15.cpp
        common/common.cpp
)

add_executable(day16
        day16/day16.cpp
        common/common.cpp
)

add_executable(day17
        day17/day17_exec.cpp
        day17/day17.cpp
        common/common.cpp
)
target_include_directories(day17 PRIVATE ${OPENSSL_INCLUDE_DIR})
target_link_libraries(day17 PRIVATE OpenSSL::Crypto)

add_executable(day18
        day18/day18.cpp
        common/common.cpp
)

add_executable(day19
        day19/day19_exec.cpp
        day19/day19.cpp
        common/common.cpp
)

add_executable(day20
        day20/day20.cpp
        common/common.cpp
)

add_executable(day21
        day21/day21_exec.cpp
        day21/day21.cpp
        common/common.cpp
)

add_executable(day22
        day22/day22.cpp
        common/common.cpp
)

add_executable(day23 day23/day23.cpp)

add_executable(day24
        day24/day24.cpp
        common/common.cpp
)

add_executable(day25
        day25/day25.cpp
        common/common.cpp
)

# ----------------------------------------------------------------------------
# GoogleTest Setup
# ----------------------------------------------------------------------------

include(FetchContent)
FetchContent_Declare(
        googletest
        URL https://github.com/google/googletest/archive/refs/heads/main.zip
)
FetchContent_MakeAvailable(googletest)

enable_testing()

# ----------------------------------------------------------------------------
# Test Executables
# ----------------------------------------------------------------------------

add_executable(day11_test
        day11/day11_logic.cpp
        day11/day11_test.cpp
        day11/day11_bfs.cpp
)

add_executable(day14_test
        day14/day14_test.cpp
        day14/day14.cpp
)
target_include_directories(day14_test PRIVATE ${OPENSSL_INCLUDE_DIR})

add_executable(day17_test
        day17/day17_test.cpp
        day17/day17.cpp
)
target_include_directories(day17_test PRIVATE ${OPENSSL_INCLUDE_DIR})

add_executable(day19_test
        day19/day19_test.cpp
        day19/day19.cpp
)

add_executable(day21_test
        day21/day21_test.cpp
        day21/day21.cpp
)
# Link GoogleTest to test targets
target_link_libraries(day11_test GTest::gtest_main)
target_link_libraries(day14_test PRIVATE GTest::gtest_main OpenSSL::Crypto)
target_link_libraries(day17_test PRIVATE GTest::gtest_main OpenSSL::Crypto)
target_link_libraries(day19_test GTest::gtest_main)
target_link_libraries(day21_test GTest::gtest_main)

include(GoogleTest)
gtest_discover_tests(day11_test)
gtest_discover_tests(day14_test)
gtest_discover_tests(day17_test)
gtest_discover_tests(day19_test)
gtest_discover_tests(day21_test)