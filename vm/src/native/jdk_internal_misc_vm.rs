use log::trace;
use model::prelude::*;

use crate::frame::VmFrameImpl;

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "initialize" => Some(initialize), // ()V
        "initializeFromArchive" => Some(initialize_from_archive), // (Ljava/lang/Class;)V
        _ => None,
    }
}

/// ()V
fn initialize(_: &mut VmThread) {
    trace!("Execute native jdk/internal/misc/VM.initialize()V");
}

/// ()V
fn initialize_from_archive(vm_thread: &mut VmThread) {
    trace!("Execute native jdk/internal/misc/VM.initializeFromArchive(Ljava/lang/Class;)V");

    // Remove parameter from stack
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let rc_instance = frame.stack_pop_objectref();
    warn!("Not properly implemented for {:?} -> will do nothing", rc_instance);
}
