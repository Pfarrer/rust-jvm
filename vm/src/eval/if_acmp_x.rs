use crate::{frame::VmFrameImpl, utils, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value2 = frame.stack_pop();
    let value1 = frame.stack_pop();

    let equals = match (&value1, &value2) {
        (&VmPrimitive::Objectref(ref rc_instance1), &VmPrimitive::Objectref(ref rc_instance2)) => {
            rc_instance1.eq(rc_instance2)
        }
        (&VmPrimitive::Objectref(_), &VmPrimitive::Null) => false,
        (&VmPrimitive::Null, &VmPrimitive::Objectref(_)) => false,
        
        (&VmPrimitive::Arrayref(ref rc_array1), &VmPrimitive::Arrayref(ref rc_array2)) => {
            rc_array1.eq(rc_array2)
        }
        (&VmPrimitive::Arrayref(_), &VmPrimitive::Null) => false,
        (&VmPrimitive::Null, &VmPrimitive::Arrayref(_)) => false,

        (&VmPrimitive::Null, &VmPrimitive::Null) => true,

        _ => panic!("Not implemented: {:?} -- {:?}", value1, value2),
    };

    let (cmp_result, instr_name) = match *code.get(pc as usize).unwrap() {
        165 => (equals, "eq"),
        166 => (!equals, "ne"),
        _ => panic!("if_acmp_x::eval was called on a non if_acmp_x instruction."),
    };

    trace!(
        "if_acmp{}: Popped two references from stack, compare result: {}? -> {}",
        instr_name,
        instr_name,
        cmp_result
    );

    if cmp_result {
        let branch_offset = utils::read_i16_code(code, pc);
        let target_pc: u16 = (pc as i16 + branch_offset) as u16;

        Some(target_pc)
    } else {
        Some(pc + 3)
    }
}
