use vm::Vm;
use vm::primitive::Primitive;

pub fn eval(vm: &mut Vm, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let value2 = frame.stack_pop_int();
    let value1 = frame.stack_pop_int();
    let result = value1 * value2;

    trace!("imul: Popping two Ints {} and {} from stack and push result {} back", value1, value2, result);
    frame.stack_push(Primitive::Int(result));

    Some(pc + 1)
}
