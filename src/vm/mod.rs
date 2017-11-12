use classloader::Classloader;
use classfile::Classfile;
use classfile::constants::Constant;
use classfile::Method;

//struct Vm {
//
////use std::collections::HashMap;
//
//}

pub fn interpret(mut classloader: Classloader, main_class: String) {
    let main_method_name = "main".to_string();
    let main_method_signature = "([Ljava/lang/String;)V".to_string();

    let main_classfile = classloader.get_classfile(&main_class);

    match find_method(&main_classfile, &main_method_name, &main_method_signature) {
        Some(method) => {
            info!("Main method found!\n{:#?}", method);
            method
        },
        None => panic!("No main method found!"),
    };

    // TODO access_flags
    //        method.access_flags == classfile.ACC_PUBLIC | classfile.ACC_STATIC;
}

fn find_method<'a>(classfile: &'a Classfile, name: &String, signature: &String) -> Option<&'a Method> {
    classfile.methods.iter().find(| &method | {
        let correct_name = match classfile.constants.get(method.name_index) {
            Some(c) => match *c {
                Constant::Utf8(ref val) => name.eq(val),
                _ => panic!("Invalid class file"),
            },
            _ => panic!("Invalid class file"),
        };

        if !correct_name {
            return false;
        }

        match classfile.constants.get(method.descriptor_index) {
            Some(c) => match *c {
                Constant::Utf8(ref val) => signature.eq(val),
                _ => panic!("Invalid class file"),
            },
            _ => panic!("Invalid class file"),
        }
    })
}