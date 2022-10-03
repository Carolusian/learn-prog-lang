# CMakeLists.txt
cmake_minimum_required(VERSION 3.0)
project(webserver)

find_package(fmt CONFIG REQUIRED)

add_executable(main src/main.cc)

target_link_libraries(main PRIVATE fmt::fmt)
