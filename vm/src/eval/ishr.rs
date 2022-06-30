use vm::primitive::Primitive;
use vm::Vm;

pub fn eval(vm: &Vm, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let value2 = frame.stack_pop_int();
    let value1 = frame.stack_pop_int();

    let result = value1 >> (value2 & 0x1f);

    trace!(
        "ishr: Shifting {} right by {} -> pushing {} to stack",
        value1,
        value2 & 0x1f,
        result
    );
    frame.stack_push(Primitive::Int(result));

    Some(pc + 1)
}
