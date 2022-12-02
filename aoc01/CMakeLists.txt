cmake_minimum_required(VERSION 3.24)
project(aoc01)

set(CMAKE_CXX_STANDARD 23)

add_executable(aoc01 main.cpp std.cppm utils.cppm)

target_compile_options(aoc01 PUBLIC /experimental:module)
