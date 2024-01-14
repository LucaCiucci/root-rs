
cmake_minimum_required(VERSION 3.10)


project(root-rs-c-bindings LANGUAGES CXX)

add_library(
    root-rs-c-bindings
    include/root-rs-c-bindings.h
    #src/TObject.cpp
    #src/TApplication.cpp
    #src/TPad.cpp
    #src/TCanvas.cpp
)

# add any file in the src directory
file(GLOB_RECURSE SOURCES src/*.cpp)
target_sources(root-rs-c-bindings PRIVATE ${SOURCES})

target_include_directories(root-rs-c-bindings PUBLIC include)

# set c++ standard
set_target_properties(root-rs-c-bindings PROPERTIES
    CXX_STANDARD 17
    CXX_STANDARD_REQUIRED YES
    CXX_EXTENSIONS NO
)

install(TARGETS root-rs-c-bindings DESTINATION lib)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/libs.txt DESTINATION lib)

message("CMAKE_CURRENT_SOURCE_DIR: ${CMAKE_CURRENT_SOURCE_DIR}")

find_package(ROOT 6.30 REQUIRED)
message("ROOT_FOUND: ${ROOT_FOUND}")
target_include_directories(root-rs-c-bindings PRIVATE ${ROOT_INCLUDE_DIRS})
message(${ROOT_INCLUDE_DIRS})
foreach(lib IN LISTS ROOT_LIBRARIES)
	message("lib: ${lib}")
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
file(WRITE ${CMAKE_CURRENT_BINARY_DIR}/libs.txt ${LIBS_LINES})


# root version as a doc string, example "630"
#set(ROOT_VERSION ${ROOT_VERSION_MAJOR}${ROOT_VERSION_MINOR}${ROOT_VERSION_PATCH})
set(RRS_ROOT_VERSION ${ROOT_VERSION_MAJOR}${ROOT_VERSION_MINOR})
message("ROOT_VERSION: ${ROOT_VERSION}")

configure_file(config.in.h ${CMAKE_CURRENT_BINARY_DIR}/root-rs-c-bindings/config.h)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/root-rs-c-bindings/config.h DESTINATION include/root-rs-c-bindings)
target_include_directories(root-rs-c-bindings PRIVATE ${CMAKE_CURRENT_BINARY_DIR})