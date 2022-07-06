use crate::defaults::VERSION;
use model::class::{ClassField, ClassInfo, JvmClass, TypeSignature};

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
            this_class: "java/lang/System".to_string(),
            super_class: Some("java/lang/Object".to_string()),
            interfaces: vec![],
        },
        fields: vec![ClassField {
            access_flags: JvmClass::ACC_STATIC,
            name: "out".to_string(),
            descriptor: TypeSignature::Class("java/io/PrintStream".to_string()),
            attributes: vec![],
        }],
        methods: vec![],
        attributes: vec![],
    }
}
