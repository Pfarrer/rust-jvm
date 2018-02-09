use vm::Vm;
use vm::frame::Frame;
use vm::primitive::Primitive;
use vm::string_pool::StringPool;
use vm::utils;

pub fn invoke(vm: &mut Vm, parent_frame: &mut Frame, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "intern" => intern(vm, parent_frame, class_path, method_name, method_signature), // ()Ljava/lang/String;
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

//fn noop(class_path: &String, method_name: &String, method_signature: &String) {
//    // Nothing to do
//    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
//}

/// ()Ljava/lang/String;
fn intern(vm: &mut Vm, frame: &mut Frame, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let rc_instance = frame.stack_pop_objectref();
    let instance = rc_instance.borrow();
    let value = utils::get_java_string_value(&instance);

    let rc_interned_instance = StringPool::intern(vm, &value);
    frame.stack_push(Primitive::Objectref(rc_interned_instance));
}