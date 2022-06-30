use vm::Vm;
use vm::primitive::Primitive;

pub fn eval(vm: &mut Vm, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let objectref = frame.stack_pop_reference();
    match objectref {
        Primitive::Null => panic!("Not implemented -> throw NullPointerException"),
        _ => (),
    };

    trace!("monitorexit: Popped one reference from stack and did nothing else");

    Some(pc + 1)
}