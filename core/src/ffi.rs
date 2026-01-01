use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jint, jlong, jstring};
use std::ffi::{CStr, CString};
use std::sync::Mutex;
use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use crate::network::service::{NetworkService, NetworkCommand};

lazy_static! {
    static ref RUNTIME: Mutex<Option<Runtime>> = Mutex::new(None);
}

// این تابع دقیقا همان چیزی است که جاوا صدا می‌زند
// پکیج: com.aeon.mobile
// کلاس: MainActivity
// تابع: startNode
#[no_mangle]
pub extern "system" fn Java_com_aeon_mobile_MainActivity_startNode(
    _env: JNIEnv,
    _class: JClass,
    seed: jlong,
) -> jint {
    let mut rt_guard = RUNTIME.lock().unwrap();
    if rt_guard.is_some() {
        return 0;
    }

    let rt = match Runtime::new() {
        Ok(r) => r,
        Err(_) => return -1,
    };

    rt.spawn(async move {
        // تبدیل seed به فرمت Rust
        if let Ok((mut service, _cmd)) = NetworkService::new(seed as u64).await {
            let addr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
            let _ = service.listen(addr);
            service.run().await;
        }
    });

    *rt_guard = Some(rt);
    0
}

#[no_mangle]
pub extern "system" fn Java_com_aeon_mobile_MainActivity_getStatus(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let output = "Aeon Node Running (JNI Mode)";
    env.new_string(output).expect("Couldn't create java string!").into_raw()
}
