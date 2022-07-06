mod defaults;
mod java_lang_object;
mod java_lang_string;
mod java_lang_system;

use model::api::Classloader;
use model::class::JvmClass;
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
}
