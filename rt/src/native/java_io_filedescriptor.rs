use log::trace;
use model::prelude::*;
use vm::frame::VmFrameImpl;

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "initIDs" => Some(init_ids), // ()V
        "getHandle" => Some(get_handle), // (I)J
        "getAppend" => Some(get_append), // (I)Z
        _ => None,
    }
}

/// ()V
fn init_ids(_: &mut VmThread) {
}

/// (I)J
fn get_handle(vm_thread: &mut VmThread) {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    
    let fd = frame.stack_pop_int();
    frame.stack_push(VmPrimitive::Long(fd as i64));
}

/// (I)Z
fn get_append(vm_thread: &mut VmThread) {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    
    let fd = frame.stack_pop_int();

    trace!("Push Boolean true for getAppend of fd {}", fd);
    frame.stack_push(VmPrimitive::Boolean(true));
}