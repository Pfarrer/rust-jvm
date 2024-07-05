use crate::{vm_mem::VmStringPoolImpl, vm_thread::VmTheadImpl, Vm};
use model::prelude::*;

pub trait VmPrimitiveImpl {
    fn get_default_value(sig: &TypeSignature) -> VmPrimitive;
    fn from_constant(_vm: &Vm, constant: &ClassConstant) -> VmPrimitive;
}

impl VmPrimitiveImpl for VmPrimitive {
    fn get_default_value(sig: &TypeSignature) -> VmPrimitive {
        match sig {
            &TypeSignature::Boolean => VmPrimitive::Boolean(false),
            &TypeSignature::Byte => VmPrimitive::Byte(0),
            &TypeSignature::Short => VmPrimitive::Short(0),
            &TypeSignature::Char => VmPrimitive::Char(0),
            &TypeSignature::Int => VmPrimitive::Int(0),
            &TypeSignature::Float => VmPrimitive::Float(0.0),
            &TypeSignature::Long => VmPrimitive::Long(0),
            &TypeSignature::Double => VmPrimitive::Double(0.0),
            &TypeSignature::Class(_) => VmPrimitive::Null,
            &TypeSignature::Array(_) => VmPrimitive::Null,
            _ => panic!("Default value not implemented for signature: {:?}", sig),
        }
    }

    fn from_constant(vm: &Vm, constant: &ClassConstant) -> VmPrimitive {
        match constant {
            &ClassConstant::Integer(value) => VmPrimitive::Int(value),
            &ClassConstant::Float(value) => VmPrimitive::Float(value),
            &ClassConstant::Long(value) => VmPrimitive::Long(value),
            &ClassConstant::Double(value) => VmPrimitive::Double(value),
            &ClassConstant::String(ref value) => {
                let mut vm_thread = VmThread::new(vm, "string_pool.intern".to_string());
                VmPrimitive::Objectref(vm.mem.string_pool.intern(&mut vm_thread, value))
            },
            c => panic!("Unexpected constant found: {:?}", c),
        }
    }
}
