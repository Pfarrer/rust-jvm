use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let index = (*code.get(pc as usize).unwrap() as i32) - 3;
    trace!("iconst_{}: Pushing constant Int {} to stack", index, index);

    vm_thread
        .frame_stack
        .last_mut()
        .unwrap()
        .stack_push(VmPrimitive::Int(index));

    Some(pc + 1)
}
