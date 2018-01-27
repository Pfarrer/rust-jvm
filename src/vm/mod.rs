mod classloader;
mod utils;
mod signature;
mod primitive;
mod array;
mod instance;
mod frame;
mod eval;
mod native;
mod string_pool;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use classfile;
use classfile::Classfile;
use classfile::Method;
use classfile::attributes::Attribute;
use vm::classloader::Classloader;
use vm::frame::Frame;
use vm::primitive::Primitive;
use vm::array::Array;
use vm::string_pool::StringPool;

const MAIN_METHOD_NAME: &str = "main";
const MAIN_METHOD_SIGNATURE: &str = "([Ljava/lang/String;)V";

pub struct Vm {
    classloader: Classloader,
    pub class_statics: HashMap<String, HashMap<String, Primitive>>,
    pub string_pool: StringPool,
}

impl Vm {
    pub fn new(class_paths: Vec<String>) -> Vm {
        let classloader = Classloader::new(class_paths);
        let class_statics = HashMap::new();
        let string_pool = StringPool::new();

        Vm {
            classloader,
            class_statics,
            string_pool,
        }
    }

    pub fn invoke_main(&mut self, class_path: &String) {
        let mut frame = Frame::new(class_path.clone(), MAIN_METHOD_NAME.to_string(), MAIN_METHOD_SIGNATURE.to_string());

        // Add args array
        let args = Array::new_complex(0, "java/lang/String".to_string());
        let rc_args = Rc::new(RefCell::new(args));
        frame.stack_push(Primitive::Arrayref(rc_args));

        self.invoke_static(class_path,
                           &MAIN_METHOD_NAME.to_string(),
                           &MAIN_METHOD_SIGNATURE.to_string(),
                           &mut frame);
    }

    pub fn invoke_static(&mut self, class_path: &String, method_name: &String, method_signature: &String, parent_frame: &mut Frame) {
        let class = self.load_and_clinit_class(class_path);
        let method = utils::find_method(&class, &method_name, &method_signature)
            .unwrap_or_else(|| panic!("Method not found: {}.{}{}", class_path, method_name, method_signature));

        // TODO access_flags: method.access_flags == classfile.ACC_PUBLIC | classfile.ACC_STATIC;

        if method.access_flags & classfile::ACC_NATIVE > 0 {
            native::invoke(self, parent_frame, &class, method, class_path, method_name, method_signature);
        } else if method.access_flags & classfile::ACC_ABSTRACT > 0 {
            panic!("{}.{}{} cannot be executed since it is abstract.", class_path, method_name, method_signature);
        } else {
            let mut frame = Frame::new(class_path.clone(), method_name.clone(), method_signature.clone());

            // Parse signature and move arguments from caller frame to callee frame
            let sig = signature::parse_method(method_signature);
            for i in (0..sig.parameters.len()).rev() {
                let arg = parent_frame.stack_pop();

                trace!(" - Write argument no. {} to inner frame: {:?}", i, arg);
                frame.locals_write(i, arg);
            }

            self.execute_method(&class, &method, &mut frame, parent_frame);
        }
    }

    pub fn execute_method(&mut self, class: &Classfile, method: &Method, frame: &mut Frame, parent_frame: &mut Frame) {
        let code_attr = utils::find_code(method).unwrap();
        let mut pc = 0;

        loop {
            match eval::eval(self, class, &code_attr.code, pc, frame, parent_frame) {
                Some(new_pc) => pc = new_pc,
                None => break,
            }
        }
    }

    fn load_and_clinit_class(&mut self, class_path: &String) -> Classfile {
        let classfile = self.classloader.get_class(&class_path);

        if !self.class_statics.contains_key(class_path) {
            self.class_statics.insert(class_path.clone(), HashMap::new());

            // Search for static fields with a ConstantValue attribute and initialize accordingly
            for field in classfile.fields.iter() {
                let field_name = utils::get_utf8_value(&classfile, field.name_index as usize);
                let type_signature = utils::get_type_signature(&classfile, field.descriptor_index as usize);

                if field.access_flags & classfile::ACC_STATIC > 0 {
                    // Static field found -> Set the types default value
                    self.class_statics.get_mut(class_path).unwrap()
                        .insert(field_name.clone(), Primitive::get_default_value(&type_signature));

                    // Maybe there is a ConstantValue attribute, so check for that
                    for attr in field.attributes.iter() {
                        if let &Attribute::ConstantValue(ref index) = attr {
                            let value = Primitive::from_constant(self, classfile.constants.get(*index as usize).unwrap());

                            // Set value
                            self.class_statics.get_mut(class_path).unwrap()
                                .insert(field_name.clone(), value);
                        }
                    }
                }
            }

            // Initialize class if necessary
            if let Some(method) = utils::find_method(&classfile, &"<clinit>".to_string(), &"()V".to_string()) {
                trace!("Class {} not initialized and contains <clinit> -> executing now", class_path);

                let mut frame = Frame::new(class_path.clone(), "<clinit>".to_string(), "()V".to_string());
                let mut parent_frame = Frame::new(class_path.clone(), "<clinit>".to_string(), "()V".to_string());
                self.execute_method(&classfile, &method, &mut frame, &mut parent_frame);

                trace!("{}.<clinit> done", class_path);
            }
        }

        classfile
    }
}