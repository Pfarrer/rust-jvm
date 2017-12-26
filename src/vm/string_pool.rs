use std::collections::HashMap;

//use classfile::Classfile;
use vm::Vm;
use vm::instance::Instance;

pub struct StringPool {
    pool: HashMap<String, Box<Instance>>,
}

impl StringPool {
    pub fn new() -> StringPool {
        let pool = HashMap::new();

        StringPool {
            pool,
        }
    }

    pub fn intern(vm: &mut Vm, value: &String) -> Box<Instance> {
        // Load String class
        let class = vm.load_and_clinit_class(&"java/lang/String".to_string());

        // Get pooled String instance or create new instance
        vm.string_pool.pool.entry(value.clone()).or_insert_with(|| {
            let instance = Instance::new(class);

            Box::new(instance)
        }).clone()
    }
}