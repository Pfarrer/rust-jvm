use crate::{VmPrimitive, VmThread};
use std::f32::NAN;

pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value2 = match frame.stack_pop() {
        VmPrimitive::Float(val) => val,
        _ => panic!("Value set conversion (§2.8.3) not implemented"),
    };
    let value1 = match frame.stack_pop() {
        VmPrimitive::Float(val) => val,
        _ => panic!("Value set conversion (§2.8.3) not implemented"),
    };

    let result = if value1 == NAN || value2 == NAN {
        let instr = *code.get(pc as usize).unwrap();
        if instr == 150 {
            // fcmpg
            1
        } else if instr == 149 {
            // fcmpl
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
        "fcmp_x: Comparing {} and {} -> pushing {} to stack",
        value1,
        value2,
        result
    );
    frame.stack_push(VmPrimitive::Int(result));

    Some(pc + 1)
}
