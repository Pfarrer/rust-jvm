use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let value: i64 = *code.get(pc as usize).unwrap() as i64 - 9;

    trace!("lconst_{}: Pushing {}L to stack", value, value);
    vm_thread
        .frame_stack
        .last_mut()
        .unwrap()
        .stack_push(VmPrimitive::Long(value));

    Some(pc + 1)
}
