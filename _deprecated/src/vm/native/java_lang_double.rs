use vm::Vm;
use vm::primitive::Primitive;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "doubleToLongBits" => double_to_long_bits(vm, class_path, method_name, method_signature), // (D)J
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

/// (D)J
fn double_to_long_bits(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let frame = vm.frame_stack.last_mut().unwrap();
    let dval = frame.stack_pop_double();
    let lval = dval as i64;

    trace!("Popped Double {} from stack and push Long {} back", dval, lval);

    frame.stack_push(Primitive::Long(lval));
}