use vm::frame::Frame;
use vm::primitive::Primitive;

pub fn invoke(parent_frame: &mut Frame, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "registerNatives" => register_natives(class_path, method_name, method_signature),
        "hashCode" => hash_code(parent_frame, class_path, method_name, method_signature), // ()I
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

fn register_natives(class_path: &String, method_name: &String, method_signature: &String) {
    // Nothing to do
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
}

fn hash_code(frame: &mut Frame, class_path: &String, method_name: &String, method_signature: &String) {
    // Nothing to do
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    frame.stack_pop();
    frame.stack_push(Primitive::Int(777));
//    panic!("{:#?}", frame);

}