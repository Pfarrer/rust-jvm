use std::rc::Rc;
use std::cell::RefCell;

use classfile::constants::Constant;
use vm::signature::TypeSignature;
use vm::array::Array;
use vm::instance::Instance;

#[derive(Debug, Clone)]
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

    pub fn from_constant(constant: &Constant) -> Primitive {
        match constant {
            &Constant::Long(value) => Primitive::Long(value),
            &Constant::Integer(value) => Primitive::Int(value),

//                                    float	CONSTANT_Float
//                                    double	CONSTANT_Double
//                                    int, short, char, byte, boolean	CONSTANT_Integer
//                                String	CONSTANT_String
            c => panic!("Unexpected constant found: {:?}", c),
        }
    }
}
