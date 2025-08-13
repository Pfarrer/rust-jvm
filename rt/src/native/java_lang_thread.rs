use std::{cell::RefCell, rc::Rc};

use model::prelude::*;
use vm::{frame::VmFrameImpl, instance::VmInstanceImpl, vm_thread::VmTheadImpl};

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "registerNatives" => Some(register_natives),
        "currentThread" => Some(current_thread), // ()Ljava/lang/Thread;
        _ => None,
    }
}

fn register_natives(_: &mut VmThread) {}

/// Ljava/lang/Thread;
fn current_thread(vm_thread: &mut VmThread) {
    let rc_thread_group_instance = {
        let class_path = "java/lang/ThreadGroup".to_string();
        let classfile = vm_thread.load_and_clinit_class(&class_path);
        let instance = VmInstance::new(vm_thread, &classfile);
        let rc_instance = Rc::new(RefCell::new(instance));

        // Push instance to stack
        {
            let frame = vm_thread.frame_stack.last_mut().unwrap();
            frame.stack_push(VmPrimitive::Objectref(rc_instance.clone()));
        }

        // Invoke constructor
        vm_thread.invoke_method(&class_path, &"<init>".to_string(), &"()V".to_string(), true);

        rc_instance
    };

    // Make instance of Thread
    let rc_thread_instance = {
        let class_path = "java/lang/Thread".to_string();
        let classfile = vm_thread.load_and_clinit_class(&class_path);
        let mut instance = VmInstance::new(vm_thread, &classfile);

        // Manually initialize some fields
        instance.fields.insert(
            "group".to_string(),
            VmPrimitive::Objectref(rc_thread_group_instance),
        );
        instance
            .fields
            .insert("priority".to_string(), VmPrimitive::Int(1));

        Rc::new(RefCell::new(instance))
    };

    // Finally, push instance to stack
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Objectref(rc_thread_instance));
}
