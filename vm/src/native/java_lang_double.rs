use model::prelude::*;

use crate::frame::VmFrameImpl;

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "doubleToRawLongBits" => Some(double_to_raw_long_bits), // (D)J
        _ => None,
    }
}

/// (D)J
fn double_to_raw_long_bits(vm_thread: &mut VmThread) {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let l_value = unsafe {
        let d_value = frame.stack_pop_double();
        std::mem::transmute::<f64, i64>(d_value)
    };
    frame.stack_push(VmPrimitive::Long(l_value));
}

