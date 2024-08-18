use model::prelude::*;

use vm::frame::VmFrameImpl;

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "doubleToRawLongBits" => Some(double_to_raw_long_bits), // (D)J
        "longBitsToDouble" => Some(long_bits_to_double), // (J)D
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

// (J)D
fn long_bits_to_double(vm_thread: &mut VmThread) {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let d_value = unsafe {
        let l_value = frame.stack_pop_long();
        std::mem::transmute::<i64, f64>(l_value)
    };
    frame.stack_push(VmPrimitive::Double(d_value));
}
