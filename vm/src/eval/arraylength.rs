use vm::primitive::Primitive;
use vm::Vm;

pub fn eval(vm: &Vm, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let rc_array = frame.stack_pop_arrayref();
    let array = rc_array.borrow();
    let length = array.elements.len();

    trace!(
        "arraylength: Popped Arrayref from stack and push Int {} to stack",
        length
    );
    frame.stack_push(Primitive::Int(length as i32));

    Some(pc + 1)
}
