use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    trace!("dup2_x1: Duplicate the top one or two operand stack values and insert two or three values down");

    let value1 = frame.stack_pop();
    let value2 = frame.stack_pop();

    let value1_is_computational_category_1 = match value1 {
        Primitive::Long(_) | Primitive::Double(_) => false,
        _ => true,
    };

//    trace!("value1_is_computational_category_1 = {:#?}", value1_is_computational_category_1);
//    trace!("value1 = {:#?}", value1);
//    trace!("value2 = {:#?}", value2);

    if value1_is_computational_category_1 {
        // Stack:
        // ..., value3, value2, value1 → ..., value2, value1, value3, value2, value1
//        panic!("{:#?}", frame);

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

//    trace!("{:#?}", frame);

    Some(pc + 1)
}