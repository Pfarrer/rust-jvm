use vm::Vm;

pub fn eval(vm: &mut Vm, pc: u16) -> Option<u16> {
    trace!("pop: Popped value from stack");
    vm.frame_stack.last_mut().unwrap().stack_pop();

    Some(pc + 1)
}
