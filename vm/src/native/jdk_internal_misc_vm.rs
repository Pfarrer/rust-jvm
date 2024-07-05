use log::trace;
use model::prelude::*;

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "initialize" => Some(initialize), // ()V
        _ => None,
    }
}

/// ()V
fn initialize(_: &mut VmThread) {
    trace!("Execute native jdk/internal/misc/VM.initialize()V");
}
