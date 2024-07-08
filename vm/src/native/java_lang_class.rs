use std::{cell::RefCell, rc::Rc};

use log::trace;
use model::prelude::*;

use crate::{
    frame::VmFrameImpl, instance::VmInstanceImpl, utils::{self, get_java_string_value}, vm_mem::VmStringPoolImpl, vm_thread::VmTheadImpl
};

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "registerNatives" => Some(register_natives),
        "getPrimitiveClass" => Some(get_primitive_class), // (Ljava/lang/String;)Ljava/lang/Class;
        "isArray" => Some(is_array), // ()Z
        _ => None,
    }
}

fn register_natives(_: &mut VmThread) {
    trace!("Execute native java/lang/Class.registerNatives()V");
}

/// (Ljava/lang/String;)Ljava/lang/Class;
fn get_primitive_class(vm_thread: &mut VmThread) {
    trace!("Execute native java/lang/Class.getPrimitiveClass(Ljava/lang/String;)Ljava/lang/Class;");

    // primitive_name will be something like "void" or "long" etc.
    let primitive_name = {
        let frame = vm_thread.frame_stack.last_mut().unwrap();

        let rc_instance = frame.stack_pop_objectref();
        let mut_ref_instance = rc_instance.borrow_mut();

        utils::get_java_string_value(&*mut_ref_instance)
    };

    let jvm_class = vm_thread.load_and_clinit_class(&"java/lang/Class".to_string());
    let mut class_instance = VmInstance::new(vm_thread, &jvm_class);

    let rc_interned_name = vm_thread
        .vm
        .mem
        .string_pool
        .intern(vm_thread, &primitive_name);
    class_instance
        .fields
        .insert("name".to_string(), VmPrimitive::Objectref(rc_interned_name));

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Objectref(Rc::new(RefCell::new(class_instance))));
}

/// ()Z
fn is_array(vm_thread: &mut VmThread) {
    let frame = vm_thread.frame_stack.last_mut().unwrap();

    let rc_instance_class = frame.stack_pop_objectref();
    let name = {
        let class_instance: &VmInstance = &*rc_instance_class.borrow_mut();
        assert_eq!(class_instance.class_path, "java/lang/Class");

        match &class_instance.fields["name"] {
            VmPrimitive::Objectref(ref rc_object) => get_java_string_value(&*rc_object.borrow_mut()),
            a => panic!("Expected Arrayref but found: {:?}", a),
        }
    };
    
    let result = name.starts_with("[");
    debug!("Popped Objectref from stack and found Class for type {}, will push Boolean {} to the stack", name, result);

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Boolean(result));
}