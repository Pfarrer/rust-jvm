use crate::utils;
use crate::VmThread;
use class_constant_impl::ClassConstantAccessor;
use model::prelude::*;

pub fn eval(
    vm_thread: &mut VmThread,
    jvm_class: &JvmClass,
    code: &Vec<u8>,
    pc: u16,
) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    let (class_path, method_name, method_signature) = jvm_class
        .constants
        .get_methodref_or(index as usize)
        .unwrap();

    debug!(
        "invokespecial: {}.{}{}",
        class_path, method_name, method_signature
    );
    vm_thread.invoke_method(class_path, method_name, &method_signature.to_string(), true);

    Some(pc + 3)
}
