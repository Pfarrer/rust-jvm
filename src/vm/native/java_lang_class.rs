use vm::frame::Frame;
use vm::primitive::Primitive;

pub fn invoke(parent_frame: &mut Frame, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "registerNatives" => register_natives(class_path, method_name, method_signature),
        "getPrimitiveClass" => get_primitive_class(parent_frame, class_path, method_name, method_signature),
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

fn register_natives(class_path: &String, method_name: &String, method_signature: &String) {
    // Nothing to do
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
}

/// getPrimitiveClass(Ljava/lang/String;)Ljava/lang/Class;
fn get_primitive_class(frame: &mut Frame, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
//    trace!("{:#?}", frame);
    warn!("This method is not implemented properly!!");
    frame.stack_push(Primitive::Null);
}