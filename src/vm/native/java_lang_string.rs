use vm::Vm;
use vm::primitive::Primitive;
use vm::string_pool::StringPool;
use vm::utils;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "intern" => intern(vm, class_path, method_name, method_signature), // ()Ljava/lang/String;
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

/// ()Ljava/lang/String;
fn intern(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let value = {
        let frame = vm.frame_stack.last_mut().unwrap();

        let rc_instance = frame.stack_pop_objectref();
        let instance = rc_instance.borrow();

        utils::get_java_string_value(&instance)
    };

    let rc_interned_instance = StringPool::intern(vm, &value);

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Objectref(rc_interned_instance));
}