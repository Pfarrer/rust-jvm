use crate::frame::VmFrameImpl;
use crate::utils::find_static_field_value;
use crate::{utils, VmThread};
use model::prelude::*;

pub fn eval(
    vm_thread: &mut VmThread,
    jvm_class: &JvmClass,
    code: &Vec<u8>,
    pc: u16,
) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    match jvm_class.constants.get(index as usize).unwrap() {
        &ClassConstant::Fieldref(ref class_path, ref field_name, ref type_name) => {
            let value = find_static_field_value(vm_thread, class_path, field_name);
            trace!(
                "getstatic: {}.{}{} -> push value to stack",
                class_path,
                field_name,
                type_name
            );

            let frame = vm_thread.frame_stack.last_mut().unwrap();
            frame.stack_push(value);
        }
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + 3)
}
