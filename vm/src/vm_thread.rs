use crate::array::Array;
use crate::eval::eval;
use crate::frame::Frame;
use crate::instance::Instance;
use crate::primitive::Primitive;
use crate::utils;
use crate::Vm;
use lazy_static::lazy_static;
use log::{debug, trace};
use model::prelude::*;
use parser::method_signature::parse_method_signature;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

lazy_static! {
    static ref LOAD_AND_CLINIT_CLASS_MUTEX: Mutex<i32> = Mutex::new(0);
}

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
        let (class, method) = utils::find_method(self, class_path, method_name, method_signature);

        if method.access_flags.contains(MethodAccessFlag::Native) {
            let native_method = self
                .vm
                .classloader
                .get_native_method(&class, &method)
                .expect(
                    format!(
                        "No native method implementation found for {}.{}{}",
                        class.this_class, method.name, method.descriptor
                    )
                    .as_str(),
                );
            native_method();
        } else {
            let code_attr = utils::find_code(&method).unwrap();
            let frame = self.create_method_frame(
                class_path,
                method_name,
                method_signature,
                code_attr.max_locals,
                is_instance,
            );

            self.execute_method(&class, code_attr, frame);
        }
    }

    fn execute_method(&mut self, class: &JvmClass, code_attribute: &Code, frame: Frame) {
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

    pub fn get_java_class_instance_for(&mut self, class_path: &String) -> Rc<RefCell<Instance>> {
        let jvm_class = self.load_and_clinit_class(&"java/lang/Class".to_string());

        // Create instance ...
        // THISISSHIT Should be located in the following lambda
        let mut instance = Instance::new(self, &jvm_class);

        let rc_interned_class_path = self.vm.mem.string_pool.intern(self, class_path);

        // Get pooled String instance or create new instance
        self.vm
            .mem
            .class_object_pool
            .pool()
            .entry(class_path.clone())
            .or_insert_with(|| {
                instance.fields.insert(
                    "name".to_string(),
                    Primitive::Objectref(rc_interned_class_path),
                );
                // instance.fields.insert("classLoader".to_string(), Primitive::Objectref(rc_class_path));

                Rc::new(RefCell::new(instance))
            })
            .clone()
    }

    pub fn load_and_clinit_class(&mut self, class_path: &String) -> JvmClass {
        let jvm_class = self
            .vm
            .classloader
            .get_class(&class_path)
            .expect(&format!("Class not found: {}", class_path));

        let _lock = LOAD_AND_CLINIT_CLASS_MUTEX.lock().unwrap();
        if !self.vm.mem.static_pool.has_class(class_path) {
            self.vm.mem.static_pool.insert_class(class_path.clone());
            self.clinit_class(jvm_class);
        }

        jvm_class.clone()
    }

    fn clinit_class(&mut self, jvm_class: &JvmClass) {
        // Search for static fields with a ConstantValue attribute and initialize accordingly
        for field in jvm_class.fields.iter() {
            if field.access_flags.contains(FieldAccessFlag::Static) {
                // Static field found -> Set the types default value
                self.vm.mem.static_pool.set_class_field(
                    &jvm_class.this_class,
                    field.name.clone(),
                    Primitive::get_default_value(&field.descriptor),
                );

                // Maybe there is a ConstantValue attribute, so check for that
                for attr in field.attributes.iter() {
                    if let &ClassAttribute::ConstantValue(ref constant) = attr {
                        let value = Primitive::from_constant(self.vm, constant,);

                        // Set value
                        self.vm.mem.static_pool.set_class_field(
                            &jvm_class.this_class,
                            field.name.clone(),
                            value,
                        );
                    }
                }
            }
        }

        // Call <clinit> if it exists
        if let Some(_) = utils::find_method_in_classfile(&jvm_class, "<clinit>", "()V") {
            debug!(
                "Class {} not initialized and contains <clinit> -> executing now",
                jvm_class.this_class
            );

            self.invoke_method(
                &jvm_class.this_class,
                &"<clinit>".to_string(),
                &"()V".to_string(),
                false,
            );

            debug!("{}.<clinit> done", jvm_class.this_class);
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

            let sig = parse_method_signature(method_signature).unwrap();
            let number_of_locals = sig.parameters.len() + if is_instance { 1 } else { 0 };
            for i in (0..number_of_locals).rev() {
                let arg = parent_frame.stack_pop();
                frame.locals_write(i, arg);
            }
        }

        frame
    }
}
