use vm::primitive::Primitive;
use vm::Vm;

pub fn eval(vm: &Vm, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let value2 = frame.stack_pop_long();
    let value1 = frame.stack_pop_long();
    let result = if value1 > value2 {
        -1
    } else if value2 > value1 {
        1
    } else {
        0
    };

    trace!(
        "lcmp: Compared {} and {} -> pushing {} to stack",
        value1,
        value2,
        result
    );
    frame.stack_push(Primitive::Int(result));

    Some(pc + 1)
}
