use model::prelude::*;

use vm::frame::VmFrameImpl;

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "isBigEndian" => Some(is_big_endian), // ()Z

        _ => None,
    }
}

/// ()Z
fn is_big_endian(vm_thread: &mut VmThread) {
    // Remove parameter from stack
    let frame = vm_thread.frame_stack.last_mut().unwrap();

    let primitive = VmPrimitive::Boolean(cfg!(target_endian = "big"));
    frame.stack_push(primitive);
}
