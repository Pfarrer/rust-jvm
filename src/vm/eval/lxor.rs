use vm::Vm;
use vm::primitive::Primitive;

pub fn eval(vm: &mut Vm, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let value2 = frame.stack_pop_long();
    let value1 = frame.stack_pop_long();
    let result = value1 & value2;

    trace!("lxor: {} & {} -> pushing {} to stack", value1, value2, result);
    frame.stack_push(Primitive::Long(result));

    Some(pc + 1)
}
