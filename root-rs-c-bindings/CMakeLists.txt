
cmake_minimum_required(VERSION 3.10)

project(root-rs-c-bindings LANGUAGES CXX)

add_library(root-rs-c-bindings)

# add all source files
file(GLOB_RECURSE SOURCES src/*.cpp include/*.h)
target_sources(root-rs-c-bindings PRIVATE ${SOURCES})

target_include_directories(
    root-rs-c-bindings
    PUBLIC
    $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/include> # TODO check
    $<INSTALL_INTERFACE:${CMAKE_INSTALL_PREFIX}/include/root-rs-c-bindings> # TODO check
)

set_target_properties(root-rs-c-bindings PROPERTIES
    CXX_STANDARD 17
    CXX_STANDARD_REQUIRED YES
    CXX_EXTENSIONS NO
)

# TODO ADJUST THIS VALUE!!!!! I use 6.30.02, but I wrote 6.20 to make it work for others.
find_package(ROOT 6.20 REQUIRED)
message(STATUS "ROOT_FOUND: ${ROOT_FOUND}")
message(STATUS "root version: ${ROOT_VERSION}")
target_include_directories(root-rs-c-bindings PRIVATE ${ROOT_INCLUDE_DIRS})
message(STATUS "ROOT_INCLUDE_DIRS: ${ROOT_INCLUDE_DIRS}")
foreach(lib IN LISTS ROOT_LIBRARIES)
    #message(STATUS "linked lib: ${lib}")
    target_link_libraries(root-rs-c-bindings PRIVATE ${lib})
endforeach()

# list all linked libraries
get_target_property(LIBS root-rs-c-bindings LINK_LIBRARIES)
message("LIBS: ${LIBS}")
# write them to a file, one per line
set(LIBS_LINES "")
foreach(lib IN LISTS LIBS)
    set(LIBS_LINES "${LIBS_LINES}${lib}\n")
endforeach()
file(WRITE ${CMAKE_CURRENT_BINARY_DIR}/root-rs-c-bindings-linked-libs.txt ${LIBS_LINES})

# root version as a doc string, example "630"
#set(ROOT_VERSION ${ROOT_VERSION_MAJOR}${ROOT_VERSION_MINOR}${ROOT_VERSION_PATCH})
set(RRS_ROOT_DOC_VERSION ${ROOT_VERSION_MAJOR}${ROOT_VERSION_MINOR})

install(TARGETS root-rs-c-bindings DESTINATION lib)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/root-rs-c-bindings-linked-libs.txt DESTINATION lib)
install(DIRECTORY include/ DESTINATION include/root-rs-c-bindings)

configure_file(config.in.h ${CMAKE_CURRENT_BINARY_DIR}/root-rs-c-bindings/config.h)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/root-rs-c-bindings/config.h DESTINATION include/root-rs-c-bindings/root-rs-c-bindings)
target_include_directories(root-rs-c-bindings PRIVATE ${CMAKE_CURRENT_BINARY_DIR})