cmake_minimum_required(VERSION 3.12)
project(demo LANGUAGES C)

# 设置 C23 标准
set(CMAKE_C_STANDARD 23)
set(CMAKE_C_STANDARD_REQUIRED ON)

# 查找 LAPACK 库
find_package(LAPACK REQUIRED)

# 设置 Homebrew 安装的 LAPACK 头文件路径
set(LAPACK_INCLUDE_DIR "/opt/homebrew/opt/lapack/include")

# 自动包含头文件目录
file(GLOB SOURCES "c-src/*.c")

# 创建共享库
add_library(demo SHARED ${SOURCES})

# 包含目录设置
target_include_directories(demo PUBLIC 
    ${CMAKE_SOURCE_DIR}/include
    ${LAPACK_INCLUDE_DIR}
)

# 链接 LAPACK 库
target_link_libraries(demo PUBLIC 
    ${LAPACK_LIBRARIES}
)

# 设置输出目录
set_target_properties(demo PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/lib
) 
