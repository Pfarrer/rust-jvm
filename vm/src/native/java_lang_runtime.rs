use log::{trace, warn};
use model::prelude::*;

use crate::frame::VmFrameImpl;

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "availableProcessors" => Some(available_processors), // ()I
        _ => None,
    }
}

/// ()I
fn available_processors(vm_thread: &mut VmThread) {
    trace!("Execute native java/lang/Runtime.availableProcessors()I");

    // Remove parameter from stack
    let frame = vm_thread.frame_stack.last_mut().unwrap();

    warn!("Not properly implemented -> will always return 8");
    frame.stack_push(VmPrimitive::Int(8));
}
