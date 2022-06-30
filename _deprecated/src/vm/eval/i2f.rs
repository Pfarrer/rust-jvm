use vm::Vm;
use vm::primitive::Primitive;

pub fn eval(vm: &mut Vm, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_int();

    trace!("i2f: Popped Int {} from stack and push it back as Float", value);
    frame.stack_push(Primitive::Float(value as f32));

    Some(pc + 1)
}