use crate::{utils, vm_thread::VmTheadImpl, VmThread};
use model::prelude::*;

pub fn eval(
    vm_thread: &mut VmThread,
    jvm_class: &JvmClass,
    code: &Vec<u8>,
    pc: u16,
) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    match jvm_class.constants.get(index as usize).unwrap() {
        &ClassConstant::Methodref(ref class_path, ref method_name, ref method_signature) |
        &ClassConstant::InterfaceMethodref(ref class_path, ref method_name, ref method_signature)=> {
            debug!(
                "invokestatic: {}.{}{}",
                class_path, method_name, method_signature
            );
            vm_thread.invoke_method(
                class_path,
                method_name,
                &method_signature.to_string(),
                false,
            );
        },
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + 3)
}
