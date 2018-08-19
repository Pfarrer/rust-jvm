use vm::Vm;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "fillInStackTrace" => fill_in_stack_trace(vm), // ()Ljava/lang/Throwable;
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

//fn noop(class_path: &String, method_name: &String, method_signature: &String) {
    // Nothing to do
//    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
//}

/// ()Ljava/lang/Throwable;
fn fill_in_stack_trace(vm: &mut Vm) {
    for frame in vm.frame_stack.iter().rev() {
        debug!("{}", frame);
    }

    panic!("test");
}
