use vm::Vm;
use vm::primitive::Primitive;
use vm::utils;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "initIDs" => noop(class_path, method_name, method_signature), // ()V
        "getBooleanAttributes0" => get_boolean_attributes0(vm, class_path, method_name, method_signature), // (Ljava/io/File;)I
        "canonicalize0" => canonicalize0(vm, class_path, method_name, method_signature), // (Ljava/lang/String;)Ljava/lang/String;
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

fn noop(class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
}

/// (Ljava/io/File;)I
/// Constants for simple boolean attributes
/// BA_EXISTS    = 0x01;
/// BA_REGULAR   = 0x02;
/// BA_DIRECTORY = 0x04;
/// BA_HIDDEN    = 0x08;
fn get_boolean_attributes0(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let file_path = {
        let frame = vm.frame_stack.last_mut().unwrap();
        let rc_file = frame.stack_pop_objectref();
        let file = rc_file.borrow();

        utils::get_instance_field_string_value(&*file, "path")
    };

    assert_eq!("/libzip.so", file_path);

    trace!("Popped Objecref from stack and push Int back");

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Int(0x01 | 0x02));
}

/// (Ljava/lang/String;)Ljava/lang/String;
fn canonicalize0(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let frame = vm.frame_stack.last_mut().unwrap();
    let rc_string = frame.stack_pop_objectref();
    let string = rc_string.borrow();

    let original_path = utils::get_java_string_value(&*string);

    assert_eq!("/libzip.so", original_path);

    trace!("Fake implementation, just pushing back the original path {}", original_path);
    frame.stack_push(Primitive::Objectref(rc_string.clone()));

 /*   let path = Path::new(&original_path);
    let canonicalized_path = fs::canonicalize(path).unwrap();

    let canonicalized_path_str = canonicalized_path.to_str().unwrap();
    trace!("Canonicalized original path ({}) to {}", original_path, canonicalized_path_str);

    // Interning is not required here and actually a waste of memeory, but it is simpler for the
    // moment than creating a new java.lang.String instance here
    let rc_canonicalized = StringPool::intern(vm, &canonicalized_path_str.to_string());

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Objectref(rc_canonicalized));
*/
}