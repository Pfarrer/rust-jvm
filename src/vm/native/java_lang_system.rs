extern crate time;

use std::rc::Rc;
use std::cell::RefCell;

use vm::Vm;
use vm::frame::Frame;
use vm::primitive::Primitive;
use vm::instance::Instance;
use vm::utils;

pub fn invoke(vm: &mut Vm, frame: &mut Frame, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "registerNatives" => register_natives(vm, class_path, method_name, method_signature, frame),
        "currentTimeMillis" => current_time_millis(class_path, method_name, method_signature, frame), // ()J
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

fn register_natives(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String, frame: &mut Frame) {
    // Nothing to do
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let class_path_output_stream = "java/io/OutputStream".to_string();
    let class_path_print_stream = "java/io/PrintStream".to_string();
    let class_output_stream = vm.load_and_clinit_class(&class_path_output_stream);
    let class_print_stream = vm.load_and_clinit_class(&class_path_print_stream);
    let class_input_stream = vm.load_and_clinit_class(&"java/io/InputStream".to_string());

    // Create out PrintStream
    {
        let output_stream = Instance::new(class_output_stream.clone());
        let rc_output_stream = Rc::new(RefCell::new(output_stream));

        frame.stack_push(Primitive::Objectref(rc_output_stream.clone()));
        utils::invoke_method(vm, &class_path_output_stream, &"<init>".to_string(), &"()V".to_string(), frame);

        let print_stream = Instance::new(class_print_stream.clone());
        let rc_print_stream = Rc::new(RefCell::new(print_stream));

        frame.stack_push(Primitive::Objectref(rc_print_stream.clone()));
        frame.stack_push(Primitive::Objectref(rc_output_stream.clone()));
        utils::invoke_method(vm, &class_path_print_stream, &"<init>".to_string(), &"(Ljava/io/OutputStream;)V".to_string(), frame);

        vm.class_statics.get_mut(class_path).unwrap()
            .insert("out".to_string(), Primitive::Objectref(rc_print_stream.clone()));
    }

    // Create err PrintStream
    let err_stream = Instance::new(class_print_stream);
    vm.class_statics.get_mut(class_path).unwrap()
        .insert("err".to_string(), Primitive::Objectref(Rc::new(RefCell::new(err_stream))));

    // Create in InputStream
    let in_stream = Instance::new(class_input_stream);
    vm.class_statics.get_mut(class_path).unwrap()
        .insert("in".to_string(), Primitive::Objectref(Rc::new(RefCell::new(in_stream))));
}

fn current_time_millis(class_path: &String, method_name: &String, method_signature: &String, parent_frame: &mut Frame) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let time_spec = time::get_time();

    // 1459440009.113178
    let millis_float: f64 = time_spec.sec as f64 + (time_spec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
    let millis_int = (millis_float * 1000.0) as i64;

    // Push result to stack
    parent_frame.stack_push(Primitive::Long(millis_int));
}