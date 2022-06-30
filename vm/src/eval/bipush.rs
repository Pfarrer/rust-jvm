use vm::primitive::Primitive;
use vm::Vm;

pub fn eval(vm: &Vm, code: &Vec<u8>, pc: u16) -> Option<u16> {
    // Sign-extend to i32
    let value = *code.get(pc as usize + 1).unwrap() as i32;

    trace!("bipush: Pushing Int {} to stack", value);
    let frame = vm.frame_stack.last_mut().unwrap();
    frame.stack_push(Primitive::Int(value));

    Some(pc + 2)
}
