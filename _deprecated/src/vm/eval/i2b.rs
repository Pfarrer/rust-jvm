use vm::Vm;
use vm::primitive::Primitive;

pub fn eval(vm: &mut Vm, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_int();
    let result = (value as i8) as i32;

    trace!("i2b: Popped Int {} from stack, truncated to Byte and pushed back as Int {}", value, result);
    frame.stack_push(Primitive::Int(result));

    Some(pc + 1)
}