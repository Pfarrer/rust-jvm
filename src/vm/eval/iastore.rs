use vm::primitive::Primitive;
use vm::Vm;

pub fn eval(vm: &mut Vm, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_int();
    let index = frame.stack_pop_int() as usize;

    let rc_array = frame.stack_pop_arrayref();
    let mut array = rc_array.borrow_mut();
    assert_eq!(array.atype.unwrap(), 10);

    trace!("iastore: Popped three values from stack and write '{}' at array index {}", value, index);
    array.elements[index] = Primitive::Int(value);

    Some(pc + 1)
}