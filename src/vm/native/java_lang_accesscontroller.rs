use vm::Vm;
use vm::frame::Frame;
//use vm::primitive::Primitive;
use vm::utils;

pub fn invoke(vm: &mut Vm, parent_frame: &mut Frame, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "registerNatives" => register_natives(class_path, method_name, method_signature),
        "doPrivileged" => do_privileged(vm, parent_frame, class_path, method_name, method_signature),
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

fn register_natives(class_path: &String, method_name: &String, method_signature: &String) {
    // Nothing to do
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
}

/// doPrivileged(Ljava/security/PrivilegedAction;)Ljava/lang/Object;
fn do_privileged(vm: &mut Vm, frame: &mut Frame, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    // TODO check if privileged to call run()Ljava/lang/Object;

    utils::invoke_instance_method(vm, &"run".to_string(), &"()Ljava/lang/Object;".to_string(), frame);
}
