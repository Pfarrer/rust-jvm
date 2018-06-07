use std::rc::Rc;
use std::cell::RefCell;

use vm::Vm;
use vm::classloader::Classloader;
use vm::primitive::Primitive;
use vm::instance::Instance;
use vm::utils;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "registerNatives" => noop(class_path, method_name, method_signature),
        "getPrimitiveClass" => get_primitive_class(vm, class_path, method_name, method_signature),
        "isArray" => is_array(vm, class_path, method_name, method_signature), // ()Z
        "getComponentType" => get_component_type(vm, class_path, method_name, method_signature), // ()Ljava/lang/Class;
        "isPrimitive" => is_primitive(vm, class_path, method_name, method_signature), // ()Z
        "getClassLoader0" => get_class_loader0(vm, class_path, method_name, method_signature), // ()Ljava/lang/ClassLoader;
        "desiredAssertionStatus0" => desired_assertion_status0(vm, class_path, method_name, method_signature), //(Ljava/lang/Class;)Z
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

fn noop(class_path: &String, method_name: &String, method_signature: &String) {
    // Nothing to do
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
}

fn is_array(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let frame = vm.frame_stack.last_mut().unwrap();

    let rc_class_instance = frame.stack_pop_objectref();
    let class_instance = rc_class_instance.borrow();

    let is_array = match class_instance.fields.get("name").unwrap() {
        &Primitive::Objectref(ref rc_string_instance) => {
            let string_instance = rc_string_instance.borrow();

            match string_instance.fields.get("value").unwrap() {
                &Primitive::Arrayref(ref rc_value_array) => {
                    let value_array = rc_value_array.borrow();
                    let first_char = &value_array.elements[0];

                    match first_char {
                        &Primitive::Char(ref code) => *code == 91u16, // 91 = [
                        _ => false,
                    }
                },
                p => panic!("Unexpected primitive: {:?}", p),
            }
        },
        p => panic!("Unexpected primitive: {:?}", p),
    };

    frame.stack_push(Primitive::Int(if is_array { 1 } else { 0 }));
}

/// getPrimitiveClass(Ljava/lang/String;)Ljava/lang/Class;
fn get_primitive_class(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let matched_class_path = {
        let frame = vm.frame_stack.last_mut().unwrap();

        let rc_instance = frame.stack_pop_objectref();
        let instance = rc_instance.borrow();
        let referenced_class_path = utils::get_java_string_value(&*instance);

        match referenced_class_path.as_ref() {
            "void" => "java/lang/Void".to_string(),
            "float" => "java/lang/Float".to_string(),
            "long" => "java/lang/Long".to_string(),
            "int" => "java/lang/Integer".to_string(),
            "byte" => "java/lang/Byte".to_string(),
            "double" => "java/lang/Double".to_string(),
            "short" => "java/lang/Short".to_string(),
            "char" => "java/lang/Character".to_string(),
            "boolean" => "java/lang/Boolean".to_string(),
            s => panic!("Class or type not whitelisted: {}", s),
        }
    };

    let rc_class = Classloader::get_class(vm, &matched_class_path);

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Objectref(rc_class));
}

/// ()Ljava/lang/Class;
fn get_component_type(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let component_class_name = {
        let frame = vm.frame_stack.last_mut().unwrap();

        let rc_class_instance = frame.stack_pop_objectref();
        let class_instance = rc_class_instance.borrow();

        match class_instance.fields.get("name").unwrap() {
            &Primitive::Objectref(ref rc_string_instance) => {
                let string_instance = rc_string_instance.borrow();

                match string_instance.fields.get("value").unwrap() {
                    &Primitive::Arrayref(ref rc_value_array) => {
                        let value_array = rc_value_array.borrow();
                        let second_char = &value_array.elements[1];

                        match second_char {
                            &Primitive::Char(ref code) => match *code {
                                67u16 => "java/lang/Character".to_string(), // 67 = C
                                c => panic!("Unexpected Char: {:?}", c),
                            },
                            p => panic!("Unexpected primitive: {:?}", p),
                        }
                    },
                    p => panic!("Unexpected primitive: {:?}", p),
                }
            },
            p => panic!("Unexpected primitive: {:?}", p),
        }
    };

    let rc_component_type = Classloader::get_class(vm, &component_class_name);

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Objectref(rc_component_type));
}

/// ()Z
fn is_primitive(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let frame = vm.frame_stack.last_mut().unwrap();

    let rc_class_instance = frame.stack_pop_objectref();
    let class_instance = rc_class_instance.borrow();

    let is_primi = match class_instance.fields.get("name").unwrap() {
        &Primitive::Objectref(ref rc_string_instance) => {
            let string_instance = rc_string_instance.borrow();
            let referenced_class_path = utils::get_java_string_value(&*string_instance);

            match referenced_class_path.as_ref() {
                "java/lang/Character" => true,
                "java/lang/Integer" => true,
                "java/lang/Boolean" => true,
                "java/lang/Long" => true,
                "java/lang/Double" => true,
                "java/lang/Float" => true,
                "java/lang/Short" => true,
                "java/lang/Byte" => true,
                p => panic!("Not implemented for: {:?}", p),
            }
        },
        p => panic!("Unexpected primitive: {:?}", p),
    };

    frame.stack_push(Primitive::Int(if is_primi { 1 } else { 0 }));
}

/// ()Ljava/lang/ClassLoader;
fn get_class_loader0(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    // Create classloader
    let classloader_classfile = vm.load_and_clinit_class(&"java/lang/ClassLoader".to_string());
    let classloader_instance = Instance::new(vm, classloader_classfile);

    let frame = vm.frame_stack.last_mut().unwrap();

    // Check whether the current class is a system class
    if frame.class_path == "java/lang/Class" {
        debug!("Providing Null as class loader of {}", frame.class_path);
        frame.stack_push(Primitive::Null);
    }
    else {
        debug!("Providing fake class loader of {}", frame.class_path);
        frame.stack_push(Primitive::Objectref(Rc::new(RefCell::new(classloader_instance))));
    }
}

///(Ljava/lang/Class;)Z
fn desired_assertion_status0(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let frame = vm.frame_stack.last_mut().unwrap();

    frame.stack_pop();
    frame.stack_push(Primitive::Int(1));
}
