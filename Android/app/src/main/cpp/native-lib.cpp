#include <jni.h>
#include <string>

extern "C" {
    const char* rust_hello();
}

extern "C"
JNIEXPORT jstring JNICALL
Java_com_example_monapp_MainActivity_rustHello(JNIEnv* env, jobject /* this */) {
    const char* msg = rust_hello();
    return env->NewStringUTF(msg);
}
