use model::prelude::*;

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "initIDs" => Some(init_ids), // ()V
        _ => None,
    }
}

/// ()V
fn init_ids(_: &mut VmThread) {
}