mod array;
mod class_hierarchy;
mod eval;
mod frame;
mod instance;
mod mem;
mod primitive;
mod utils;
mod vm_thread;

use crate::primitive::Primitive;
use crate::vm_thread::VmThread;
use class_hierarchy::ClassHierarchy;
use loader::Classloader;
use log::trace;
use mem::VmMem;
use model::prelude::*;
use simple_error::SimpleError;
use std::error::Error;
use std::sync::RwLock;

pub struct Vm {
    classloader: Box<dyn Classloader>,
    class_hierarchy: ClassHierarchy,
    mem: RwLock<VmMem>,
}

impl Vm {
    pub fn new(classloader: impl Classloader + 'static) -> Vm {
        Vm {
            classloader: Box::new(classloader),
            class_hierarchy: ClassHierarchy::new(),
            mem: RwLock::new(VmMem::new()),
        }
    }

    pub fn spawn_thread(&self) -> VmThread {
        VmThread::new(self)
    }

    pub fn load_and_clinit_class(&self, class_path: &String) -> Result<JvmClass, Box<dyn Error>> {
        let jvm_class = self
            .classloader
            .get_class(&class_path)
            .ok_or(SimpleError::new(format!("Class not found: {}", class_path)))?;

        trace!("Acquiring vm.mem write lock...");
        let mut mem = self.mem.write().unwrap();

        if !mem.static_pool_has_class(class_path) {
            mem.static_pool_insert_class(class_path.clone());

            // Search for static fields with a ConstantValue attribute and initialize accordingly
            for field in jvm_class.fields.iter() {
                if field.access_flags & JvmClass::ACC_STATIC > 0 {
                    // Static field found -> Set the types default value
                    mem.static_pool_set_class_field(
                        class_path,
                        field.name.clone(),
                        Primitive::get_default_value(&field.descriptor),
                    );

                    // Maybe there is a ConstantValue attribute, so check for that
                    for attr in field.attributes.iter() {
                        if let &ClassAttribute::ConstantValue(ref index) = attr {
                            let value = Primitive::from_constant(
                                self,
                                jvm_class.constants.get(*index as usize).unwrap(),
                            );

                            // Set value
                            mem.static_pool_set_class_field(class_path, field.name.clone(), value);
                        }
                    }
                }
            }

            //     // Initialize class if necessary
            //     if let Some(method) = utils::find_method_in_classfile(&jvm_class, &"<clinit>".to_string(), &"()V".to_string()) {
            //         debug!("Class {} not initialized and contains <clinit> -> executing now", class_path);
            //
            //         let code_attr = utils::find_code(&method).unwrap();
            //         let frame = Frame::new(code_attr.max_locals, class_path.clone(), "<clinit>".to_string(), "()V".to_string());
            //         self.execute_method(&jvm_class, &method, frame);
            //
            //         debug!("{}.<clinit> done", class_path);
            //     }
        }
        todo!()
        // jvm_class
    }
}
