mod java_lang_system;
mod java_lang_object;
mod java_lang_class;
mod java_lang_class_loader;
mod java_lang_class_loader_native_library;
mod java_lang_accesscontroller;
mod java_lang_string;
mod java_lang_thread;
mod java_lang_throwable;
mod java_io_objectstreamclass;
mod java_io_fileinputstream;
mod java_io_filedescriptor;
mod java_io_fileoutputstream;
mod java_io_filesystem;
mod java_io_unixfilesystem;
mod java_util_concurrent_atomic_atomiclong;
mod sun_misc_unsafe;
mod sun_reflect_reflection;
mod sun_reflect_native_constructor_accessor_impl;
mod sun_misc_vm;

use vm::Vm;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match class_path.as_ref() {
        "java/lang/System" => java_lang_system::invoke(vm, class_path, method_name, method_signature),
        "java/lang/Object" => java_lang_object::invoke(vm, class_path, method_name, method_signature),
        "java/lang/Class" => java_lang_class::invoke(vm, class_path, method_name, method_signature),
        "java/lang/ClassLoader" => java_lang_class_loader::invoke(class_path, method_name, method_signature),
        "java/lang/ClassLoader$NativeLibrary" => java_lang_class_loader_native_library::invoke(vm, class_path, method_name, method_signature),
        "java/security/AccessController" => java_lang_accesscontroller::invoke(vm, class_path, method_name, method_signature),
        "java/lang/String" => java_lang_string::invoke(vm, class_path, method_name, method_signature),
        "java/lang/Thread" => java_lang_thread::invoke(vm, class_path, method_name, method_signature),
        "java/lang/Throwable" => java_lang_throwable::invoke(vm, class_path, method_name, method_signature),
        "java/io/ObjectStreamClass" => java_io_objectstreamclass::invoke(class_path, method_name, method_signature),
        "java/io/FileInputStream" => java_io_fileinputstream::invoke(class_path, method_name, method_signature),
        "java/io/FileDescriptor" => java_io_filedescriptor::invoke(class_path, method_name, method_signature),
        "java/io/FileOutputStream" => java_io_fileoutputstream::invoke(class_path, method_name, method_signature),
        "java/io/FileSystem" => java_io_filesystem::invoke(vm, class_path, method_name, method_signature),
        "java/io/UnixFileSystem" => java_io_unixfilesystem::invoke(vm, class_path, method_name, method_signature),
        "java/util/concurrent/atomic/AtomicLong" => java_util_concurrent_atomic_atomiclong::invoke(vm, class_path, method_name, method_signature),
        "sun/misc/Unsafe" => sun_misc_unsafe::invoke(vm, class_path, method_name, method_signature),
        "sun/reflect/Reflection" => sun_reflect_reflection::invoke(vm, class_path, method_name, method_signature),
        "sun/reflect/NativeConstructorAccessorImpl" => sun_reflect_native_constructor_accessor_impl::invoke(vm, class_path, method_name, method_signature),
        "sun/misc/VM" => sun_misc_vm::invoke(vm, class_path, method_name, method_signature),

        _ => panic!("No native implementation available for class {}", class_path),
    }
}
