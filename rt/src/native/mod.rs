use model::prelude::*;

mod java_lang_object;
mod java_lang_system;
mod java_lang_class;
mod jdk_internal_misc_unsafe;

pub fn get_method(
    jvm_class: &JvmClass,
    class_method: &ClassMethod,
) -> Option<NativeMethod> {
    match jvm_class.this_class.as_str() {
        "java/lang/Object" => java_lang_object::get_method(jvm_class, class_method),
        "java/lang/System" => java_lang_system::get_method(jvm_class, class_method),
        "java/lang/Class" => java_lang_class::get_method(jvm_class, class_method),
        // "java/lang/ClassLoader" => java_lang_class_loader::invoke(class_path, method_name, method_signature),
        // "java/lang/ClassLoader$NativeLibrary" => java_lang_class_loader_native_library::invoke(jvm_class, class_method),
        // "java/security/AccessController" => java_lang_accesscontroller::invoke(jvm_class, class_method),
        // "java/lang/String" => java_lang_string::invoke(jvm_class, class_method),
        // "java/lang/Thread" => java_lang_thread::invoke(jvm_class, class_method),
        // "java/lang/Throwable" => java_lang_throwable::invoke(jvm_class, class_method),
        // "java/io/ObjectStreamClass" => java_io_objectstreamclass::invoke(class_path, method_name, method_signature),
        // "java/io/FileInputStream" => java_io_fileinputstream::invoke(class_path, method_name, method_signature),
        // "java/io/FileDescriptor" => java_io_filedescriptor::invoke(class_path, method_name, method_signature),
        // "java/io/FileOutputStream" => java_io_fileoutputstream::invoke(jvm_class, class_method),
        // "java/io/FileSystem" => java_io_filesystem::invoke(jvm_class, class_method),
        // "java/io/UnixFileSystem" => java_io_unixfilesystem::invoke(jvm_class, class_method),
        // "java/util/concurrent/atomic/AtomicLong" => java_util_concurrent_atomic_atomiclong::invoke(jvm_class, class_method),
        "jdk/internal/misc/Unsafe" => jdk_internal_misc_unsafe::get_method(jvm_class, class_method),
        // "sun/reflect/Reflection" => sun_reflect_reflection::invoke(jvm_class, class_method),
        // "sun/reflect/NativeConstructorAccessorImpl" => sun_reflect_native_constructor_accessor_impl::invoke(jvm_class, class_method),
        // "sun/misc/VM" => sun_misc_vm::invoke(jvm_class, class_method),
        // "sun/misc/Signal" => sun_misc_signal::invoke(jvm_class, class_method),
        // "java/lang/Float" => java_lang_float::invoke(jvm_class, class_method),
        // "java/lang/Double" => java_lang_double::invoke(jvm_class, class_method),

        _ => None,
    }
}