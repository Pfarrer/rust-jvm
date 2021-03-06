pub fn invoke(class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "initNative" => init_native(class_path, method_name, method_signature),
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

fn init_native(class_path: &String, method_name: &String, method_signature: &String) {
    // Nothing to do
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
}