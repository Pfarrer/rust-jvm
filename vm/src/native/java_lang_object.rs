// extern crate time;
// extern crate dirs;

use log::trace;
use model::prelude::*;

use crate::{frame::VmFrameImpl, vm_thread::{self, VmTheadImpl}};

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "registerNatives" => Some(register_natives),
        "getClass" => Some(get_class), // ()Ljava/lang/Class;
        _ => None,
    }
}

fn register_natives(_: &mut VmThread) {
    trace!("Execute native java/lang/Object.registerNatives()V");
}

/// ()Ljava/lang/Class;
fn get_class(vm_thread: &mut VmThread) {
    trace!("Execute native java/lang/Object.getClass()Ljava/lang/Class;");

    let class_path = {
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        let rc_instance = frame.stack_pop_objectref();
        let instance = rc_instance.borrow();

        instance.class_path.clone()
    };

    let rc_class_instance = vm_thread.get_java_class_instance_for(&class_path);

    debug!("{}: {:?}", class_path, rc_class_instance);
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Objectref(rc_class_instance));
}
