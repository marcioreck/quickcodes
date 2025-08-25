cmake_minimum_required(VERSION 3.12)
project(quickcodes_cpp_test)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add the include directory
include_directories(include)

# Find the QuickCodes library
find_library(QUICKCODES_LIB quickcodes PATHS /usr/local/lib)

# Create the test executable
add_executable(test_quickcodes test/test.cpp)

# Link against the QuickCodes library
target_link_libraries(test_quickcodes ${QUICKCODES_LIB})
