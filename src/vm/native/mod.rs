mod java_lang_system;

use vm::Vm;
use vm::frame::Frame;
use classfile::Classfile;
use classfile::Method;

pub fn invoke(_vm: &mut Vm, parent_frame: &mut Frame, _classfile: &Classfile, _method: &Method, class_path: &String, method_name: &String, method_signature: &String) {
    match class_path.as_ref() {
        "java/lang/System" => java_lang_system::invoke(parent_frame, class_path, method_name, method_signature),
        _ => panic!("No native implementation available for class {}", class_path),
    }
}