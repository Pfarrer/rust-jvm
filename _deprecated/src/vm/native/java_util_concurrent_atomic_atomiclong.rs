use vm::Vm;
use vm::primitive::Primitive;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "registerNatives" => register_natives(class_path, method_name, method_signature),
        "VMSupportsCS8" => vm_supports_cs8(vm, class_path, method_name, method_signature),
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

fn register_natives(class_path: &String, method_name: &String, method_signature: &String) {
    // Nothing to do
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
}

fn vm_supports_cs8(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Boolean(true));
}
