use vm::Vm;
use vm::primitive::Primitive;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "registerNatives" => noop(class_path, method_name, method_signature), // ()V
        "objectFieldOffset" => object_field_offset(vm, class_path, method_name, method_signature), // (Ljava/lang/reflect/Field;)J
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

fn noop(class_path: &String, method_name: &String, method_signature: &String) {
    // Nothing to do
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
}

/// (Ljava/lang/reflect/Field;)J
fn object_field_offset(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    // Remove parameter from stack
    let frame = vm.frame_stack.last_mut().unwrap();
    let _ = frame.stack_pop_objectref();
//    let instance = rc_instance.borrow();

    warn!("Not properly implemented -> will always return 0L");

    frame.stack_push(Primitive::Long(0));
}
