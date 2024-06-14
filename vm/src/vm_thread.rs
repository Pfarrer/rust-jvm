use crate::array::VmArrayImpl;
use crate::frame::VmFrameImpl;
use crate::instance::VmInstanceImpl;
use crate::primitive::VmPrimitiveImpl;
// use crate::eval::eval;
use crate::utils;
use crate::vm_mem::VmStaticPoolImpl;
use log::{debug, trace};
use model::prelude::*;
use parser::method_signature::parse_method_signature;
use std::cell::RefCell;
use std::rc::Rc;

pub trait VmTheadImpl<'a> {
    fn new(vm: &'a Vm, thread_name: String) -> VmThread<'a>;
    fn invoke_method(
        &mut self,
        class_path: &String,
        method_name: &String,
        method_signature: &String,
        is_instance: bool,
    );
    fn get_java_class_instance_for(&mut self, class_path: &String) -> Rc<RefCell<VmInstance>>;
    fn load_and_clinit_class(&mut self, class_path: &String) -> JvmClass;
}

impl<'a> VmTheadImpl<'a> for VmThread<'a> {
    fn new(vm: &'a Vm, thread_name: String) -> VmThread<'a> {
        // Create root frame
        let mut frame = VmFrame::new(0, "<root_frame>".to_string(), "<root_frame>".to_string(), "<root_frame>".to_string());

        // Add args array
        let args = VmArray::new_complex(0, "java/lang/String".to_string());
        let rc_args = Rc::new(RefCell::new(args));
        frame.stack_push(VmPrimitive::Arrayref(rc_args));
        let frame_stack = vec![frame];

        VmThread {
            vm,
            thread_name,
            frame_stack,
        }
    }

    fn invoke_method(
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

            native_method(self);
        } else {
            let code = utils::find_code(&method).unwrap();
            let frame = create_method_frame(
                self,
                class_path,
                method_name,
                method_signature,
                code.max_locals,
                is_instance,
            );

            execute_method(self, &class, code, frame);
        }
    }

    fn get_java_class_instance_for(&mut self, class_path: &String) -> Rc<RefCell<VmInstance>> {
        let jvm_class = self.load_and_clinit_class(&"java/lang/Class".to_string());

        // Create instance ...
        // THISISSHIT Should be located in the following lambda
        let mut instance = VmInstance::new(self, &jvm_class);

        let rc_interned_class_path = self.vm.mem.string_pool.intern(self, class_path);

        // Get pooled String instance or create new instance
        self.vm
            .mem
            .class_object_pool
            .pool
            .write()
            .unwrap()
            .entry(class_path.clone())
            .or_insert_with(|| {
                instance.fields.insert(
                    "name".to_string(),
                    VmPrimitive::Objectref(rc_interned_class_path),
                );
                // instance.fields.insert("classLoader".to_string(), VmPrimitive::Objectref(rc_class_path));

                Rc::new(RefCell::new(instance))
            })
            .clone()
    }

    fn load_and_clinit_class(&mut self, class_path: &String) -> JvmClass {
        let jvm_class = self
            .vm
            .classloader
            .get_class(&class_path)
            .expect(&format!("Class not found: {}", class_path));

        if !self.vm.mem.static_pool.has_class(class_path) {
            self.vm.mem.static_pool.insert_class(class_path.clone());
            clinit_class(self, jvm_class);
        }

        jvm_class.clone()
    }
}

fn execute_method(vm_thread: &mut VmThread, class: &JvmClass, code_attribute: &Code, frame: VmFrame) {
    trace!(
        "Executing {}.{}{}s in thread {} now...",
        frame.class_path,
        frame.method_name,
        frame.method_signature,
        vm_thread.thread_name,
    );

    vm_thread.frame_stack.push(frame);
    let mut pc = 0;

    loop {
        todo!()
        // match eval(self, class, &code_attribute.code, pc) {
        //     Some(new_pc) => pc = new_pc,
        //     None => break,
        // }
    }

    vm_thread.frame_stack.pop();
}

fn create_method_frame(
    vm_thread: &mut VmThread,
    class_path: &String,
    method_name: &String,
    method_signature: &String,
    max_locals: u16,
    is_instance: bool,
) -> VmFrame {
    let mut frame = VmFrame::new(
        max_locals,
        class_path.clone(),
        method_name.clone(),
        method_signature.clone(),
    );

    // Parse signature and move arguments from caller frame to callee frame
    {
        let parent_frame = vm_thread.frame_stack.last_mut().unwrap();

        let sig = parse_method_signature(method_signature).unwrap();
        let number_of_locals = sig.parameters.len() + if is_instance { 1 } else { 0 };
        for i in (0..number_of_locals).rev() {
            let arg = parent_frame.stack_pop();
            frame.locals_write(i, arg);
        }
    }

    frame
}

fn clinit_class(vm_thread: &mut VmThread, jvm_class: &JvmClass) {
    // Search for static fields with a ConstantValue attribute and initialize accordingly
    for field in jvm_class.fields.iter() {
        if field.access_flags.contains(FieldAccessFlag::Static) {
            // Static field found -> Set the types default value
            vm_thread.vm.mem.static_pool.set_class_field(
                &jvm_class.this_class,
                field.name.clone(),
                VmPrimitive::get_default_value(&field.descriptor),
            );

            // Maybe there is a ConstantValue attribute, so check for that
            for attr in field.attributes.iter() {
                if let &ClassAttribute::ConstantValue(ref constant) = attr {
                    let value = VmPrimitive::from_constant(vm_thread.vm, constant);

                    // Set value
                    vm_thread.vm.mem.static_pool.set_class_field(
                        &jvm_class.this_class,
                        field.name.clone(),
                        value,
                    );
                }
            }
        }
    }

    // Call <clinit> if it exists
    if utils::find_method_in_classfile(&jvm_class, "<clinit>", "()V").is_some() {
        debug!(
            "Class {} not initialized and contains <clinit> -> executing now",
            jvm_class.this_class
        );

        vm_thread.invoke_method(
            &jvm_class.this_class,
            &"<clinit>".to_string(),
            &"()V".to_string(),
            false,
        );

        debug!("{}.<clinit> done", jvm_class.this_class);
    }
}