use std::collections::HashMap;
use crate::Primitive;

pub struct VmMem {
    class_static_pool: HashMap<String, HashMap<String, Primitive>>,
    // pub string_pool: StringPool,
    // pub memory_pool: MemoryPool,
}

unsafe impl Send for VmMem {}
unsafe impl Sync for VmMem {}

impl VmMem {
    pub fn new() -> VmMem {
        VmMem {
            class_static_pool: HashMap::new(),
        }
    }

    pub fn class_static_pool_has_class(&self, class_path: &String) -> bool {
        self.class_static_pool.contains_key(class_path)
    }
}