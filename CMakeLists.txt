cmake_minimum_required(VERSION 3.8)
project(alembic_rs)

set(CMAKE_CXX_STANDARD 11)

find_package(Alembic CONFIG REQUIRED)

set(SOURCE_FILES
        alembic-sys/src/glue.cpp
        alembic-sys/src/glue.hpp)

add_executable(alembic_rs ${SOURCE_FILES})
target_link_libraries(alembic_rs PRIVATE Alembic::Alembic)
target_include_directories(alembic_rs PRIVATE "C:\\Users\\Alexandre\\Developpement\\vcpkg\\installed\\x64-windows\\include\\OpenEXR")