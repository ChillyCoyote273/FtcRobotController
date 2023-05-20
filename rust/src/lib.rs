mod jni_interface;
mod utils;

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jint;

#[no_mangle]
pub extern "system" fn Java_org_firstinspires_ftc_teamcode_RustTest_multiply<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
    a: jint,
    b: jint
) -> jint {
    a * b
}
