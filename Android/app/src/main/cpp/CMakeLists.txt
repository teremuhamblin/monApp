add_library(monapp_rust_android SHARED IMPORTED)
set_target_properties(monapp_rust_android PROPERTIES IMPORTED_LOCATION
    ${CMAKE_SOURCE_DIR}/../jniLibs/${ANDROID_ABI}/libmonapp_rust_android.so)

add_library(native-lib SHARED native-lib.cpp)

target_link_libraries(native-lib monapp_rust_android)
