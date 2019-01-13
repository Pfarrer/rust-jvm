use vm::Vm;
use vm::primitive::Primitive;
use vm::utils;

pub fn eval(vm: &mut Vm, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let offset = utils::read_u16_code(code, pc) as i16;
    let target_pc = pc as i32 + offset as i32;

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::ReturnAddress(pc + 3));

    trace!("jsr: Read offset {} from code, pushed ReturnAddress to stack and branch to subroutine", offset);
    Some(target_pc as u16)
}
