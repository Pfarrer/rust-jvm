use vm::Vm;
use vm::primitive::Primitive;

pub fn eval(vm: &mut Vm, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let value1 = frame.stack_pop();
    let value2 = frame.stack_pop();

    let value1_is_computational_category_1 = match value1 {
        Primitive::Long(_) | Primitive::Double(_) => false,
        _ => true,
    };

    trace!("dup2_x1: Duplicate the top one or two operand stack values and insert two or three values down");

    if value1_is_computational_category_1 {
        // Stack:
        // ..., value3, value2, value1 → ..., value2, value1, value3, value2, value1
        let value3 = frame.stack_pop();

        frame.stack_push(value2.clone());
        frame.stack_push(value1.clone());
        frame.stack_push(value3);
        frame.stack_push(value2);
        frame.stack_push(value1);
    }
    else {
        // Stack
        // ..., value2, value1 → ..., value1, value2, value1
        frame.stack_push(value1.clone());
        frame.stack_push(value2);
        frame.stack_push(value1);
    }

    Some(pc + 1)
}