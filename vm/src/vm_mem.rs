use model::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{RwLock, RwLockWriteGuard};

use crate::array::VmArrayImpl;
use crate::instance::VmInstanceImpl;
use crate::vm_thread::VmTheadImpl;

pub trait VmMemImpl {
    fn new() -> VmMem;
}

impl VmMemImpl for VmMem {
    fn new() -> VmMem {
        VmMem {
            static_pool: VmStaticPool::new(),
            string_pool: VmStringPool::new(),
            class_object_pool: VmClassObjectPool::new(),
        }
    }
}

pub trait VmStaticPoolImpl {
    fn new() -> VmStaticPool;
    fn has_class(&self, class_path: &String) -> bool;
    fn insert_class(&self, class_path: String);
    fn set_class_field(&self, class_path: &String, field_name: String, value: VmPrimitive);
    fn get_class_field(&self, class_path: &String, field_name: &String) -> Option<VmPrimitive>;
}

impl VmStaticPoolImpl for VmStaticPool {
    fn new() -> VmStaticPool {
        VmStaticPool {
            pool: RwLock::new(HashMap::new()),
        }
    }

    fn has_class(&self, class_path: &String) -> bool {
        self.pool.read().unwrap().contains_key(class_path)
    }

    fn insert_class(&self, class_path: String) {
        self.pool
            .write()
            .unwrap()
            .insert(class_path, HashMap::new());
    }

    fn set_class_field(&self, class_path: &String, field_name: String, value: VmPrimitive) {
        self.pool
            .write()
            .unwrap()
            .get_mut(class_path)
            .unwrap()
            .insert(field_name, value);
    }

    fn get_class_field(&self, class_path: &String, field_name: &String) -> Option<VmPrimitive> {
        self.pool
            .read()
            .unwrap()
            .get(class_path)
            .and_then(|fields_map| fields_map.get(field_name))
            .map(|p| p.clone())
    }
}

pub trait VmStringPoolImpl {
    fn new() -> VmStringPool;
    fn intern(&self, thread: &mut VmThread, string: &String) -> Rc<RefCell<VmInstance>>;
}

impl VmStringPoolImpl for VmStringPool {
    fn new() -> VmStringPool {
        VmStringPool {
            pool: RwLock::new(HashMap::new()),
        }
    }

    fn intern(&self, thread: &mut VmThread, string: &String) -> Rc<RefCell<VmInstance>> {
        let jvm_class = thread.load_and_clinit_class(&"java/lang/String".to_string());
        let mut instance = VmInstance::new(thread, &jvm_class);

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
                    .insert("count".to_string(), VmPrimitive::Int(count as i32));

                let mut array = VmArray::new_primitive(count, 5);
                for (i, c) in string.encode_utf16().enumerate() {
                    array.elements[i] = VmPrimitive::Char(c);
                }
                let rc_array = Rc::new(RefCell::new(array));
                instance
                    .fields
                    .insert("value".to_string(), VmPrimitive::Arrayref(rc_array));

                Rc::new(RefCell::new(instance))
            })
            .clone()
    }
}

trait VmClassObjectPoolImpl {
    fn new() -> VmClassObjectPool;
}

impl VmClassObjectPoolImpl for VmClassObjectPool {
    fn new() -> VmClassObjectPool {
        VmClassObjectPool {
            pool: RwLock::new(HashMap::new()),
        }
    }
}
