use vm::Vm;

pub fn eval(vm: &mut Vm, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let value1 = frame.stack_pop();
    let value2 = frame.stack_pop();

    trace!("dup_x1: Duplicate the top operand stack value and insert two values down");
    frame.stack_push(value1.clone());
    frame.stack_push(value2);
    frame.stack_push(value1);

    Some(pc + 1)
}