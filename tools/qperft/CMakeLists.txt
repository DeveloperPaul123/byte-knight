cmake_minimum_required(VERSION 3.0)
set(CMAKE_C_STANDARD 90)

project(qperft)

add_executable(qperft perft.c)
target_compile_options(qperft PRIVATE -O3 -march=native -flto)
