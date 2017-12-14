mod classloader;
mod utils;
mod signature;
mod primitive;
mod instance;
mod frame;
mod eval;
mod native;

use std::collections::HashMap;

use classfile;
use classfile::Classfile;
use classfile::Method;
use classfile::attributes::Attribute;
use classfile::constants::Constant;
use vm::classloader::Classloader;
use vm::frame::Frame;
use vm::primitive::Primitive;

const MAIN_METHOD_NAME: &str = "main";
const MAIN_METHOD_SIGNATURE: &str = "([Ljava/lang/String;)V";

pub struct Vm {
    classloader: Classloader,
    pub class_statics: HashMap<String, HashMap<String, Primitive>>,
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
        let mut frame = Frame::new();
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
            let mut frame = Frame::new();
            self.execute_method(&class, &method, &mut frame, parent_frame);
        }
    }

    pub fn execute_method(&mut self, class: &Classfile, method: &Method, frame: &mut Frame, parent_frame: &mut Frame) {
        let code_attr = utils::find_code(method).unwrap();
        trace!("{:#?}", code_attr);

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
                if field.access_flags & classfile::ACC_STATIC > 0 {
                    // Static field found -> check if there also is a ConstantValue attribute
                    for attr in field.attributes.iter() {
                        if let &Attribute::ConstantValue(ref index) = attr {
                            let value = match classfile.constants.get(*index as usize).unwrap() {
                                &Constant::Long(value) => Primitive::Long(value),
//                                    float	CONSTANT_Float
//                                    double	CONSTANT_Double
//                                    int, short, char, byte, boolean	CONSTANT_Integer
//                                String	CONSTANT_String
                                c => panic!("Unexpected constant found: {:?}", c),
                            };

                            // Set value
                            match classfile.constants.get(field.name_index as usize).unwrap() {
                                &Constant::Utf8(ref field_name) => {
                                    self.class_statics.get_mut(class_path).unwrap()
                                        .insert(field_name.clone(), value);
                                },
                                _ => panic!("Failed to get name of field from constant pool."),
                            }

                        }
                    }
                }
            }

            // Initialize class if necessary
            if let Some(method) = utils::find_method(&classfile, &"<clinit>".to_string(), &"()V".to_string()) {
                trace!("Class {} not initialized and contains <clinit> -> executing now", class_path);

                let mut frame = Frame::new();
                let mut parent_frame = Frame::new();
                self.execute_method(&classfile, &method, &mut frame, &mut parent_frame);

                trace!("{}.<clinit> done", class_path);
            }
        }

        classfile
    }
}