use model::prelude::*;

mod java_io_filedescriptor;
mod java_io_fileinputstream;
mod java_io_fileoutputstream;
mod java_io_unixfilesystem;
mod java_lang_class;
mod java_lang_classloader;
mod java_lang_double;
mod java_lang_float;
mod java_lang_object;
mod java_lang_reflect_array;
mod java_lang_runtime;
mod java_lang_string_utf16;
mod java_lang_system;
mod java_lang_thread;
mod jdk_internal_misc_signal;
mod jdk_internal_misc_unsafe;
mod jdk_internal_misc_vm;

pub struct NativeClassloader {
    pub classloader: Box<dyn Classloader + 'static>,
}

impl Classloader for NativeClassloader {
    fn list_classes(&self) -> Vec<&str> {
        self.classloader.list_classes()
    }

    fn get_class(&self, classpath: &str) -> Option<&JvmClass> {
        self.classloader.get_class(classpath)
    }

    fn get_native_method(
        &self,
        jvm_class: &JvmClass,
        class_method: &ClassMethod,
    ) -> Option<NativeMethod> {
        match jvm_class.this_class.as_str() {
            "java/lang/Object" => java_lang_object::get_method(jvm_class, class_method),
            "java/lang/System" => java_lang_system::get_method(jvm_class, class_method),
            "java/lang/Thread" => java_lang_thread::get_method(jvm_class, class_method),
            "java/lang/Class" => java_lang_class::get_method(jvm_class, class_method),
            "java/lang/Runtime" => java_lang_runtime::get_method(jvm_class, class_method),
            "java/lang/reflect/Array" => {
                java_lang_reflect_array::get_method(jvm_class, class_method)
            }
            "jdk/internal/misc/VM" => jdk_internal_misc_vm::get_method(jvm_class, class_method),
            "java/lang/ClassLoader" => java_lang_classloader::get_method(jvm_class, class_method),
            // "java/lang/ClassLoader$NativeLibrary" => java_lang_class_loader_native_library::invoke(jvm_class, class_method),
            // "java/security/AccessController" => java_lang_accesscontroller::invoke(jvm_class, class_method),
            // "java/lang/String" => java_lang_string::invoke(jvm_class, class_method),
            // "java/lang/VmThread" => java_lang_thread::invoke(jvm_class, class_method),
            // "java/lang/Throwable" => java_lang_throwable::invoke(jvm_class, class_method),
            // "java/io/ObjectStreamClass" => java_io_objectstreamclass::invoke(class_path, method_name, method_signature),
            "java/io/FileInputStream" => {
                java_io_fileinputstream::get_method(jvm_class, class_method)
            }
            "java/io/FileDescriptor" => java_io_filedescriptor::get_method(jvm_class, class_method),
            "java/io/FileOutputStream" => {
                java_io_fileoutputstream::get_method(jvm_class, class_method)
            }
            // "java/io/FileSystem" => java_io_filesystem::invoke(jvm_class, class_method),
            "java/io/UnixFileSystem" => java_io_unixfilesystem::get_method(jvm_class, class_method),
            // "java/util/concurrent/atomic/AtomicLong" => java_util_concurrent_atomic_atomiclong::invoke(jvm_class, class_method),
            "jdk/internal/misc/Unsafe" => {
                jdk_internal_misc_unsafe::get_method(jvm_class, class_method)
            }
            // "sun/reflect/Reflection" => sun_reflect_reflection::invoke(jvm_class, class_method),
            // "sun/reflect/NativeConstructorAccessorImpl" => sun_reflect_native_constructor_accessor_impl::invoke(jvm_class, class_method),
            // "sun/misc/VM" => sun_misc_vm::invoke(jvm_class, class_method),
            "jdk/internal/misc/Signal" => {
                jdk_internal_misc_signal::get_method(jvm_class, class_method)
            }
            "java/lang/Float" => java_lang_float::get_method(jvm_class, class_method),
            "java/lang/Double" => java_lang_double::get_method(jvm_class, class_method),
            "java/lang/StringUTF16" => java_lang_string_utf16::get_method(jvm_class, class_method),
            _ => None,
        }
    }
}
