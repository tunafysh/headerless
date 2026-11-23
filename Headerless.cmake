# Headerless.cmake

set(HEADERLESS_VERSION "1.0.0")
set(HEADERLESS_DIR "${CMAKE_BINARY_DIR}/headerless")

if(WIN32)
    set(HEADERLESS_PLATFORM "windows-x86_64")
    set(HEADERLESS_EXECUTABLE "headerless.exe")
elseif(APPLE)
    set(HEADERLESS_PLATFORM "macos-aarch64")
    set(HEADERLESS_EXECUTABLE "headerless")
elseif(UNIX)
    set(HEADERLESS_PLATFORM "linux-x86_64")
    set(HEADERLESS_EXECUTABLE "headerless")
endif()

set(HEADERLESS_BINARY "${HEADERLESS_DIR}/${HEADERLESS_EXECUTABLE}")

if(NOT EXISTS "${HEADERLESS_BINARY}")
    message(STATUS "Downloading Headerless ${HEADERLESS_VERSION}...")

    file(MAKE_DIRECTORY "${HEADERLESS_DIR}")

    set(HEADERLESS_ARCHIVE
        "${HEADERLESS_DIR}/headerless.tar.gz"
    )

    set(HEADERLESS_URL
        "https://github.com/tunafysh/headerless/releases/latest/download/headerless-${HEADERLESS_PLATFORM}.tar.gz"
    )

    file(DOWNLOAD 
        "${HEADERLESS_URL}"
        "${HEADERLESS_ARCHIVE}"
        STATUS DOWNLOAD_STATUS
        SHOW_PROGRESS
    )

    list(GET DOWNLOAD_STATUS 0 DOWNLOAD_CODE)

    list(GET DOWNLOAD_STATUS 1 DOWNLOAD_MESSAGE)

    if(NOT DOWNLOAD_CODE EQUAL 0)
        message(FATAL_ERROR
            "Failed to download Headerless: ${DOWNLOAD_MESSAGE}"
        )
    endif()

    file(ARCHIVE_EXTRACT INPUT "${HEADERLESS_ARCHIVE}" DESTINATION "${HEADERLESS_DIR}")

    file(REMOVE "${HEADERLESS_ARCHIVE}")

    if(UNIX)
        file(CHMOD "${HEADERLESS_BINARY}"
            PERMISSIONS
                OWNER_READ OWNER_WRITE OWNER_EXECUTE
                GROUP_READ GROUP_EXECUTE
                WORLD_READ WORLD_EXECUTE
        )
    endif()
endif()

message(STATUS "Using Headerless: ${HEADERLESS_BINARY}")



function(headerless_generate)
    cmake_parse_arguments(
        HL
        ""
        "TARGET"
        "SOURCES"
        ${ARGN}
    )

    if(NOT HL_TARGET)
        message(FATAL_ERROR "headerless_generate requires TARGET")
    endif()

    set(GENERATED_DIR "${CMAKE_CURRENT_BINARY_DIR}/generated")

    set(GENERATED_SOURCES "")
    set(GENERATED_HEADERS "")
    list(GET HL_SOURCES 0 FIRST_SOURCE)
    get_filename_component(SOURCE_ROOT "${FIRST_SOURCE}" DIRECTORY)

    foreach(SOURCE IN LISTS HL_SOURCES)
        get_filename_component(SOURCE_DIR "${SOURCE}" DIRECTORY)
        get_filename_component(SOURCE_DIR "${SOURCE_DIR}" ABSOLUTE)
        get_filename_component(SOURCE_ROOT "${SOURCE_ROOT}" ABSOLUTE)

        while(SOURCE_DIR)
            string(FIND "${SOURCE_DIR}/" "${SOURCE_ROOT}/" SOURCE_ROOT_INDEX)
            if(SOURCE_ROOT_INDEX EQUAL 0)
                break()
            endif()

            get_filename_component(SOURCE_ROOT "${SOURCE_ROOT}" DIRECTORY)
        endwhile()
    endforeach()

    foreach(SOURCE IN LISTS HL_SOURCES)
        file(RELATIVE_PATH SOURCE_REL "${SOURCE_ROOT}" "${SOURCE}")
        string(REGEX REPLACE "\\.[^.]+$" ".h" HEADER_REL "${SOURCE_REL}")

        list(APPEND GENERATED_SOURCES
            "${GENERATED_DIR}/${SOURCE_REL}"
        )

        list(APPEND GENERATED_HEADERS
            "${GENERATED_DIR}/${HEADER_REL}"
        )
    endforeach()

    set(GENERATED_OUTPUTS
        ${GENERATED_SOURCES}
        ${GENERATED_HEADERS}
    )

    add_custom_command(
        OUTPUT ${GENERATED_OUTPUTS}

        COMMAND "${CMAKE_COMMAND}" -E make_directory
            "${GENERATED_DIR}"

        COMMAND "${HEADERLESS_BINARY}"
            --output "${GENERATED_DIR}"
            ${HL_SOURCES}

        DEPENDS
            ${HL_SOURCES}
            "${HEADERLESS_BINARY}"

        VERBATIM
        COMMENT "Generating HeaderLess sources for ${HL_TARGET}"
    )

    target_sources(${HL_TARGET} PRIVATE
        ${GENERATED_SOURCES}
        ${GENERATED_HEADERS}
    )

    target_include_directories(${HL_TARGET}
        PUBLIC "${GENERATED_DIR}"
    )
endfunction()