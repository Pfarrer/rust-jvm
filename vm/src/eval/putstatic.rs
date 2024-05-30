use crate::{utils, VmThread};
use model::class::*;

pub fn eval(
    vm_thread: &mut VmThread,
    jvm_class: &JvmClass,
    code: &Vec<u8>,
    pc: u16,
) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    match jvm_class.constants.0.get(index as usize).unwrap() {
        &ClassConstant::Fieldref(ref class_path, ref field_name, _) => {
            // Initialize class
            vm_thread.load_and_clinit_class(class_path);

            // Pop value and push to statics
            let frame = vm_thread.frame_stack.last_mut().unwrap();
            let value = frame.stack_pop();
            trace!(
                "putstatic: Popped value from stack and store it in {}.{}",
                class_path,
                field_name
            );

            vm_thread
                .vm
                .mem
                .static_pool
                .set_class_field(class_path, field_name.clone(), value);
        }
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + 3)
}
