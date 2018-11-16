use classfile;
use std::cell::RefCell;
use std::rc::Rc;
use vm::array::Array;
use vm::classloader::Classloader;
use classfile::constants::Constant;
use vm::instance::Instance;
use vm::primitive::Primitive;
use vm::string_pool::StringPool;
use vm::utils;
use vm::Vm;
use vm::signature;
use vm::classfile::ACC_INTERFACE;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "registerNatives" => noop(class_path, method_name, method_signature),
        "getPrimitiveClass" => get_primitive_class(vm, class_path, method_name, method_signature),
        "isArray" => is_array(vm, class_path, method_name, method_signature), // ()Z
        "getComponentType" => get_component_type(vm, class_path, method_name, method_signature), // ()Ljava/lang/Class;
        "isPrimitive" => is_primitive(vm, class_path, method_name, method_signature), // ()Z
        "isInterface" => is_interface(vm, class_path, method_name, method_signature), // ()Z
        "getClassLoader0" => get_class_loader0(vm, class_path, method_name, method_signature), // ()Ljava/lang/ClassLoader;
        "desiredAssertionStatus0" => desired_assertion_status0(vm, class_path, method_name, method_signature), //(Ljava/lang/Class;)Z
        "getDeclaredFields0" => get_declared_fields0(vm, class_path, method_name, method_signature), // (Z)[Ljava/lang/reflect/Field;
        "getDeclaredConstructors0" => get_declared_constructors0(vm, class_path, method_name, method_signature), // (Z)[Ljava/lang/reflect/Constructor;
        "forName0" => for_name0(vm, class_path, method_name, method_signature), // (Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;
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
                }
                p => panic!("Unexpected primitive: {:?}", p),
            }
        }
        p => panic!("Unexpected primitive: {:?}", p),
    };

    frame.stack_push(Primitive::Int(if is_array { 1 } else { 0 }));
}

/// getPrimitiveClass(Ljava/lang/String;)Ljava/lang/Class;
fn get_primitive_class(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    // primitive_name will be something like "void" or "long" etc.
    let primitive_name = {
        let frame = vm.frame_stack.last_mut().unwrap();

        let rc_instance = frame.stack_pop_objectref();
        let instance = rc_instance.borrow();

        utils::get_java_string_value(&*instance)
    };

    let classfile = vm.load_and_clinit_class(&"java/lang/Class".to_string());
    let mut class_instance = Instance::new(vm, classfile);

    let rc_interned_name = StringPool::intern(vm, &primitive_name);
    class_instance.fields.insert("name".to_string(), Primitive::Objectref(rc_interned_name));

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Objectref(Rc::new(RefCell::new(class_instance))));
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
                    }
                    p => panic!("Unexpected primitive: {:?}", p),
                }
            }
            p => panic!("Unexpected primitive: {:?}", p),
        }
    };

    let rc_component_type = Classloader::get_class(vm, &component_class_name);

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Objectref(rc_component_type));
}

/// (Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;
fn for_name0(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let class_path = {
        let mut frame = vm.frame_stack.last_mut().unwrap();

        // Pop classloader instance
        frame.stack_pop_reference();

        let initialize = frame.stack_pop_boolean();
        if !initialize {
            panic!("Not implemented: initialize is set to false");
        }

        let rc_class_path_instance = frame.stack_pop_objectref();
        let class_path_instance = rc_class_path_instance.borrow();

        class_path_instance.class_path.clone()
    };

    let rc_class_instance = Classloader::get_class(vm, &class_path);

    trace!("Push Class<{}> instance to stack", class_path);

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Objectref(rc_class_instance));
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
                "character" => true,
                "char" => true, // character and char?
                "integer" => true,
                "int" => true,
                "boolean" => true,
                "long" => true,
                "double" => true,
                "float" => true,
                "short" => true,
                "byte" => true,
                "void" => true,
                _ => false,
            }
        }
        p => panic!("Unexpected primitive: {:?}", p),
    };

    frame.stack_push(Primitive::Boolean(is_primi));
}

