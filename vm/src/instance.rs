use crate::class_hierarchy::HierarchyIterator;
use crate::primitive::Primitive;
use crate::VmThread;
use model::class::JvmClass;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    pub class_path: String,
    pub fields: HashMap<String, Primitive>,
}

impl Instance {
    pub fn new(vm_thread: &mut VmThread, jvm_class: &JvmClass) -> Instance {
        let class_path = jvm_class.class_info.this_class.clone();

        let mut fields = HashMap::new();
        initialize_instance_fields(&jvm_class, &mut fields);

        let hierarchy_iter = HierarchyIterator::hierarchy_iter(vm_thread, &class_path);
        for (parent_class, _, _) in hierarchy_iter {
            initialize_instance_fields(&parent_class, &mut fields);
        }

        Instance { class_path, fields }
    }
}

fn initialize_instance_fields(
    jvm_class: &JvmClass,
    instance_fields: &mut HashMap<String, Primitive>,
) {
    for field in &jvm_class.fields {
        let default_value = Primitive::get_default_value(&field.descriptor);
        instance_fields.insert(field.name.clone(), default_value);
    }
}
