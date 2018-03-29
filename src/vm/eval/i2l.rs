use vm::Vm;
use vm::primitive::Primitive;

pub fn eval(vm: &mut Vm, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_int();

    trace!("i2l: Popped Int {} from stack and push it back as Long", value);
    frame.stack_push(Primitive::Long(value as i64));

    Some(pc + 1)
}