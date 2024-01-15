
if(NOT ROOT_RS_SYSTEM_ROOT)
    # check that the version is correct
    if( NOT ROOR_RS_ROOT_DOWNLOAD_VERSION STREQUAL "6.30.02" )
        # this should never happen
        message( FATAL_ERROR "selected ROOT download version wrong, please update this script" )
    endif()

    # get OS name and version
    # https://cmake.org/cmake/help/latest/command/cmake_host_system_information.html
    cmake_host_system_information(RESULT os_name QUERY OS_NAME)
    cmake_host_system_information(RESULT os_version QUERY OS_VERSION)
    message( STATUS "OS name: ${os_name}" )
    message( STATUS "OS version: ${os_version}" )

    # get target architecture
    # https://cmake.org/cmake/help/latest/variable/CMAKE_SYSTEM_PROCESSOR.html
    set(target_architecture ${CMAKE_SYSTEM_PROCESSOR})
    message( STATUS "target architecture: ${target_architecture}" )
    set(build_type ${CMAKE_BUILD_TYPE})
    message( STATUS "build type: ${build_type}" )

    # TODO written by copilot, check for correctness
    if( os_name STREQUAL "Almalinux" )
        if( os_version STREQUAL "8.8" )
            set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-almalinux8.8-x86_64-gcc8.5.tar.gz" )
        elseif( os_version STREQUAL "9.3" )
            set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-almalinux9.3-x86_64-gcc11.4.tar.gz" )
        else()
            message( FATAL_ERROR "unsupported Almalinux version" )
        endif()
    elseif( os_name STREQUAL "Centosstream" )
        if( os_version STREQUAL "8" )
            set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-centosstream8-x86_64-gcc8.5.tar.gz" )
        elseif( os_version STREQUAL "9" )
            set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-centosstream9-x86_64-gcc11.3.tar.gz" )
        else()
            message( FATAL_ERROR "unsupported Centosstream version" )
        endif()
    elseif( os_name STREQUAL "Fedora" )
        if( os_version STREQUAL "37" )
            set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-fedora37-x86_64-gcc12.3.tar.gz" )
        elseif( os_version STREQUAL "38" )
            set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-fedora38-x86_64-gcc13.2.tar.gz" )
        elseif( os_version STREQUAL "39" )
            set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-fedora39-x86_64-gcc13.2.tar.gz" )
        else()
            message( FATAL_ERROR "unsupported Fedora version" )
        endif()
    elseif( os_name STREQUAL "Ubuntu" )
        if( os_version STREQUAL "20.04" )
            set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-ubuntu20.04-x86_64-gcc9.4.tar.gz" )
        elseif( os_version STREQUAL "22.04" )
            set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-ubuntu22.04-x86_64-gcc11.4.tar.gz" )
        elseif( os_version STREQUAL "23.04" )
            set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-ubuntu23.04-x86_64-gcc12.3.tar.gz" )
        else()
            message( FATAL_ERROR "unsupported Ubuntu version" )
        endif()
    elseif( os_name STREQUAL "macOS" )
        if( os_version STREQUAL "12.7" )
            if( CMAKE_HOST_SYSTEM_PROCESSOR STREQUAL "arm64" )
                set( ROOT_DOWNLOAD_URL "root_v6.30.02.macos-12.7-arm64-clang140.tar.gz" )
            elseif( CMAKE_HOST_SYSTEM_PROCESSOR STREQUAL "x86_64" )
                set( ROOT_DOWNLOAD_URL "root_v6.30.02.macos-12.7-x86_64-clang140.tar.gz" )
            endif()
        elseif( os_version STREQUAL "13.6" )
            if( CMAKE_HOST_SYSTEM_PROCESSOR STREQUAL "arm64" )
                set( ROOT_DOWNLOAD_URL "root_v6.30.02.macos-13.6-arm64-clang150.tar.gz" )
            elseif( CMAKE_HOST_SYSTEM_PROCESSOR STREQUAL "x86_64" )
                set( ROOT_DOWNLOAD_URL "root_v6.30.02.macos-13.6-x86_64-clang150.tar.gz" )
            endif()
        elseif( os_version STREQUAL "14.1" )
            if( CMAKE_HOST_SYSTEM_PROCESSOR STREQUAL "arm64" )
                set( ROOT_DOWNLOAD_URL "root_v6.30.02.macos-14.1-arm64-clang150.tar.gz" )
            elseif( CMAKE_HOST_SYSTEM_PROCESSOR STREQUAL "x86_64" )
                set( ROOT_DOWNLOAD_URL "root_v6.30.02.macos-14.1-x86_64-clang150.tar.gz" )
            endif()
        else()
            message( FATAL_ERROR "unsupported macOS version" )
        endif()
    elseif( os_name STREQUAL "Windows" )
        if(target_architecture STREQUAL "AMD64")
            if( build_type STREQUAL "Debug" )
                set( ROOT_DOWNLOAD_URL "root_v6.30.02.win64.vc17.debug.zip" )
            else()
                set( ROOT_DOWNLOAD_URL "root_v6.30.02.win64.vc17.zip" )
            endif()
        else()
            message( FATAL_ERROR "unsupported Windows architecture" )
        endif()
    else()
        message( FATAL_ERROR "unsupported OS" )
    endif()

    set(ROOT_DOWNLOAD_URL "https://root.cern/download/${ROOT_DOWNLOAD_URL}")

    include(FetchContent)
    FetchContent_Declare(
    root_prebuilt_release
    URL     ${ROOT_DOWNLOAD_URL}
    )
    FetchContent_MakeAvailable(root_prebuilt_release) # this will download and extract the zip file TODO maybe instead use FetchContent_Populate?
    #list(APPEND CMAKE_PREFIX_PATH ${root_prebuilt_release_SOURCE_DIR}/cmake)

    find_package(ROOT 6.20 REQUIRED PATHS ${root_prebuilt_release_SOURCE_DIR}/cmake NO_DEFAULT_PATH)
else()
    find_package(ROOT 6.20 REQUIRED)
endif()