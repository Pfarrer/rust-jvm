use vm::primitive::Primitive;
use vm::utils;
use vm::Vm;

pub fn eval(vm: &mut Vm, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let value = utils::read_u16_code(code, pc) as i32;

    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Int(value));

    trace!("sipush: Pushed Int {} to stack", value);

    Some(pc + 3)
}
