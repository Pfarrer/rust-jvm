use crate::defaults::VERSION;
use model::class::{ClassInfo, JvmClass};

pub fn tuple() -> (String, JvmClass) {
    let jvm_class = make_jvm_class();
    (jvm_class.class_info.this_class.clone(), jvm_class)
}

fn make_jvm_class() -> JvmClass {
    JvmClass {
        version: VERSION,
        constants: vec![],
        class_info: ClassInfo {
            access_flags: 0,
            this_class: "java/lang/Object".to_string(),
            super_class: None,
            interfaces: vec![],
        },
        fields: vec![],
        methods: vec![],
        attributes: vec![],
    }
}
