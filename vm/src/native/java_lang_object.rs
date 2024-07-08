// extern crate time;
// extern crate dirs;

use log::trace;
use model::prelude::*;

use crate::{frame::VmFrameImpl, vm_thread::VmTheadImpl};

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
        match frame.stack_pop() {
            VmPrimitive::Objectref(rc_object) => rc_object.borrow().class_path.clone(),
            VmPrimitive::Arrayref(rc_array) => {
                let name = &*rc_array.borrow_mut().class_path.clone().unwrap();
                format!("[{}", name)
            },
            p => panic!("Expected to pop Objectref or Arrayref from stack but found: {:?}", p),
        }
    };

    let rc_class_instance = vm_thread.get_java_class_instance_for(&class_path);

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Objectref(rc_class_instance));
}
