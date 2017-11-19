use classfile::Classfile;
use classfile::constants::Constant;
use classfile::attributes;
use classfile::Method;

pub fn find_method<'a>(classfile: &'a Classfile, name: &String, signature: &String) -> Option<&'a Method> {
    classfile.methods.iter().find(| &method | {
        let correct_name = match classfile.constants.get(method.name_index).unwrap() {
            &Constant::Utf8(ref val) => name.eq(val),
            _ => panic!("Invalid class file"),
        };

        if !correct_name {
            return false;
        }

        match classfile.constants.get(method.descriptor_index).unwrap() {
            &Constant::Utf8(ref val) => signature.eq(val),
            _ => panic!("Invalid class file"),
        }
    })
}

pub fn find_code<'a>(method: &'a Method) -> Option<&'a attributes::CodeAttribute> {
    for attr in method.attributes.iter() {
        if let &attributes::Attribute::Code(ref code) = attr {
            return Some(code);
        }
    }

    None
}