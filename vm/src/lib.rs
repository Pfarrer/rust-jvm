mod vm_thread;
mod frame;
mod primitive;
mod array;
mod instance;
mod class_hierarchy;
mod mem;
mod utils;

use mem::VmMem;
use std::error::Error;
use simple_error::SimpleError;
use loader::Classloader;
use crate::vm_thread::VmThread;
use class_hierarchy::ClassHierarchy;
use model::class::JvmClass;
use crate::primitive::Primitive;
use std::sync::{Mutex, RwLock};

pub struct Vm {
    classloader: Box<dyn Classloader>,
    class_hierarchy: ClassHierarchy,
    threads: Mutex<Vec<VmThread>>,
    mem: RwLock<VmMem>,
}

impl Vm {
    pub fn new(classloader: impl Classloader + 'static) -> Vm {
        Vm {
            classloader: Box::new(classloader),
            class_hierarchy: ClassHierarchy::new(),
            threads: Mutex::new(Vec::new()),
            mem: RwLock::new(VmMem::new()),
        }
    }

    pub fn spawn_thread(&mut self) -> &VmThread {
        let threads = self.threads.get_mut().unwrap();
        threads.push(VmThread::new());
        threads.last().unwrap()
    }

    pub fn load_and_clinit_class(&self, class_path: &String) -> Result<JvmClass, Box<dyn Error>> {
        let jvm_class = self.classloader.get_class(&class_path).ok_or(
            SimpleError::new(format!("Class not found: {}", class_path))
        )?;
        let mem = self.mem.write().unwrap();

        if !mem.class_static_pool_has_class(class_path) {
        //     self.class_statics.insert(class_path.clone(), HashMap::new());

            // Search for static fields with a ConstantValue attribute and initialize accordingly
            for field in jvm_class.fields.iter() {
                let field_name = utils::get_utf8_value(&jvm_class, field.name_index as usize);
        //         let type_signature = utils::get_type_signature(&jvm_class, field.descriptor_index as usize);
        //
        //         if field.access_flags & JvmClass::ACC_STATIC > 0 {
        //             // Static field found -> Set the types default value
        //             self.class_statics.get_mut(class_path).unwrap()
        //                 .insert(field_name.clone(), Primitive::get_default_value(&type_signature));
        //
        //             // Maybe there is a ConstantValue attribute, so check for that
        //             for attr in field.attributes.iter() {
        //                 if let &Attribute::ConstantValue(ref index) = attr {
        //                     let value = Primitive::from_constant(self, jvm_class.constants.get(*index as usize).unwrap());
        //
        //                     // Set value
        //                     self.class_statics.get_mut(class_path).unwrap()
        //                         .insert(field_name.clone(), value);
        //                 }
        //             }
        //         }
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
