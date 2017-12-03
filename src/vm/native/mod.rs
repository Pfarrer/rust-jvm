mod java_lang_System;

use vm::Vm;
use vm::frame::Frame;
use classfile::Classfile;
use classfile::Method;

pub fn invoke(vm: &mut Vm, frame: &mut Frame, classfile: &Classfile, method: &Method, class_path: &String, method_name: &String, method_signature: &String) {
    match class_path.as_ref() {
        "java/lang/System" => java_lang_System::invoke(vm, frame, classfile, method, class_path, method_name, method_signature),
        _ => panic!("No native implementation available for class {}", class_path),
    }
}