use crate::{utils, VmPrimitive, VmThread};
use model::prelude::*;

/// Can handle instructions ldc (decimal 18) and ldc_2 (decimal 19).
pub fn eval(
    vm_thread: &mut VmThread,
    jvm_class: &JvmClass,
    code: &Vec<u8>,
    pc: u16,
) -> Option<u16> {
    // Check which instruction triggered this call, if it was ldc, then only one byte should be read,
    // when it was ldc_w, two bytes must be read
    let (index, pc_inc, instr_name) = match *code.get(pc as usize).unwrap() {
        18 => (*code.get((pc + 1) as usize).unwrap() as u16, 2, "ldc"),
        19 => (utils::read_u16_code(code, pc), 3, "ldc_w"),
        i => panic!("Unexpected invocation of this instruction, found: {}", i),
    };

    match jvm_class.constants.get(index as usize).unwrap() {
        &ClassConstant::String(ref value) => {
            trace!("{}: Pushing String \"{}\" to stack", instr_name, value);

            let rc_instance = vm_thread.vm.mem.string_pool.intern(vm_thread, value);
            vm_thread
                .frame_stack
                .last_mut()
                .unwrap()
                .stack_push(VmPrimitive::Objectref(rc_instance));
        }
        &ClassConstant::Float(ref value) => {
            trace!("{}: Pushing Float {} to stack", instr_name, value);
            vm_thread
                .frame_stack
                .last_mut()
                .unwrap()
                .stack_push(VmPrimitive::Float(value.clone()));
        }
        &ClassConstant::Integer(ref value) => {
            trace!("{}: Pushing Int {} to stack", instr_name, value);
            vm_thread
                .frame_stack
                .last_mut()
                .unwrap()
                .stack_push(VmPrimitive::Int(value.clone()));
        }
        &ClassConstant::Class(ref class_path) => {
            trace!("{}: Found Class {}", instr_name, class_path);
            let rc_instance = vm_thread.get_java_class_instance_for(class_path);
            vm_thread
                .frame_stack
                .last_mut()
                .unwrap()
                .stack_push(VmPrimitive::Objectref(rc_instance));
        }
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + pc_inc)
}
