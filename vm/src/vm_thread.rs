use crate::array::Array;
use crate::eval::eval;
use crate::frame::Frame;
use crate::primitive::Primitive;
use crate::{utils, VmMem};
use crate::Vm;
use model::class::{ClassAttribute, CodeAttribute, JvmClass};
use parser::parse_method_signature;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::RwLockWriteGuard;
use log::{debug, trace};

pub struct VmThread<'a> {
    pub(crate) vm: &'a Vm,
    pub(crate) thread_name: String,
    pub(crate) frame_stack: Vec<Frame>,
}

impl<'a> VmThread<'a> {
    pub fn new(vm: &'a Vm, thread_name: String) -> VmThread<'a> {
        // Create root frame
        let mut frame = Frame::new(
            0,
            "<root_frame>".to_string(),
            "<root_frame>".to_string(),
            "<root_frame>".to_string(),
        );

        // Add args array
        let args = Array::new_complex(0, "java/lang/String".to_string());
        let rc_args = Rc::new(RefCell::new(args));
        frame.stack_push(Primitive::Arrayref(rc_args));

        VmThread {
            vm,
            thread_name,
            frame_stack: vec![frame],
        }
    }

    pub fn invoke_method(
        &mut self,
        class_path: &String,
        method_name: &String,
        method_signature: &String,
        is_instance: bool,
    ) {
        let (class, method) =
            utils::find_method(self, class_path, method_name, method_signature);

        if method.access_flags & JvmClass::ACC_NATIVE > 0 {
            todo!()
            // let resolved_class_path = get_class_path(&class);
            // native::invoke(vm, &resolved_class_path, method_name, method_signature);
        } else {
            let code_attr = utils::find_code(&method).unwrap();
            let frame = self.create_method_frame(
                class_path,
                method_name,
                method_signature,
                code_attr.max_locals,
                is_instance,
            );

            self.execute_method(&class, &code_attr, frame);
        }
    }

    fn execute_method(&mut self, class: &JvmClass, code_attribute: &CodeAttribute, frame: Frame) {
        trace!(
            "Executing {}.{}{}s in thread {} now...",
            frame.class_path,
            frame.method_name,
            frame.method_signature,
            self.thread_name,
        );

        self.frame_stack.push(frame);
        let mut pc = 0;

        loop {
            match eval(self, class, &code_attribute.code, pc) {
                Some(new_pc) => pc = new_pc,
                None => break,
            }
        }

        self.frame_stack.pop();
    }

    pub fn load_and_clinit_class(&mut self, class_path: &String) -> JvmClass {
        let jvm_class = self
            .vm
            .classloader
            .get_class(&class_path)
            .expect(&format!("Class not found: {}", class_path));

        trace!("Acquiring vm.mem write lock...");
        let mut mem = self.vm.mem.write().unwrap();

        if !mem.static_pool_has_class(class_path) {
            mem.static_pool_insert_class(class_path.clone());

            self.clinit_class(mem, jvm_class);
        }

        jvm_class.clone()
    }

    fn clinit_class(&mut self, mut mem:  RwLockWriteGuard<VmMem>, jvm_class: &JvmClass) {
        // Search for static fields with a ConstantValue attribute and initialize accordingly
        for field in jvm_class.fields.iter() {
            if field.access_flags & JvmClass::ACC_STATIC > 0 {
                // Static field found -> Set the types default value
                mem.static_pool_set_class_field(
                    &jvm_class.class_info.this_class,
                    field.name.clone(),
                    Primitive::get_default_value(&field.descriptor),
                );

                // Maybe there is a ConstantValue attribute, so check for that
                for attr in field.attributes.iter() {
                    if let &ClassAttribute::ConstantValue(ref index) = attr {
                        let value = Primitive::from_constant(
                            self.vm,
                            jvm_class.constants.get(*index as usize).unwrap(),
                        );

                        // Set value
                        mem.static_pool_set_class_field(&jvm_class.class_info.this_class, field.name.clone(), value);
                    }
                }
            }
        }

        // Call <clinit> if it exists
        if let Some(class_method) = utils::find_method_in_classfile(&jvm_class, "<clinit>", "()V") {
            debug!("Class {} not initialized and contains <clinit> -> executing now", jvm_class.class_info.this_class);

            let code_attribute = utils::find_code(&class_method).unwrap();
            let frame = Frame::new(code_attribute.max_locals, jvm_class.class_info.this_class.clone(), "<clinit>".to_string(), "()V".to_string());
            self.execute_method(&jvm_class, &code_attribute, frame);

            debug!("{}.<clinit> done", jvm_class.class_info.this_class);
        }
    }

    fn create_method_frame(
        &mut self,
        class_path: &String,
        method_name: &String,
        method_signature: &String,
        max_locals: u16,
        is_instance: bool,
    ) -> Frame {
        let mut frame = Frame::new(
            max_locals,
            class_path.clone(),
            method_name.clone(),
            method_signature.clone(),
        );

        // Parse signature and move arguments from caller frame to callee frame
        {
            let parent_frame = self.frame_stack.last_mut().unwrap();

            let sig = parse_method_signature(method_signature);
            let number_of_locals = sig.parameters.len() + if is_instance { 1 } else { 0 };
            for i in (0..number_of_locals).rev() {
                let arg = parent_frame.stack_pop();
                frame.locals_write(i, arg);
            }
        }

        frame
    }
}
