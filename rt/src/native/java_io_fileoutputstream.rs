use model::prelude::*;
use vm::frame::VmFrameImpl;

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "initIDs" => Some(init_ids),       // ()V
        "writeBytes" => Some(write_bytes), // ([BIIZ)V
        _ => None,
    }
}

/// ()V
fn init_ids(_: &mut VmThread) {}

/// ([BIIZ)V
fn write_bytes(vm_thread: &mut VmThread) {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let _append = frame.stack_pop_boolean() as usize;
    let length = frame.stack_pop_int() as usize;
    let offset = frame.stack_pop_int() as usize;

    let rc_byte_array = frame.stack_pop_arrayref();
    let byte_array = rc_byte_array.borrow();
    assert_eq!(Some(8), byte_array.atype);

    let fd = {
        let rc_fos_instance = frame.stack_pop_objectref();
        let fos_instance = rc_fos_instance.borrow();

        let fd_instance = match fos_instance.fields.get("fd").unwrap() {
            &VmPrimitive::Objectref(ref rc_fd_instance) => rc_fd_instance.borrow(),
            a => panic!("Not implemented for {:?}", a),
        };

        match fd_instance.fields.get("fd").unwrap() {
            &VmPrimitive::Int(ref val) => val.clone(),
            a => panic!("Not implemented for {:?}", a),
        }
    };

    // Write raw bytes to the file descriptor without attempting string decoding
    use std::io::{self, Write};

    let slice = &byte_array.elements[offset..offset + length];
    let mut buf: Vec<u8> = Vec::with_capacity(slice.len());
    for prim in slice {
        match prim {
            VmPrimitive::Byte(b) => buf.push(*b),
            _ => panic!("Unexpected primitive in byte array: {:?}", prim),
        }
    }

    // Write raw bytes to stdout or stderr
    let mut handle: Box<dyn Write> = match fd {
        1 => Box::new(io::stdout()),
        2 => Box::new(io::stderr()),
        _ => panic!("Unexpected fd = {}", fd),
    };
    handle.write_all(&buf).expect("Failed to write to output");
    handle.flush().expect("Failed to flush output");
}
