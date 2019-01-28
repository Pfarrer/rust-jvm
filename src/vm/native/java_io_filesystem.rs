use std::cell::RefCell;
use std::rc::Rc;

use vm::Vm;
use vm::instance::Instance;
use vm::primitive::Primitive;
use vm::utils;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "getFileSystem" => get_file_system(vm, class_path, method_name, method_signature), // ()Ljava/io/FileSystem;
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

/// ()Ljava/io/FileSystem;
fn get_file_system(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    // Use class java.io.UnixFileSystem

    // First, create instance ...
    let fs_classpath = "java.io.UnixFileSystem".to_string();
    let class = vm.load_and_clinit_class(&fs_classpath);
    let instance = Instance::new(vm, class);

    // Push twice on the stack (once for the invocation of the constructor, once as return value)
    {
        let frame = vm.frame_stack.last_mut().unwrap();
        let objectref = Primitive::Objectref(Rc::new(RefCell::new(instance)));
        frame.stack_push(objectref.clone());
        frame.stack_push(objectref);
    }

    // Call default constructor
    utils::invoke_method(vm, &fs_classpath, &"<init>".to_string(), &"()V".to_string(), true);

    trace!("Pushing initialized reference of Class {} to stack", fs_classpath);
}