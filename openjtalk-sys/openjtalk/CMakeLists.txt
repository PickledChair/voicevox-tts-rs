cmake_minimum_required(VERSION 3.16)

project(OpenJTalkUtil)

set(CMAKE_MACOSX_RPATH 1)

add_subdirectory(open_jtalk/src)

add_library(openjtalkutil
	src/openjtalk.cpp)
set_property(TARGET openjtalkutil PROPERTY CXX_STANDARD 17)
set_property(TARGET openjtalkutil PROPERTY POSITION_INDEPENDENT_CODE ON) # fPIC

target_compile_options(openjtalkutil PRIVATE
	$<$<CXX_COMPILER_ID:MSVC>: /W4 /O2 /utf-8>
	$<$<CXX_COMPILER_ID:GNU>: -Wall -Wextra -O2>
)
target_include_directories(openjtalkutil
	PRIVATE open_jtalk/src/jpcommon
	PRIVATE open_jtalk/src/mecab/src
	PRIVATE open_jtalk/src/mecab2njd
	PRIVATE open_jtalk/src/mecab-naist-jdic
	PRIVATE open_jtalk/src/njd
	PRIVATE open_jtalk/src/njd2jpcommon
	PRIVATE open_jtalk/src/njd_set_accent_phrase
	PRIVATE open_jtalk/src/njd_set_accent_type
	PRIVATE open_jtalk/src/njd_set_digit
	PRIVATE open_jtalk/src/njd_set_long_vowel
	PRIVATE open_jtalk/src/njd_set_pronunciation
	PRIVATE open_jtalk/src/njd_set_unvoiced_vowel
	PRIVATE open_jtalk/src/text2mecab)
target_link_libraries(openjtalkutil PUBLIC openjtalk)

# cmake --installを行うとcoreライブラリ、onnxruntimeライブラリ、core.hがインストール先のlibフォルダにコピーされる
install(TARGETS openjtalkutil
	ARCHIVE DESTINATION lib
	LIBRARY DESTINATION lib
	RUNTIME DESTINATION lib)
install(FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/openjtalk.hpp
	DESTINATION ${CMAKE_INSTALL_PREFIX}/lib)
