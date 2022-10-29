mod defaults;
mod java_lang_object;
mod java_lang_string;
mod java_lang_system;

use model::api::{Classloader, NativeMethod};
use model::class::{ClassMethod, JvmClass};
use std::collections::HashMap;

pub fn make_classloader() -> impl Classloader {
    NativeRuntimeLoader {
        classes: HashMap::from([
            java_lang_object::tuple(),
            java_lang_system::tuple(),
            java_lang_string::tuple(),
        ]),
    }
}

pub struct NativeRuntimeLoader {
    classes: HashMap<String, JvmClass>,
}

impl Classloader for NativeRuntimeLoader {
    fn list_classes(&self) -> Vec<&str> {
        self.classes.keys().map(|s| s.as_str()).collect()
    }

    fn get_class(&self, class_path: &str) -> Option<&JvmClass> {
        self.classes.get(class_path)
    }

    fn get_native_method(
        &self,
        jvm_class: &JvmClass,
        class_method: &ClassMethod,
    ) -> Option<NativeMethod> {
        println!("get_native {}", class_method.name);
        match jvm_class.class_info.this_class.as_ref() {
            "java/lang/System" => java_lang_system::get_native_method(class_method),
            _ => None,
        }
    }
}
