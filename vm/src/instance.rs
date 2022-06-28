use std::collections::HashMap;
use model::class::JvmClass;
use crate::primitive::Primitive;
use crate::Vm;

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    pub class_path: String,
    pub fields: HashMap<String, Primitive>
}

impl Instance {
    pub fn new(_vm: &Vm, _jvm_class: JvmClass) -> Instance {
        todo!()
        // let class_path = utils::get_class_path(&jvm_class);
        //
        // let mut fields = HashMap::new();
        // initialize_instance_fields(&jvm_class, &mut fields);
        //
        // let hierarchy_iter = ClassHierarchy::hierarchy_iter(vm, &class_path);
        // for (parent_class, _, _) in hierarchy_iter {
        //     initialize_instance_fields(&parent_class, &mut fields);
        // }
        //
        // Instance {
        //     class_path,
        //     fields,
        // }
    }
}

// fn initialize_instance_fields(jvm_class: &JvmClass, instance_fields: &mut HashMap<String, Primitive>) {
//     for field in &jvm_class.fields {
//         let name = utils::get_utf8_value(&jvm_class, field.name_index as usize);
//         let descriptor = utils::get_utf8_value(&jvm_class, field.descriptor_index as usize);
//
//         let sig = signature::parse_field(&descriptor);
//         let default_value = Primitive::get_default_value(&sig);
//
//         instance_fields.insert(name, default_value);
//     }
// }
