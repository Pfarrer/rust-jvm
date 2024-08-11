use model::prelude::*;

use crate::frame::VmFrameImpl;

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "floatToRawIntBits" => Some(float_to_raw_int_bits), // (F)I
        _ => None,
    }
}

/// (F)I
fn float_to_raw_int_bits(vm_thread: &mut VmThread) {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let i_value = unsafe {
        let f_value = frame.stack_pop_float();
        std::mem::transmute::<f32, i32>(f_value)
    };
    frame.stack_push(VmPrimitive::Int(i_value));
}

