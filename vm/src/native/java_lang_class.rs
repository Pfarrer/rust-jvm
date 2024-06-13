use log::trace;
use model::prelude::*;

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "registerNatives" => Some(register_natives),
        _ => None,
    }
}

fn register_natives() {
    trace!("Execute native java/lang/Class.registerNatives()V");
}
