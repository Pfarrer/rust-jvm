use log::trace;
use model::prelude::*;
use vm::frame::VmFrameImpl;

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "getStackAccessControlContext" => Some(get_stack_access_control_context), // getStackAccessControlContext()Ljava/security/AccessControlContext;
        _ => None,
    }
}

/// getStackAccessControlContext()Ljava/security/AccessControlContext;
fn get_stack_access_control_context(vm_thread: &mut VmThread) {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Null);

    trace!("Pushed Null to stack");
}
