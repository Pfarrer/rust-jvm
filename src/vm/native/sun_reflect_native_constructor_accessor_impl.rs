use std::cell::RefCell;
use std::rc::Rc;
use vm::Vm;
use vm::primitive::Primitive;
use vm::instance::Instance;
use vm::utils;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "newInstance0" => new_instance0(vm, class_path, method_name, method_signature), // (Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object; 
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

/// (Ljava/lang/reflect/Constructor;[Ljava/lang/Object;)Ljava/lang/Object;
fn new_instance0(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

   let (class_name_to_create, constructor_signature) = {
        let frame = vm.frame_stack.last_mut().unwrap();
        let rc_paraneters_array = frame.stack_pop();
        let rc_instance = frame.stack_pop_objectref();
        let constructor_instance = rc_instance.borrow();

        assert_eq!("java/lang/reflect/Constructor", constructor_instance.class_path);
        assert_eq!(Primitive::Null, rc_paraneters_array); // Parameters are not implemented yet

        let class_instance = match constructor_instance.fields.get("clazz").unwrap() {
            Primitive::Objectref(ref instance) => instance.clone(),
            _ => panic!("Expected Objectref")
        };
        let class_name_to_create = match class_instance.borrow().fields.get("name").unwrap() {
            Primitive::Objectref(ref instance) => utils::get_java_string_value(&*instance.borrow()),
            _ => panic!("Expected Objectref")
        };

        let constructor_signature = match constructor_instance.fields.get("signature").unwrap() {
            Primitive::Objectref(ref instance) => utils::get_java_string_value(&*instance.borrow()),
            _ => panic!("Expected Objectref")
        };

        (class_name_to_create, constructor_signature)
   };

    trace!("Going to create instance of class {} using constructor {}", class_name_to_create, constructor_signature);
    panic!("{}", class_name_to_create);
let class_name_to_create = "sun/nio/cs/UTF_8".to_string();

    let class_to_create = vm.load_and_clinit_class(&class_name_to_create);
    let instance = Instance::new(vm, class_to_create);
    let primitive = Primitive::Objectref(Rc::new(RefCell::new(instance)));
    
    // Push uninitialized instance to stack for upcoming invocation of the constructor
    {
        let frame = vm.frame_stack.last_mut().unwrap();
        frame.stack_push(primitive.clone());
    }

    utils::invoke_method(vm, &class_name_to_create, &"<init>".to_string(), &constructor_signature, true);

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(primitive.clone());
}