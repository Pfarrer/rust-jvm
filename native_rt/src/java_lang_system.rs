use crate::defaults::VERSION;
use model::api::NativeMethod;
use model::class::{
    ClassConstant, ClassField, ClassInfo, ClassMethod, JvmClass, MethodSignature, TypeSignature,
};

pub fn tuple() -> (String, JvmClass) {
    let jvm_class = make_jvm_class();
    (jvm_class.class_info.this_class.clone(), jvm_class)
}

fn make_jvm_class() -> JvmClass {
    JvmClass {
        version: VERSION,
        constants: vec![ClassConstant::None()],
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
        methods: vec![ClassMethod {
            access_flags: JvmClass::ACC_STATIC | JvmClass::ACC_NATIVE,
            name: "<clinit>".to_string(),
            descriptor: MethodSignature {
                parameters: vec![],
                return_type: TypeSignature::Void,
            },
            attributes: vec![],
        }],
        attributes: vec![],
    }
}

pub fn get_native_method(class_method: &ClassMethod) -> Option<NativeMethod> {
    match (
        class_method.name.as_ref(),
        class_method.descriptor.to_string().as_ref(),
    ) {
        ("<clinit>", "()V") => Some(native_clinit),
        _ => None,
    }
}

fn native_clinit() {
    // Initialize static member variables
    panic!()
}
