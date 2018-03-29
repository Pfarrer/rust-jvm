use vm::primitive::Primitive;
use vm::Vm;

pub fn eval(vm: &mut Vm) -> Option<u16> {
    let mut frame = vm.frame_stack.pop().unwrap();
    let mut parent_frame = vm.frame_stack.pop().unwrap();

    let ret_val = frame.stack_pop_int();
    parent_frame.stack_push(Primitive::Int(ret_val));
    trace!("ireturn: Popped Int {} from stack, returning to parent method", ret_val);

    // Push frames back to the stack
    vm.frame_stack.push(parent_frame);
    vm.frame_stack.push(frame);

    None
}
