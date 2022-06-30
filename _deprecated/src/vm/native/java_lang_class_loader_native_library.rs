use vm::Vm;
use vm::primitive::Primitive;
use vm::utils;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "load" => load(vm, class_path, method_name, method_signature), // (Ljava/lang/String;)V
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

/// (Ljava/lang/String;)V
fn load(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let value = {
        let frame = vm.frame_stack.last_mut().unwrap();
        let rc_string = frame.stack_pop_objectref();
        let string = rc_string.borrow();

        utils::get_java_string_value(&*string)
    };

    assert_eq!("/libzip.so", value);

    let frame = vm.frame_stack.last_mut().unwrap();
    let rc_nativelib = frame.stack_pop_objectref();
    let mut nativelib = rc_nativelib.borrow_mut();

    nativelib.fields.insert("handle".to_owned(), Primitive::Long(1337));

    warn!("Loading /libzip.so not implemented... will fake a handle value");
}
