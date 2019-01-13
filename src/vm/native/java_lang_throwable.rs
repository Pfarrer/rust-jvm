use vm::Vm;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "fillInStackTrace" => fill_in_stack_trace(vm), // ()Ljava/lang/Throwable;
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

/// ()Ljava/lang/Throwable;
fn fill_in_stack_trace(vm: &mut Vm) {
    for frame in vm.frame_stack.iter() {
        println!("Frame of: {}.{}{}", frame.class_path, frame.method_name, frame.method_signature);
    }

    panic!("not implemented");
}