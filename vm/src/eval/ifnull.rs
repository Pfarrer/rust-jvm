use crate::utils;
use crate::{VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_reference();
    match value {
        VmPrimitive::Null => {
            trace!("ifnull: Popped Null from stack -> branching");

            let branchoffset = utils::read_i16_code(code, pc);
            let target_pc: u16 = (pc as i16 + branchoffset) as u16;

            Some(target_pc)
        }
        _ => {
            trace!("ifnull: Popped Reference from stack -> not branching");

            Some(pc + 3)
        }
    }
}
