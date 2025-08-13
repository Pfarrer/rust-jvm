use log::warn;
use model::prelude::*;

use vm::frame::VmFrameImpl;

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "initialize" => Some(initialize),                         // ()V
        "initializeFromArchive" => Some(initialize_from_archive), // (Ljava/lang/Class;)V
        _ => None,
    }
}

/// ()V
fn initialize(_: &mut VmThread) {}

/// ()V
fn initialize_from_archive(vm_thread: &mut VmThread) {
    // Remove parameter from stack
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let rc_instance = frame.stack_pop_objectref();
    warn!(
        "Not properly implemented for {:?} -> will do nothing",
        rc_instance
    );
}
