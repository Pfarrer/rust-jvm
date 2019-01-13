use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

use classfile::constants::Constant;
use vm::Vm;
use vm::signature::TypeSignature;
use vm::array::Array;
use vm::instance::Instance;
use vm::string_pool::StringPool;

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Boolean(bool),

    Byte(u8),

    Short(i16),
    Char(u16),

    Int(i32),
    Float(f32),

    Long(i64),
    Double(f64),

    Arrayref(Rc<RefCell<Array>>),
    Objectref(Rc<RefCell<Instance>>),
    ReturnAddress(u16),

    Null,
}

impl Primitive {
    pub fn get_default_value(sig: &TypeSignature) -> Primitive {
        match sig {
            &TypeSignature::Boolean => Primitive::Boolean(false),
            &TypeSignature::Byte => Primitive::Byte(0),
            &TypeSignature::Short => Primitive::Short(0),
            &TypeSignature::Char => Primitive::Char(0),
            &TypeSignature::Int => Primitive::Int(0),
            &TypeSignature::Float => Primitive::Float(0.0),
            &TypeSignature::Long => Primitive::Long(0),
            &TypeSignature::Double => Primitive::Double(0.0),
            &TypeSignature::Class(_) => Primitive::Null,
            &TypeSignature::Array(_) => Primitive::Null,
            _ => panic!("Default value not implemented for signature: {:?}", sig),
        }
    }

    pub fn from_constant(vm: &mut Vm, constant: &Constant) -> Primitive {
        match constant {
            &Constant::Integer(value) => Primitive::Int(value),
            &Constant::Float(value) => Primitive::Float(value),
            &Constant::Long(value) => Primitive::Long(value),
            &Constant::Double(value) => Primitive::Double(value),
            &Constant::String(ref value) => Primitive::Objectref(StringPool::intern(vm, value)),
            c => panic!("Unexpected constant found: {:?}", c),
        }
    }
}

impl fmt::Display for Primitive {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fn fmt_instance(instance: &Instance) -> String {
            format!("Objectref {}", instance.class_path)
        }

        let desc = match self {
            &Primitive::Int(_) => "Int".to_string(),
            &Primitive::Arrayref(_) => "Arrayref".to_string(),
            &Primitive::Objectref(ref rc) => fmt_instance(&rc.borrow()),
            &Primitive::Null => "Null".to_string(),
            _ => format!("Formatter not implemented, found: {:?}", self),
        };
        let _ = fmt.write_str(&desc);

        Ok(())
    }
}
