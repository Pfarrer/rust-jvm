use std::{cell::RefCell, rc::Rc};

use model::prelude::*;

use vm::{array::VmArrayImpl, frame::VmFrameImpl, utils::get_java_string_value};

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "newArray" => Some(new_array), // (Ljava/lang/Class;I)Ljava/lang/Object;
        _ => None,
    }
}

/// (Ljava/lang/Class;I)Ljava/lang/Object;
fn new_array(vm_thread: &mut VmThread) {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let count = frame.stack_pop_int() as usize;
    let rc_instance_class = frame.stack_pop_objectref();

    let class_path = {
        let class_instance: &VmInstance = &*rc_instance_class.borrow_mut();
        assert_eq!(class_instance.class_path, "java/lang/Class");

        match &class_instance.fields["name"] {
            VmPrimitive::Objectref(ref rc_object) => get_java_string_value(&*rc_object.borrow_mut()),
            a => panic!("Expected Arrayref but found: {:?}", a),
        }
    };

    let array = VmArray::new_complex(count, class_path);
    let rc_array = Rc::new(RefCell::new(array));

    let frame: &mut VmFrame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Arrayref(rc_array));
}
