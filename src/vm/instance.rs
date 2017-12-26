use std::collections::HashMap;

use classfile::Classfile;
use vm::primitive::Primitive;
use vm::utils;

#[derive(Debug, Clone)]
pub struct Instance {
    class: Classfile,
    fields: HashMap<String, Primitive>
}

impl Instance {
    pub fn new(class: Classfile) -> Instance {
        let fields = HashMap::new();
        for field in &class.fields {
            let name = utils::get_utf8_value(&class, field.name_index as usize);
            let descriptor = utils::get_utf8_value(&class, field.descriptor_index as usize);

            panic!("Initialize field {} of type {}", name, descriptor);
        }

        Instance {
            class,
            fields,
        }
    }
}