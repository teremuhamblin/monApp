use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_example_monapp_rust_RustBridge_getMessage(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let msg = "Message Rust depuis le SDK monApp";
    env.new_string(msg).unwrap().into_raw()
}