/// ()Z
fn is_interface(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    let referenced_class_path = {
        let frame = vm.frame_stack.last_mut().unwrap();

        let rc_class_instance = frame.stack_pop_objectref();
        let class_instance = rc_class_instance.borrow();

        match class_instance.fields.get("name").unwrap() {
            &Primitive::Objectref(ref rc_string_instance) => {
                let string_instance = rc_string_instance.borrow();
                utils::get_java_string_value(&*string_instance)
            }
            p => panic!("Unexpected primitive: {:?}", p),
        }
    };

    let classfile = vm.load_and_clinit_class(&referenced_class_path);
    let is_interface = classfile.class_info.access_flags & ACC_INTERFACE > 0;

    trace!("{} is an interface? -> {}", referenced_class_path, is_interface);

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Boolean(is_interface));
}

/// (Z)[Ljava/lang/reflect/Field;
fn get_declared_fields0(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    // Get the last stack frame and pop the boolean value
    let public_only = {
        let mut frame = vm.frame_stack.last_mut().unwrap();
        frame.stack_pop_boolean()
    };

    // Get class_path based on the "name" field of the Class instance of the
    // last stack frame
    let class_path = {
        let mut frame = vm.frame_stack.last_mut().unwrap();
        let rc_instance = frame.stack_pop_objectref();
        let class_instance = rc_instance.borrow();

        let name_instance = class_instance.fields.get("name").unwrap();
        match name_instance {
            &Primitive::Objectref(ref rc_name) => utils::get_java_string_value(&rc_name.borrow()),
            _ => panic!("Unexpected instance found: {:?}", name_instance)
        }
    };

    let classfile = vm.load_and_clinit_class(&class_path);
    let field_classfile = vm.load_and_clinit_class(&"java/lang/reflect/Field".to_string());
    let field_instance_template = Instance::new(vm, field_classfile.clone());

    // Prepare a Java Field class instance per field found
    let field_instances: Vec<Primitive> = classfile.fields.iter()
        .filter(|field| {
            if public_only {
                classfile::ACC_PUBLIC & field.access_flags > 0
            } else { true }
        })
        .map(|field| {
            let mut field_instance = field_instance_template.clone();

            // This is guaranteed to be interned by the VM in the 1.4
            // reflection implementation
            let name = utils::get_utf8_value(&classfile, field.name_index as usize);

            let rc_interned_name = StringPool::intern(vm, &name);

            // name
            field_instance.fields.insert("name".to_string(), Primitive::Objectref(rc_interned_name));

            // modifiers
            field_instance.fields.insert("modifiers".to_string(), Primitive::Int(field.access_flags as i32));

            // signature
            let signature_string = match classfile.constants.get(field.descriptor_index as usize).unwrap() {
                &Constant::Utf8(ref signature_string) => signature_string.clone(),
                it => panic!("Expected Utf8 but found: {:?}", it),
            };
            let rc_interned_signature = StringPool::intern(vm, &signature_string);
            field_instance.fields.insert("signature".to_string(), Primitive::Objectref(rc_interned_signature));

            // type
            {
                let type_name = match signature::parse_field(&signature_string) {
                    signature::TypeSignature::Char => "char".to_string(),
                    signature::TypeSignature::Int => "int".to_string(),
                    signature::TypeSignature::Boolean => "boolean".to_string(),
                    signature::TypeSignature::Long => "long".to_string(),
                    signature::TypeSignature::Double => "double".to_string(),
                    signature::TypeSignature::Float => "float".to_string(),
                    signature::TypeSignature::Short => "short".to_string(),
                    signature::TypeSignature::Byte => "byte".to_string(),
                    signature::TypeSignature::Void => panic!("Field of type void?!"),
                    signature::TypeSignature::Class(ref class_path) => class_path.clone(),
                    signature::TypeSignature::Array(_) => signature_string.clone(),
                };

                let classfile = vm.load_and_clinit_class(&"java/lang/Class".to_string());
                let mut class_instance = Instance::new(vm, classfile);

                let rc_interned_type_name = StringPool::intern(vm, &type_name);
                class_instance.fields.insert("name".to_string(), Primitive::Objectref(rc_interned_type_name));

                // Finally, set created Class instance on the field instance
                field_instance.fields.insert("type".to_string(), Primitive::Objectref(Rc::new(RefCell::new(class_instance))));
            }

            // TODO not implemented fields:
//            private Class<?>            clazz;
//            private int                 slot;
//            // generic info repository; lazily initialized
//            private transient FieldRepository genericInfo;
//            private byte[]              annotations;
//            // Cached field accessor created without override
//            private FieldAccessor fieldAccessor;
//            // Cached field accessor created with override
//            private FieldAccessor overrideFieldAccessor;
//            // For sharing of FieldAccessors. This branching structure is
//            // currently only two levels deep (i.e., one root Field and
//            // potentially many Field objects pointing to it.)
//            //
//            // If this branching structure would ever contain cycles, deadlocks can
//            // occur in annotation code.
//            private Field               root;

            Primitive::Objectref(Rc::new(RefCell::new(field_instance)))
        })
        .collect();

    // Make a Java array with all these Field class instances as elements
    let mut fields_array = Array::new_complex(field_instances.len(), "java/lang/reflect/Field".to_string());
    fields_array.elements = field_instances;

    // Push the array to the stack and quit
    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Arrayref(Rc::new(RefCell::new(fields_array))));
    trace!("Pushed Arrayref to stack");
}

