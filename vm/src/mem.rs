use crate::Primitive;
use std::collections::HashMap;

pub struct VmMem {
    static_pool: HashMap<String, HashMap<String, Primitive>>,
    // pub string_pool: StringPool,
    // pub memory_pool: MemoryPool,
}

unsafe impl Send for VmMem {}
unsafe impl Sync for VmMem {}

impl VmMem {
    pub fn new() -> VmMem {
        VmMem {
            static_pool: HashMap::new(),
        }
    }

    pub fn static_pool_has_class(&self, class_path: &String) -> bool {
        self.static_pool.contains_key(class_path)
    }
    pub fn static_pool_insert_class(&mut self, class_path: String) {
        self.static_pool.insert(class_path, HashMap::new());
    }
    pub fn static_pool_set_class_field(
        &mut self,
        class_path: &String,
        field_name: String,
        value: Primitive,
    ) {
        self.static_pool
            .get_mut(class_path)
            .unwrap()
            .insert(field_name, value);
    }
}
