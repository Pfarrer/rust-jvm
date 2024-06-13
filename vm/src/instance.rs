use crate::class_hierarchy::HierarchyIterator;
use model::prelude::*;
use std::collections::HashMap;

pub trait VmInstanceImpl {
    fn new(vm_thread: &mut VmThread, jvm_class: &JvmClass) -> VmInstance;
}

impl VmInstanceImpl for VmInstance {
    fn new(vm_thread: &mut VmThread, jvm_class: &JvmClass) -> VmInstance {
        let class_path = jvm_class.this_class.clone();

        let mut fields = HashMap::new();
        initialize_instance_fields(&jvm_class, &mut fields);

        let hierarchy_iter = HierarchyIterator::hierarchy_iter(vm_thread, &class_path);
        for (parent_class, _, _) in hierarchy_iter {
            initialize_instance_fields(&parent_class, &mut fields);
        }

        VmInstance { class_path, fields }
    }
}

fn initialize_instance_fields(
    jvm_class: &JvmClass,
    instance_fields: &mut HashMap<String, VmPrimitive>,
) {
    for field in &jvm_class.fields {
        let default_value = VmPrimitive::get_default_value(&field.descriptor);
        instance_fields.insert(field.name.clone(), default_value);
    }
}
