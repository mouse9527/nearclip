cmake_minimum_required(VERSION 3.18.1)

project("nearclip_jni")

# 设置C标准
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

# 查找Android日志库
find_library(log-lib log)

# 查找JNI库
find_library(jni-lib jni)

# 包含Rust生成的头文件目录
include_directories(${CMAKE_SOURCE_DIR}/src/main/cpp/include)

# 添加Rust静态库
add_library(nearclip_core STATIC IMPORTED)
set_target_properties(nearclip_core PROPERTIES
    IMPORTED_LOCATION ${CMAKE_SOURCE_DIR}/src/main/jniLibs/${ANDROID_ABI}/libnearclip_core.a)

# 创建JNI包装库
add_library(
    nearclip_jni
    SHARED
    src/main/cpp/nearclip_jni.cpp
    src/main/cpp/jni_callbacks.cpp
    src/main/cpp/device_manager.cpp
    src/main/cpp/clipboard_manager.cpp
)

# 链接库
target_link_libraries(
    nearclip_jni
    nearclip_core
    ${log-lib}
    ${jni-lib}
)

# 设置编译选项
target_compile_options(nearclip_jni PRIVATE
    -Wall
    -Wextra
    -O2
    -fno-omit-frame-pointer
)

# 设置预处理器宏
target_compile_definitions(nearclip_jni PRIVATE
    ANDROID_PLATFORM=android-${ANDROID_PLATFORM_LEVEL}
)

# 设置输出目录
set_target_properties(nearclip_jni PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/build/intermediates/cmake/debug/obj/${ANDROID_ABI}
)