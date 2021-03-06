use vm::Vm;
use vm::primitive::Primitive;

pub fn eval(vm: &mut Vm, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_int();

    trace!("i2c: Popped Int {} from stack and push it back as Char", value);
    frame.stack_push(Primitive::Char(value as u16));

    Some(pc + 1)
}
