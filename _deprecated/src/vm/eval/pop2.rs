use vm::Vm;
use vm::primitive::Primitive;

pub fn eval(vm: &mut Vm, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();

    match frame.stack_pop() {
        Primitive::Long(_) | Primitive::Double(_) => {
            trace!("pop2: Popping value from stack");
        }
        _ => {
            frame.stack_pop();
            trace!("pop2: Popping two values from stack");
        }
    };

    Some(pc + 1)
}
