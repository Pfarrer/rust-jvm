use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};
use std::f64::NAN;

pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value2 = frame.stack_pop_double();
    let value1 = frame.stack_pop_double();

    let result = if value1 == NAN || value2 == NAN {
        let instr = *code.get(pc as usize).unwrap();
        if instr == 152 {
            // dcmpg
            1
        } else if instr == 151 {
            // dcmpl
            -1
        } else {
            panic!("Unexpected instruction: {}", instr);
        }
    } else {
        if value1 > value2 {
            1
        } else if value2 > value1 {
            -1
        } else {
            0
        }
    };

    trace!(
        "dcmp{}: Comparing {} and {} -> pushing {} to stack",
        if *code.get(pc as usize).unwrap() == 151 {
            "dcmpl"
        } else {
            "dcmpg"
        },
        value1,
        value2,
        result
    );
    frame.stack_push(VmPrimitive::Int(result));

    Some(pc + 1)
}
