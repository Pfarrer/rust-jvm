use model::prelude::*;

use vm::frame::VmFrameImpl;

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "floatToRawIntBits" => Some(float_to_raw_int_bits), // (F)I
        "intBitsToFloat" => Some(int_bits_to_float),        // (I)F
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

/// (I)F
fn int_bits_to_float(vm_thread: &mut VmThread) {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let f_value = unsafe {
        let i_value = frame.stack_pop_int();
        std::mem::transmute::<i32, f32>(i_value)
    };
    frame.stack_push(VmPrimitive::Float(f_value));
}
