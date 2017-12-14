extern crate time;

use vm::frame::Frame;
use vm::primitive::Primitive;

pub fn invoke(parent_frame: &mut Frame, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "registerNatives" => register_natives(class_path, method_name, method_signature),
        "currentTimeMillis" => current_time_millis(parent_frame, class_path, method_name, method_signature), // ()J
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

fn register_natives(class_path: &String, method_name: &String, method_signature: &String) {
    // Nothing to do
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
}

fn current_time_millis(parent_frame: &mut Frame, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let time_spec = time::get_time();

    // 1459440009.113178
    let millis_float: f64 = time_spec.sec as f64 + (time_spec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
    let millis_int = (millis_float * 1000.0) as i64;

    // Push result to stack
    parent_frame.stack_push(Primitive::Long(millis_int));
}