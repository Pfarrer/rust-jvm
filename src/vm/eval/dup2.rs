use vm::primitive::Primitive;
use vm::Vm;

pub fn eval(vm: &mut Vm, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let value1 = frame.stack_pop();

    let value1_is_computational_category_1 = match value1 {
        Primitive::Long(_) | Primitive::Double(_) => false,
        _ => true,
    };

    trace!("dup2: Duplicate the top one or two operand stack values");

    if value1_is_computational_category_1 {
        // Stack:
        // ..., value2, value1 → ..., value2, value1, value2, value1
        let value2 = frame.stack_pop();

        frame.stack_push(value2.clone());
        frame.stack_push(value1.clone());
        frame.stack_push(value2);
        frame.stack_push(value1);
    }
    else {
        // Stack
        // ..., value1 → ..., value1, value1
        frame.stack_push(value1.clone());
        frame.stack_push(value1);
    }

    Some(pc + 1)
}