cmake_minimum_required(VERSION 3.15)
project(VRGamePlatform VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# ============================================
# Build Options
# ============================================

option(BUILD_SHARED_LIBS "Build shared libraries" ON)
option(BUILD_EXAMPLES "Build example applications" ON)
option(ENABLE_OPENXR "Enable OpenXR VR support" ON)
option(ENABLE_STEAMVR "Enable SteamVR support" OFF)

# ============================================
# Find Dependencies
# ============================================

# OpenGL
find_package(OpenGL REQUIRED)

# GLFW for window management
find_package(glfw3 REQUIRED)

# Vulkan (optional, for better performance)
find_package(Vulkan)

# OpenXR (VR support)
if(ENABLE_OPENXR)
    find_package(OpenXR)
    if(OpenXR_FOUND)
        add_definitions(-DUSE_OPENXR)
    endif()
endif()

# Bullet Physics (optional)
find_package(Bullet)

# OpenAL for 3D audio
find_package(OpenAL)

# Lua for scripting
find_package(Lua)

# ============================================
# Source Files
# ============================================

set(VR_ENGINE_SOURCES
    src/VREngine.cpp
    src/VREngine.h
)

# ============================================
# VR Engine Library
# ============================================

add_library(VREngine ${VR_ENGINE_SOURCES})

target_include_directories(VREngine PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/src
    ${OPENGL_INCLUDE_DIR}
)

target_link_libraries(VREngine PUBLIC
    ${OPENGL_LIBRARIES}
    glfw
)

# Link optional libraries
if(Vulkan_FOUND)
    target_link_libraries(VREngine PUBLIC Vulkan::Vulkan)
    target_compile_definitions(VREngine PUBLIC USE_VULKAN)
endif()

if(OpenXR_FOUND)
    target_link_libraries(VREngine PUBLIC OpenXR::openxr_loader)
endif()

if(BULLET_FOUND)
    target_link_libraries(VREngine PUBLIC ${BULLET_LIBRARIES})
    target_include_directories(VREngine PUBLIC ${BULLET_INCLUDE_DIRS})
    target_compile_definitions(VREngine PUBLIC USE_BULLET_PHYSICS)
endif()

if(OPENAL_FOUND)
    target_link_libraries(VREngine PUBLIC ${OPENAL_LIBRARY})
    target_include_directories(VREngine PUBLIC ${OPENAL_INCLUDE_DIR})
    target_compile_definitions(VREngine PUBLIC USE_OPENAL)
endif()

if(LUA_FOUND)
    target_link_libraries(VREngine PUBLIC ${LUA_LIBRARIES})
    target_include_directories(VREngine PUBLIC ${LUA_INCLUDE_DIR})
    target_compile_definitions(VREngine PUBLIC USE_LUA_SCRIPTING)
endif()

# ============================================
# Main Application
# ============================================

add_executable(VRPlatformApp
    src/main.cpp
)

target_link_libraries(VRPlatformApp PRIVATE VREngine)

# ============================================
# Examples
# ============================================

if(BUILD_EXAMPLES)
    add_subdirectory(examples)
endif()

# ============================================
# Installation
# ============================================

install(TARGETS VREngine VRPlatformApp
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    RUNTIME DESTINATION bin
)

install(FILES src/VREngine.h
    DESTINATION include/VRPlatform
)

# ============================================
# Testing
# ============================================

enable_testing()

add_executable(VREngineTests
    tests/test_main.cpp
)

target_link_libraries(VREngineTests PRIVATE VREngine)

add_test(NAME VREngineTests COMMAND VREngineTests)

# ============================================
# Documentation
# ============================================

find_package(Doxygen)
if(DOXYGEN_FOUND)
    set(DOXYGEN_OUTPUT_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/docs)
    set(DOXYGEN_GENERATE_HTML YES)
    set(DOXYGEN_GENERATE_MAN NO)
    
    doxygen_add_docs(docs
        ${CMAKE_CURRENT_SOURCE_DIR}/src
        COMMENT "Generate API documentation"
    )
endif()

# ============================================
# Print Configuration Summary
# ============================================

message(STATUS "")
message(STATUS "====================================")
message(STATUS "VR Game Platform - Build Configuration")
message(STATUS "====================================")
message(STATUS "C++ Standard: ${CMAKE_CXX_STANDARD}")
message(STATUS "Build Type: ${CMAKE_BUILD_TYPE}")
message(STATUS "Build Shared Libs: ${BUILD_SHARED_LIBS}")
message(STATUS "Build Examples: ${BUILD_EXAMPLES}")
message(STATUS "")
message(STATUS "Dependencies:")
message(STATUS "  OpenGL: ${OPENGL_FOUND}")
message(STATUS "  Vulkan: ${Vulkan_FOUND}")
message(STATUS "  OpenXR: ${OpenXR_FOUND}")
message(STATUS "  Bullet Physics: ${BULLET_FOUND}")
message(STATUS "  OpenAL: ${OPENAL_FOUND}")
message(STATUS "  Lua: ${LUA_FOUND}")
message(STATUS "====================================")
message(STATUS "")
