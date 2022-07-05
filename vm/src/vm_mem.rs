use crate::array::Array;
use crate::instance::Instance;
use crate::{Primitive, VmThread};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{RwLock, RwLockWriteGuard};

pub struct VmMem {
    pub static_pool: StaticPool,
    pub string_pool: StringPool,

    // Object pool for java/lang/Class instances
    pub class_object_pool: ClassObjectPool,
    // pub memory_pool: MemoryPool,
}

unsafe impl Send for VmMem {}
unsafe impl Sync for VmMem {}

impl VmMem {
    pub fn new() -> VmMem {
        VmMem {
            static_pool: StaticPool::new(),
            string_pool: StringPool::new(),
            class_object_pool: ClassObjectPool::new(),
        }
    }
}

pub struct StaticPool {
    pool: RwLock<HashMap<String, HashMap<String, Primitive>>>,
}
impl StaticPool {
    fn new() -> StaticPool {
        StaticPool {
            pool: RwLock::new(HashMap::new()),
        }
    }

    pub fn has_class(&self, class_path: &String) -> bool {
        self.pool.read().unwrap().contains_key(class_path)
    }
    pub fn insert_class(&self, class_path: String) {
        self.pool
            .write()
            .unwrap()
            .insert(class_path, HashMap::new());
    }
    pub fn set_class_field(&self, class_path: &String, field_name: String, value: Primitive) {
        self.pool
            .write()
            .unwrap()
            .get_mut(class_path)
            .unwrap()
            .insert(field_name, value);
    }
    pub fn get_class_field(&self, class_path: &String, field_name: &String) -> Option<Primitive> {
        self.pool
            .read()
            .unwrap()
            .get(class_path)
            .and_then(|fields_map| fields_map.get(field_name))
            .map(|p| p.clone())
    }
}

pub struct StringPool {
    pool: RwLock<HashMap<String, Rc<RefCell<Instance>>>>,
}
impl StringPool {
    fn new() -> StringPool {
        StringPool {
            pool: RwLock::new(HashMap::new()),
        }
    }

    pub fn intern(&self, vm_thread: &mut VmThread, string: &String) -> Rc<RefCell<Instance>> {
        let jvm_class = vm_thread.load_and_clinit_class(&"java/lang/String".to_string());
        let mut instance = Instance::new(vm_thread, &jvm_class);

        // Get pooled String instance or create new instance
        self.pool
            .write()
            .unwrap()
            .entry(string.clone())
            .or_insert_with(|| {
                // ... and set fields
                let count = string.encode_utf16().count();
                instance
                    .fields
                    .insert("count".to_string(), Primitive::Int(count as i32));

                let mut array = Array::new_primitive(count, 5);
                for (i, c) in string.encode_utf16().enumerate() {
                    array.elements[i] = Primitive::Char(c);
                }
                let rc_array = Rc::new(RefCell::new(array));
                instance
                    .fields
                    .insert("value".to_string(), Primitive::Arrayref(rc_array));

                Rc::new(RefCell::new(instance))
            })
            .clone()
    }
}

pub struct ClassObjectPool {
    pool: RwLock<HashMap<String, Rc<RefCell<Instance>>>>,
}
impl ClassObjectPool {
    fn new() -> ClassObjectPool {
        ClassObjectPool {
            pool: RwLock::new(HashMap::new()),
        }
    }

    pub fn pool(&self) -> RwLockWriteGuard<HashMap<String, Rc<RefCell<Instance>>>> {
        self.pool.write().unwrap()
    }
}
