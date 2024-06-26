cmake_minimum_required(VERSION 3.11)
project(perf_prog_npb_bt)

set_property(GLOBAL PROPERTY C_STANDARD 11)

if(MSVC)
  add_compile_options(/W4)
else()
  add_compile_options(-Wall -Wextra -Wno-unknown-pragmas -Wno-unused-parameter)
endif()

file(GLOB_RECURSE SOURCE_FILES CONFIGURE_DEPENDS src/*.c src/*.h)

link_libraries(m)

set(SIZES w s a b c)
set(OPTIMIZATION_LEVELS 0 1 2 3 s fast)

foreach(OPT_LEVEL ${OPTIMIZATION_LEVELS})
  foreach(SIZE ${SIZES})
    set(CMAKE_CXX_FLAGS "-O${OPT_LEVEL}")
    set(CMAKE_CXX_FLAGS "-O${OPT_LEVEL}")
    set(P npb_bt_${SIZE}_O${OPT_LEVEL})

    add_executable(${P} ${SOURCE_FILES})
    target_include_directories(${P} PRIVATE src/w)
    target_compile_options(${P} PRIVATE -O${OPT_LEVEL})
  endforeach()
endforeach()

