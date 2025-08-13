use log::warn;
use model::prelude::*;

use vm::{frame::VmFrameImpl, utils::get_java_string_value};

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "findSignal0" => Some(find_signal0), // (Ljava/lang/String;)I
        "handle0" => Some(handle0),          // (IJ)J
        _ => None,
    }
}

/// (Ljava/lang/String;)I
fn find_signal0(vm_thread: &mut VmThread) {
    // Remove parameter from stack
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let rc_objectref = frame.stack_pop_objectref();

    warn!(
        "Not properly implemented for signal {} -> will always return 0",
        get_java_string_value(&*rc_objectref.borrow())
    );
    frame.stack_push(VmPrimitive::Int(0));
}

/// (IJ)J
fn handle0(vm_thread: &mut VmThread) {
    // Remove parameters from stack
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let native_h = frame.stack_pop_long();
    let sig = frame.stack_pop_int();

    warn!(
        "Not properly implemented for signal {} and nativeH {} -> will always return 0",
        sig, native_h
    );
    frame.stack_push(VmPrimitive::Long(0));
}
