use std::rc::Rc;
use std::cell::RefCell;

use vm::Vm;
use vm::instance::Instance;
use vm::primitive::Primitive;
use vm::utils;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "registerNatives" => noop(class_path, method_name, method_signature),
        "currentThread" => current_thread(vm, class_path, method_name, method_signature),
        "setPriority0" => set_priority0(vm, class_path, method_name, method_signature),
        "isAlive" => is_alive(vm, class_path, method_name, method_signature),
        "start0" => noop(class_path, method_name, method_signature),
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

fn noop(class_path: &String, method_name: &String, method_signature: &String) {
    // Nothing to do
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
}

/// Ljava/lang/Thread;
fn current_thread(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let rc_thread_group_instance = {
        let class_path = "java/lang/ThreadGroup".to_string();
        let classfile = vm.load_and_clinit_class(&class_path);
        let instance = Instance::new(vm, classfile);
        let rc_instance = Rc::new(RefCell::new(instance));

        // Push instance to stack
        {
            let frame = vm.frame_stack.last_mut().unwrap();
            frame.stack_push(Primitive::Objectref(rc_instance.clone()));
        }

        // Invoke constructor
        utils::invoke_method(vm, &class_path, &"<init>".to_string(), &"()V".to_string(), true);

        rc_instance
    };

    // Make instance of Thread
    let rc_thread_instance = {
        let class_path = "java/lang/Thread".to_string();
        let classfile = vm.load_and_clinit_class(&class_path);
        let mut instance = Instance::new(vm, classfile);

        // Manually initialize some fields
        instance.fields.insert("group".to_string(), Primitive::Objectref(rc_thread_group_instance));
        instance.fields.insert("priority".to_string(), Primitive::Int(1));

        Rc::new(RefCell::new(instance))
    };

    // Finally, push instance to stack
    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Objectref(rc_thread_instance));

    trace!("Pushed Thread reference to stack");
}

/// (I)V
fn set_priority0(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let frame = vm.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_int();

    trace!("Popped {} from stack, doing noting else", value);
}

/// ()Z
fn is_alive(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Boolean(false));

    trace!("Pushed False to stack");
}
