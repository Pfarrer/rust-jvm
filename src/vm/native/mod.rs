mod java_lang_system;
mod java_lang_object;
mod java_lang_class;
mod java_lang_classloader;
mod java_lang_accesscontroller;
mod java_lang_string;
mod java_io_objectstreamclass;
mod java_io_fileinputstream;
mod java_io_filedescriptor;
mod java_io_fileoutputstream;
mod sun_misc_unsafe;
mod sun_reflect_reflection;

use vm::Vm;
use vm::frame::Frame;
use classfile::Classfile;
use classfile::Method;

pub fn invoke(vm: &mut Vm, parent_frame: &mut Frame, _classfile: &Classfile, _method: &Method, class_path: &String, method_name: &String, method_signature: &String) {
    match class_path.as_ref() {
        "java/lang/System" => java_lang_system::invoke(vm, parent_frame, class_path, method_name, method_signature),
        "java/lang/Object" => java_lang_object::invoke(parent_frame, class_path, method_name, method_signature),
        "java/lang/Class" => java_lang_class::invoke(vm, parent_frame, class_path, method_name, method_signature),
        "java/lang/ClassLoader" => java_lang_classloader::invoke(vm, parent_frame, class_path, method_name, method_signature),
        "java/security/AccessController" => java_lang_accesscontroller::invoke(vm, parent_frame, class_path, method_name, method_signature),
        "java/lang/String" => java_lang_string::invoke(vm, parent_frame, class_path, method_name, method_signature),
        "java/io/ObjectStreamClass" => java_io_objectstreamclass::invoke(parent_frame, class_path, method_name, method_signature),
        "java/io/FileInputStream" => java_io_fileinputstream::invoke(parent_frame, class_path, method_name, method_signature),
        "java/io/FileDescriptor" => java_io_filedescriptor::invoke(parent_frame, class_path, method_name, method_signature),
        "java/io/FileOutputStream" => java_io_fileoutputstream::invoke(parent_frame, class_path, method_name, method_signature),
        "sun/misc/Unsafe" => sun_misc_unsafe::invoke(parent_frame, class_path, method_name, method_signature),
        "sun/reflect/Reflection" => sun_reflect_reflection::invoke(vm, parent_frame, class_path, method_name, method_signature),
        _ => panic!("No native implementation available for class {}", class_path),
    }
}