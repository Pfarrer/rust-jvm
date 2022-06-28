use std::rc::Rc;
use std::cell::RefCell;

use model::class::{ClassConstant, TypeSignature};
use crate::array::Array;
use crate::instance::Instance;
use crate::Vm;

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

    pub fn from_constant(vm: &Vm, constant: &ClassConstant) -> Primitive {
        match constant {
            &ClassConstant::Integer(value) => Primitive::Int(value),
            &ClassConstant::Float(value) => Primitive::Float(value),
            &ClassConstant::Long(value) => Primitive::Long(value),
            &ClassConstant::Double(value) => Primitive::Double(value),
            &ClassConstant::String(ref value) => todo!(), //Primitive::Objectref(StringPool::intern(vm, value)),
            c => panic!("Unexpected constant found: {:?}", c),
        }
    }
}
