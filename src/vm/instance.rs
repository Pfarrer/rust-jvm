use std::collections::HashMap;

use classfile::Classfile;
use vm::primitive::Primitive;
use vm::signature;
use vm::utils;

#[derive(Debug, Clone)]
pub struct Instance {
    class_path: String,
    pub fields: HashMap<String, Primitive>
}

impl Instance {
    pub fn new(class: Classfile) -> Instance {
        let class_path = utils::get_class_path(&class);
        debug!("Create new instance of class {}", class_path);

        let mut fields = HashMap::new();
        for field in &class.fields {
            let name = utils::get_utf8_value(&class, field.name_index as usize);
            let descriptor = utils::get_utf8_value(&class, field.descriptor_index as usize);

            let sig = signature::parse_field(&descriptor);
            let default_value = Primitive::get_default_value(&sig);

            fields.insert(name, default_value);
        }

        Instance {
            class_path,
            fields,
        }
    }
}