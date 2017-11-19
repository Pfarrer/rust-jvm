mod classloader;
mod utils;
mod eval;

use classfile::Classfile;
use classfile::Method;
use vm::classloader::Classloader;

pub struct Vm {
    classloader: Classloader,
}

impl Vm {

    pub fn new(class_paths: Vec<String>) -> Vm {
        let classloader = Classloader::new(class_paths);

        Vm {
            classloader,
        }
    }

    pub fn invoke_static(&mut self, class_path: &String, method_name: &String, method_signature: &String) {
        let class = self.classloader.get_class(self, &class_path);
        let method = utils::find_method(&class, &method_name, &method_signature)
            .unwrap_or_else(|| panic!("Method not found: {}.{}{}", class_path, method_name, method_signature));

        // TODO access_flags
        //        method.access_flags == classfile.ACC_PUBLIC | classfile.ACC_STATIC;

        trace!("invoke_static {}.{}{}", class_path, method_name, method_signature);
        self.invoke_method(&class, method)
    }

    fn invoke_method(&mut self, class: &Classfile, method: &Method) {
        let code_attr = utils::find_code(method).unwrap();
        trace!("{:#?}", code_attr);

        let mut frame = Frame::new(code_attr.max_locals as usize, code_attr.max_stack as usize, &mut self.classloader);
        let mut pc = 0;

        loop {
            match eval::eval(self, class, &code_attr.code, pc, &mut frame) {
                Some(new_pc) => pc = new_pc,
                None => break,
            }
        }

        trace!("invoke_static finished");
    }
}

//use std::collections::HashMap;

pub struct Frame<'a> {

    locals: Vec<u32>,
    stack: Vec<u32>,
    classloader: &'a mut Classloader,

}

impl <'a> Frame<'a> {

    fn new(max_locals: usize, max_stack: usize, classloader: &mut Classloader) -> Frame {
        Frame {
            locals: Vec::with_capacity(max_locals),
            stack: Vec::with_capacity(max_stack),
            classloader,
        }
    }

}