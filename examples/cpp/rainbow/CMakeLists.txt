cmake_minimum_required(VERSION 3.10)
project(example)

add_executable(${PROJECT_NAME} main.cpp)

include(ExternalProject)

set_directory_properties(PROPERTIES EP_PREFIX ${CMAKE_BINARY_DIR}/Rust)
set(RUST_PROJ_DIR "${CMAKE_SOURCE_DIR}/../../..")
set(RUST_OUT_DIR "${RUST_PROJ_DIR}/target/debug")

add_custom_target(navigator
    COMMAND cargo build
    COMMENT "C binds generated: ${RUST_OUT_DIR}"
)

add_dependencies(${PROJECT_NAME} navigator)

if(CMAKE_SYSTEM_NAME STREQUAL "Darwin")
    set(LIB_EXT "dylib")
elseif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    set(LIB_EXT "dll")
else()
    set(LIB_EXT "so")
endif()

target_link_libraries(
    ${PROJECT_NAME}
        "${RUST_OUT_DIR}/libnavigator.${LIB_EXT}"
)

set_target_properties(${PROJECT_NAME} PROPERTIES CXX_STANDARD 11 CXX_STANDARD_REQUIRED ON)
target_include_directories(${PROJECT_NAME} PRIVATE "${RUST_OUT_DIR}")
