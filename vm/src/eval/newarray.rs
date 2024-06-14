use std::cell::RefCell;
use std::rc::Rc;

use model::prelude::*;

use crate::{array::VmArrayImpl, frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let count = frame.stack_pop_int();
    let atype = *code.get(pc as usize + 1).unwrap();

    if count < 0 {
        panic!("Not implemented: Throw NegativeArraySizeException");
    }

    trace!(
        "newarray: Create new VmArray of length {} and push Arrayref to stack",
        count
    );

    let array = VmArray::new_primitive(count as usize, atype);
    frame.stack_push(VmPrimitive::Arrayref(Rc::new(RefCell::new(array))));

    Some(pc + 2)
}