/// (Z)[Ljava/lang/reflect/Constructor;
fn get_declared_constructors0(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    // Get the last stack frame and pop the boolean value
    let public_only = {
        let mut frame = vm.frame_stack.last_mut().unwrap();
        frame.stack_pop_boolean()
    };

    // Get class_path based on the "name" field of the Class instance of the
    // last stack frame
    let class_path = {
        let frame = vm.frame_stack.last_mut().unwrap();
        let rc_instance = frame.stack_pop_objectref();
        let class_instance = rc_instance.borrow();

        let name_instance = class_instance.fields.get("name").unwrap();
        match name_instance {
            &Primitive::Objectref(ref rc_name) => utils::get_java_string_value(&rc_name.borrow()),
            _ => panic!("Unexpected instance found: {:?}", name_instance)
        }
    };

    let constructor_classfile = vm.load_and_clinit_class(&"java/lang/reflect/Constructor".to_string());
    let constructor_instance_template = Instance::new(vm, constructor_classfile.clone());

    let classfile = vm.load_and_clinit_class(&class_path);
    let constructor_instances: Vec<Primitive> = classfile.methods.iter()
        .filter(|method| {
            if public_only {
                classfile::ACC_PUBLIC & method.access_flags > 0
            } else { true }
        })
        .filter(|method| {
            let name = utils::get_utf8_value(&classfile, method.name_index as usize);
            name == "<init>"
        })
        .map(|method| {
            let mut method_instance = constructor_instance_template.clone();

            // modifiers
            method_instance.fields.insert("modifiers".to_string(), Primitive::Int(method.access_flags as i32));

            // signature
            let signature_string = match classfile.constants.get(method.descriptor_index as usize).unwrap() {
                &Constant::Utf8(ref signature_string) => signature_string.clone(),
                it => panic!("Expected Utf8 but found: {:?}", it),
            };
            let rc_interned_signature = StringPool::intern(vm, &signature_string);
            method_instance.fields.insert("signature".to_string(), Primitive::Objectref(rc_interned_signature));

            // Determine parameterTypes
            let parameter_types: Vec<Primitive> = signature::parse_method(&signature_string).parameters.iter()
                .map(|param| Classloader::get_class_by_type_signature(vm, param))
                .map(|rc| Primitive::Objectref(rc))
                .collect();
            let parameter_types_array = Array::new_complex_of(parameter_types, "java/lang/Class".to_string());
            method_instance.fields.insert("parameterTypes".to_string(), Primitive::Arrayref(Rc::new(RefCell::new(parameter_types_array))));

            Primitive::Objectref(Rc::new(RefCell::new(method_instance)))
        })
        .collect();

    // Make a Java array with all these Constructor class instances as elements
    let mut constructors_array = Array::new_complex(constructor_instances.len(), "java/lang/reflect/Constructor".to_string());
    constructors_array.elements = constructor_instances;

    // Push the array to the stack and quit
    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Arrayref(Rc::new(RefCell::new(constructors_array))));
    trace!("Pushed Arrayref to stack");
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
    } else {
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
