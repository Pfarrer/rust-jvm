use vm::Vm;
use vm::primitive::Primitive;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "initIDs" => noop(class_path, method_name, method_signature), // ()V
        "writeBytes" => write_bytes(vm, class_path, method_name, method_signature), // ([BII)V
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

fn noop(class_path: &String, method_name: &String, method_signature: &String) {
    // Nothing to do
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
}

/// ([BII)V
fn write_bytes(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let frame = vm.frame_stack.last_mut().unwrap();
    let length = frame.stack_pop_int() as usize;
    let offset = frame.stack_pop_int() as usize;

    let rc_byte_array = frame.stack_pop_arrayref();
    let byte_array = rc_byte_array.borrow();
    assert_eq!(Some(8), byte_array.atype);

    let element_values: Vec<u8> = byte_array.elements[offset..offset+length].iter()
        .map(|p| match p {
            &Primitive::Byte(ref code) => *code,
            p => panic!("Unexpected primitive: {:?}", p),
        })
        .collect();
    let text = String::from_utf8(element_values).unwrap();

    let fd = {
        let rc_fos_instance = frame.stack_pop_objectref();
        let fos_instance = rc_fos_instance.borrow();

        let fd_instance = match fos_instance.fields.get("fd").unwrap() {
            &Primitive::Objectref(ref rc_fd_instance) => rc_fd_instance.borrow(),
            a => panic!("Not implemented for {:?}", a)
        };

        match fd_instance.fields.get("fd").unwrap() {
            &Primitive::Int(ref val) => val.clone(),
            a => panic!("Not implemented for {:?}", a)
        }
    };

    match fd {
        1 => print!("{}", text),
        2 => error!("{}", text),
        _ => panic!("Unexpected fd = {}", fd),
    };
}