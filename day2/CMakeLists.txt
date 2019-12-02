cmake_minimum_required(VERSION 3.10)
project("day2")

include_directories(day2)

SET(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g -Wall")

file(GLOB src
    "*.h"
    "*.c"
)

add_executable(day2 ${src})