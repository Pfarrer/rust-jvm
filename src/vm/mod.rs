mod classloader;
mod utils;
mod frame;
mod eval;
mod native;

use std::collections::HashMap;

use classfile;
use classfile::Classfile;
use classfile::Method;
use vm::classloader::Classloader;
use vm::frame::Frame;

const MAIN_METHOD_NAME: &str = "main";
const MAIN_METHOD_SIGNATURE: &str = "([Ljava/lang/String;)V";

pub struct Vm {
    classloader: Classloader,
    class_statics: HashMap<String, ()>,
}

impl Vm {
    pub fn new(class_paths: Vec<String>) -> Vm {
        let classloader = Classloader::new(class_paths);
        let class_statics = HashMap::new();

        Vm {
            classloader,
            class_statics,
        }
    }

    pub fn invoke_main(&mut self, class_path: &String) {
        let mut frame = Frame::new(None);
        self.invoke_static(class_path,
                           &MAIN_METHOD_NAME.to_string(),
                           &MAIN_METHOD_SIGNATURE.to_string(),
                           &mut frame);
    }

    pub fn invoke_static(&mut self, class_path: &String, method_name: &String, method_signature: &String, frame: &mut Frame) {
        let class = self.load_and_clinit_class(class_path);
        let method = utils::find_method(&class, &method_name, &method_signature)
            .unwrap_or_else(|| panic!("Method not found: {}.{}{}", class_path, method_name, method_signature));

        // TODO access_flags: method.access_flags == classfile.ACC_PUBLIC | classfile.ACC_STATIC;

        if method.access_flags & classfile::ACC_NATIVE > 0 {
            native::invoke(self, frame, &class, method, class_path, method_name, method_signature);
        } else if method.access_flags & classfile::ACC_ABSTRACT > 0 {
            panic!("{}.{}{} cannot be executed since it is abstract.", class_path, method_name, method_signature);
        } else {
            self.invoke_method(&class, &method, frame);
        }
    }

    fn invoke_method(&mut self, class: &Classfile, method: &Method, frame: &mut Frame) {
        let code_attr = utils::find_code(method).unwrap();
        trace!("{:#?}", code_attr);

        let mut pc = 0;

        loop {
            match eval::eval(self, class, &code_attr.code, pc, frame) {
                Some(new_pc) => pc = new_pc,
                None => break,
            }
        }
    }

    fn load_and_clinit_class(&mut self, class_path: &String) -> Classfile {
        let classfile = self.classloader.get_class(&class_path);

        if !self.class_statics.contains_key(class_path) {
            self.class_statics.insert(class_path.clone(), ());

            // Initialize class if necessary
            if let Some(method) = utils::find_method(&classfile, &"<clinit>".to_string(), &"()V".to_string()) {
                trace!("Class {} contains <clinit> -> executing now", class_path);

                let mut frame = Frame::new(None);
                self.invoke_method(&classfile, &method, &mut frame);

                trace!("{}.<clinit> done", class_path);

            }
        }

        classfile
    }
}