use vm::Vm;
use vm::primitive::Primitive;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "floatToIntBits" => float_to_int_bits(vm, class_path, method_name, method_signature), // (F)I
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

/// (F)I
fn float_to_int_bits(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let frame = vm.frame_stack.last_mut().unwrap();
    let fval = frame.stack_pop_float();
    let ival = fval as i32;

    trace!("Popped Float {} from stack and push Int {} back", fval, ival);

    frame.stack_push(Primitive::Int(ival));
}