
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

    set(RRS_NEEDS_VDT OFF)

    # TODO written by copilot, check for correctness
    if( os_version MATCHES "Almalinux" )
        #if( os_version MATCHES "8.8" )
        #    set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-almalinux8.8-x86_64-gcc8.5.tar.gz" )
        #elseif( os_version MATCHES "9.3" )
        #    set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-almalinux9.3-x86_64-gcc11.4.tar.gz" )
        #else()
        #    message( FATAL_ERROR "unsupported Almalinux version" )
        #endif()
        message( FATAL_ERROR "Almalinux support not yet implemented, see https://github.com/LucaCiucci/root-rs/issues/1" )
    elseif( os_version MATCHES "Centosstream" )
        #if( os_version STREQUAL "8" )
        #    set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-centosstream8-x86_64-gcc8.5.tar.gz" )
        #elseif( os_version STREQUAL "9" )
        #    set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-centosstream9-x86_64-gcc11.3.tar.gz" )
        #else()
        #    message( FATAL_ERROR "unsupported Centosstream version" )
        #endif()
        message( FATAL_ERROR "Centosstream support not yet implemented, see https://github.com/LucaCiucci/root-rs/issues/2" )
    elseif( os_version MATCHES "Fedora" )
        #if( os_version STREQUAL "37" )
        #    set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-fedora37-x86_64-gcc12.3.tar.gz" )
        #elseif( os_version STREQUAL "38" )
        #    set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-fedora38-x86_64-gcc13.2.tar.gz" )
        #elseif( os_version STREQUAL "39" )
        #    set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-fedora39-x86_64-gcc13.2.tar.gz" )
        #else()
        #    message( FATAL_ERROR "unsupported Fedora version" )
        #endif()
        message( FATAL_ERROR "Fedora support not yet implemented, see https://github.com/LucaCiucci/root-rs/issues/3" )
    elseif( os_version MATCHES "Ubuntu" )
        if( os_version MATCHES "20.04" )
            set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-ubuntu20.04-x86_64-gcc9.4.tar.gz" )
        elseif( os_version MATCHES "22.04" )
            set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-ubuntu22.04-x86_64-gcc11.4.tar.gz" )
        elseif( os_version MATCHES "23.04" )
            set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-ubuntu23.04-x86_64-gcc12.3.tar.gz" )
        else()
            message( WARNING "unsupported Ubuntu version, using root_v6.30.02.Linux-ubuntu23.04-x86_64-gcc12.3.tar.gz" )
            set( ROOT_DOWNLOAD_URL "root_v6.30.02.Linux-ubuntu23.04-x86_64-gcc12.3.tar.gz" )
        endif()
        set(RRS_NEEDS_VDT ON)
    elseif( os_name STREQUAL "macOS" )
        #if( os_version MATCHES "12.7" )
        #    if( CMAKE_HOST_SYSTEM_PROCESSOR STREQUAL "arm64" )
        #        set( ROOT_DOWNLOAD_URL "root_v6.30.02.macos-12.7-arm64-clang140.tar.gz" )
        #    elseif( CMAKE_HOST_SYSTEM_PROCESSOR STREQUAL "x86_64" )
        #        set( ROOT_DOWNLOAD_URL "root_v6.30.02.macos-12.7-x86_64-clang140.tar.gz" )
        #    endif()
        #elseif( os_version MATCHES "13.6" )
        #    if( CMAKE_HOST_SYSTEM_PROCESSOR STREQUAL "arm64" )
        #        set( ROOT_DOWNLOAD_URL "root_v6.30.02.macos-13.6-arm64-clang150.tar.gz" )
        #    elseif( CMAKE_HOST_SYSTEM_PROCESSOR STREQUAL "x86_64" )
        #        set( ROOT_DOWNLOAD_URL "root_v6.30.02.macos-13.6-x86_64-clang150.tar.gz" )
        #    endif()
        #elseif( os_version MATCHES "14.1" )
        #    if( CMAKE_HOST_SYSTEM_PROCESSOR STREQUAL "arm64" )
        #        set( ROOT_DOWNLOAD_URL "root_v6.30.02.macos-14.1-arm64-clang150.tar.gz" )
        #    elseif( CMAKE_HOST_SYSTEM_PROCESSOR STREQUAL "x86_64" )
        #        set( ROOT_DOWNLOAD_URL "root_v6.30.02.macos-14.1-x86_64-clang150.tar.gz" )
        #    endif()
        #else()
        #    message( FATAL_ERROR "unsupported macOS version" )
        #endif()
        message( FATAL_ERROR "macOS support not yet implemented, see https://github.com/LucaCiucci/root-rs/issues/4" )
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
        nlohmann_json
        GIT_REPOSITORY https://github.com/nlohmann/json
        GIT_TAG v3.11.3
        OVERRIDE_FIND_PACKAGE
    )
    FetchContent_MakeAvailable(nlohmann_json)
    if(RRS_NEEDS_VDT)
        list(APPEND CMAKE_PREFIX_PATH ${CMAKE_INSTALL_PREFIX})

        # check if vdt is already installed
        find_package(vdt QUIET)
        if(vdt_FOUND)
            message(STATUS "vdt found")
        else()
            message(STATUS "vdt not found")
        endif()

        if (NOT vdt_FOUND)
            message(STATUS "vdt not found, downloading and installing it")
            FetchContent_Declare(
                vdt
                GIT_REPOSITORY https://github.com/dpiparo/vdt
                GIT_TAG v0.4.4
                #OVERRIDE_FIND_PACKAGE
            )
            #FetchContent_MakeAvailable(vdt)
            FetchContent_GetProperties(vdt)
            if(NOT vdt_POPULATED)
                FetchContent_Populate(vdt)
            endif()
            # Now install into ${CMAKE_INSTALL_PREFIX}:
            # git clone https://github.com/dpiparo/vdt.git
            # cd vdt
            # cmake -DCMAKE_INSTALL_PREFIX=$INSTALLDIR .
            # make
            # make install
            set(command "cd ${vdt_SOURCE_DIR}; ${CMAKE_COMMAND} -DCMAKE_INSTALL_PREFIX=${CMAKE_INSTALL_PREFIX} .; make; make install")
            message(STATUS "executing command: ${command}")
            exec_program( "cd ${vdt_SOURCE_DIR}; ${CMAKE_COMMAND} -DCMAKE_INSTALL_PREFIX=${CMAKE_INSTALL_PREFIX} .; make; make install" )
            # add CMAKE_INSTALL_PREFIX to CMAKE_PREFIX_PATH
        endif()
    endif()

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

#set(tmp "#18~22.04.1-Ubuntu SMP Tue Nov 21 19:25:02 UTC 2023")
##if(tmp MATCHES "22.04")
#if(  tmp MATCHES "22.04"  )
#    message(FATAL_ERROR "Ubuntu 22.04 detected")
#else()
#    message(FATAL_ERROR "Ubuntu 22.04 not detected")
#endif()