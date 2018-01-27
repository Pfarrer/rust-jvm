mod java_lang_system;
mod java_lang_object;
mod java_lang_class;
mod java_lang_accesscontroller;
mod java_io_objectstreamclass;

use vm::Vm;
use vm::frame::Frame;
use classfile::Classfile;
use classfile::Method;

pub fn invoke(vm: &mut Vm, parent_frame: &mut Frame, _classfile: &Classfile, _method: &Method, class_path: &String, method_name: &String, method_signature: &String) {
    match class_path.as_ref() {
        "java/lang/System" => java_lang_system::invoke(vm, parent_frame, class_path, method_name, method_signature),
        "java/lang/Object" => java_lang_object::invoke(parent_frame, class_path, method_name, method_signature),
        "java/lang/Class" => java_lang_class::invoke(parent_frame, class_path, method_name, method_signature),
        "java/security/AccessController" => java_lang_accesscontroller::invoke(vm, parent_frame, class_path, method_name, method_signature),
        "java/io/ObjectStreamClass" => java_io_objectstreamclass::invoke(parent_frame, class_path, method_name, method_signature),
        _ => panic!("No native implementation available for class {}", class_path),
    }
}