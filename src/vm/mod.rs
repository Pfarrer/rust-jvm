use classfile;
use classfile::attributes::Attribute;
use classfile::Classfile;
use classfile::Method;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use vm::array::Array;
use vm::class_hierarchy::ClassHierarchy;
use vm::classloader::Classloader;
use vm::frame::Frame;
use vm::primitive::Primitive;
use vm::string_pool::StringPool;

mod classloader;
mod class_hierarchy;
mod utils;
mod signature;
mod primitive;
mod array;
mod instance;
mod frame;
mod eval;
mod native;
mod string_pool;

const MAIN_METHOD_NAME: &str = "main";
const MAIN_METHOD_SIGNATURE: &str = "([Ljava/lang/String;)V";

// CONT: sun.nio.cs.FastCharsetProvider line 76

pub struct Vm {
    classloader: Classloader,
    class_hierarchy: ClassHierarchy,
    frame_stack: Vec<Frame>,
    pub class_statics: HashMap<String, HashMap<String, Primitive>>,
    pub string_pool: StringPool,
}

impl Vm {
    pub fn new(class_paths: Vec<String>) -> Vm {
        let classloader = Classloader::new(class_paths);
        let class_hierarchy = ClassHierarchy::new();
        let frame_stack = Vec::new();
        let class_statics = HashMap::new();
        let string_pool = StringPool::new();

        Vm {
            classloader,
            class_hierarchy,
            frame_stack,
            class_statics,
            string_pool,
        }
    }

    pub fn invoke_main(&mut self, class_path: &String) {
        // Add args array
        let args = Array::new_complex(0, "java/lang/String".to_string());
        let rc_args = Rc::new(RefCell::new(args));

        let mut frame = Frame::new("<root_frame>".to_string(), "<root_frame>".to_string(), "<root_frame>".to_string());
        frame.stack_push(Primitive::Arrayref(rc_args));
        self.frame_stack.push(frame);

        utils::invoke_method(self, class_path, &MAIN_METHOD_NAME.to_string(), &MAIN_METHOD_SIGNATURE.to_string(), false);
    }

    pub fn execute_method(&mut self, class: &Classfile, method: &Method, frame: Frame) {
        self.frame_stack.push(frame);

        let code_attr = utils::find_code(method).unwrap();
        let mut pc = 0;

        loop {
            match eval::eval(self, class, &code_attr.code, pc) {
                Some(new_pc) => pc = new_pc,
                None => break,
            }
        }

        self.frame_stack.pop();
    }

    pub fn load_and_clinit_class(&mut self, class_path: &String) -> Classfile {
        let classfile = self.classloader.get_classfile(&class_path);

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
            if let Some(method) = utils::find_method_in_classfile(&classfile, &"<clinit>".to_string(), &"()V".to_string()) {
                debug!("Class {} not initialized and contains <clinit> -> executing now", class_path);

                let frame = Frame::new(class_path.clone(), "<clinit>".to_string(), "()V".to_string());
                self.execute_method(&classfile, &method, frame);

                debug!("{}.<clinit> done", class_path);
            }
        }

        classfile
    }
}
