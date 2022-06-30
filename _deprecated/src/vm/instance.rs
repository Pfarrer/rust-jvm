use std::collections::HashMap;

use classfile::Classfile;
use vm::Vm;
use vm::primitive::Primitive;
use vm::class_hierarchy::ClassHierarchy;
use vm::signature;
use vm::utils;

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    pub class_path: String,
    pub fields: HashMap<String, Primitive>
}

impl Instance {
    pub fn new(vm: &mut Vm, class: Classfile) -> Instance {
        let class_path = utils::get_class_path(&class);

        let mut fields = HashMap::new();
        initialize_instance_fields(&class, &mut fields);

        let hierarchy_iter = ClassHierarchy::hierarchy_iter(vm, &class_path);
        for (parent_class, _, _) in hierarchy_iter {
            initialize_instance_fields(&parent_class, &mut fields);
        }

        Instance {
            class_path,
            fields,
        }
    }
}

fn initialize_instance_fields(class: &Classfile, instance_fields: &mut HashMap<String, Primitive>) {
    for field in &class.fields {
        let name = utils::get_utf8_value(&class, field.name_index as usize);
        let descriptor = utils::get_utf8_value(&class, field.descriptor_index as usize);

        let sig = signature::parse_field(&descriptor);
        let default_value = Primitive::get_default_value(&sig);

        instance_fields.insert(name, default_value);
    }
}
