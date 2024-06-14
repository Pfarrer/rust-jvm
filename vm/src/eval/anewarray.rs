use std::cell::RefCell;
use std::rc::Rc;

use crate::{array::VmArrayImpl, frame::VmFrameImpl, utils, VmPrimitive, VmThread};
use model::prelude::*;

pub fn eval(
    vm_thread: &mut VmThread,
    jvm_class: &JvmClass,
    code: &Vec<u8>,
    pc: u16,
) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let count = frame.stack_pop_int();
    let index = utils::read_u16_code(code, pc);

    if count < 0 {
        panic!("Not implemented: Throw NegativeArraySizeException");
    }

    let class_path = match jvm_class.constants.get(index as usize).unwrap() {
        &ClassConstant::Class(ref class_path) => class_path.clone(),
        c => panic!("Unexpected constant ref: {:?}", c),
    };

    trace!(
        "anewarray: Create new VmArray of length {} and push Arrayref to stack",
        count
    );

    let array = VmArray::new_complex(count as usize, class_path);
    frame.stack_push(VmPrimitive::Arrayref(Rc::new(RefCell::new(array))));

    Some(pc + 3)
}
