use std::cell::RefCell;
use crate::frame::Frame;
use std::rc::Rc;
use model::class::{ClassAttribute, ClassMethod, CodeAttribute, JvmClass};
use parser::parse_method_signature;
use crate::array::Array;
use crate::primitive::Primitive;
use crate::Vm;
use crate::utils;

pub struct VmThread {
    frame_stack: Vec<Frame>,
}

impl VmThread {
    pub fn new() -> VmThread {
        // Create root frame
        let mut frame = Frame::new(0, "<root_frame>".to_string(), "<root_frame>".to_string(), "<root_frame>".to_string());

        // Add args array
        let args = Array::new_complex(0, "java/lang/String".to_string());
        let rc_args = Rc::new(RefCell::new(args));
        frame.stack_push(Primitive::Arrayref(rc_args));

        VmThread {
            frame_stack: vec![frame],
        }
    }

    pub fn invoke_method(&mut self, vm: &Vm, class_path: &String, method_name: &String, method_signature: &String, is_instance: bool) {
        let (class, method) = utils::find_method(vm, class_path, method_name, method_signature);

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
                is_instance
            );

            self.execute_method(&class, &code_attr, frame);
        }
    }

    fn execute_method(&mut self, class: &JvmClass, code_attribute: &CodeAttribute, frame: Frame) {
        self.frame_stack.push(frame);
        let mut pc = 0;

        loop {
            match eval::eval(self, class, &code_attribute.code, pc) {
                Some(new_pc) => pc = new_pc,
                None => break,
            }
        }

        self.frame_stack.pop();
    }

    fn create_method_frame(
        &mut self,
        class_path: &String,
        method_name: &String,
        method_signature: &String,
        max_locals: u16,
        is_instance: bool
    ) -> Frame {
        let mut frame = Frame::new(
            max_locals,
            class_path.clone(),
            method_name.clone(),
            method_signature.clone()
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