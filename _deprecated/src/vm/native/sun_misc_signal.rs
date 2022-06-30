use vm::Vm;
use vm::primitive::Primitive;
use vm::utils;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "findSignal" => find_signal(vm, class_path, method_name, method_signature), // (Ljava/lang/String;)I
        "handle0" => handle0(vm, class_path, method_name, method_signature), // (IJ)J
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

/// (Ljava/lang/String;)I
fn find_signal(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
    
    let frame = vm.frame_stack.last_mut().unwrap();

    let rc_string = frame.stack_pop_objectref();
    let string = rc_string.borrow();
    let value = utils::get_java_string_value(&*string);

    warn!("findSignal called for {}, will return 0", value);

    frame.stack_push(Primitive::Int(0));
}

/// (IJ)J
fn handle0(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let frame = vm.frame_stack.last_mut().unwrap();

    let val2 = frame.stack_pop_long();
    let val1 = frame.stack_pop_int();

    trace!("handle0: Popped Int {} and Long {} from stack and pushed Long 0 back", val1, val2);

    frame.stack_push(Primitive::Long(0));
}