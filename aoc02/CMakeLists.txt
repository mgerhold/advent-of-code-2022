cmake_minimum_required(VERSION 3.24)
project(aoc02)

set(CMAKE_CXX_STANDARD 23)

add_executable(aoc02 main.cpp std.cppm utils.cppm expected.hpp magic_enum.hpp strong_type.hpp)

target_compile_options(aoc02 PUBLIC /experimental:module)
