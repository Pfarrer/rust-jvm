extern crate time;

use vm::Vm;
use vm::frame::Frame;
use vm::primitive::Primitive;

pub fn invoke(vm: &mut Vm, frame: &mut Frame, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "registerNatives" => register_natives(vm, class_path, method_name, method_signature, frame),
        "currentTimeMillis" => current_time_millis(class_path, method_name, method_signature, frame), // ()J
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

fn register_natives(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String, frame: &mut Frame) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
    vm.invoke_static(class_path, &"initializeSystemClass".to_string(), &"()V".to_string(), frame);
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