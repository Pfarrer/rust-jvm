use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use vm::Vm;
use vm::primitive::Primitive;
use vm::array::Array;
use vm::instance::Instance;

pub struct StringPool {
    pool: HashMap<String, Rc<RefCell<Instance>>>,
}

impl StringPool {
    pub fn new() -> StringPool {
        let pool = HashMap::new();

        StringPool {
            pool,
        }
    }

    pub fn intern(vm: &mut Vm, value: &String) -> Rc<RefCell<Instance>> {
        // Load String class
        let class = vm.load_and_clinit_class(&"java/lang/String".to_string());

        // Create instance ...
        // THISISSHIT Should be located in the following lambda
        let mut instance = Instance::new(vm, class);

        // Get pooled String instance or create new instance
        vm.string_pool.pool.entry(value.clone()).or_insert_with(|| {
            // ... and set fields
            let count = value.encode_utf16().count();
            instance.fields.insert("count".to_string(), Primitive::Int(count as i32));

            let mut array = Array::new_primitive(count, 5);
            for (i, c) in value.encode_utf16().enumerate() {
                array.elements[i] = Primitive::Char(c);
            }
            instance.fields.insert("value".to_string(), Primitive::Arrayref(Rc::new(RefCell::new(array))));

            Rc::new(RefCell::new(instance))
        }).clone()
    }
}