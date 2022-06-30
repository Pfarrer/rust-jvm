use vm::Vm;
use vm::primitive::Primitive;

pub fn eval(vm: &mut Vm, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let index = frame.stack_pop_int() as usize;
    let rc_array = frame.stack_pop_arrayref();
    let array = rc_array.borrow();
    assert_eq!(array.atype.unwrap(), 5);

    if let &Primitive::Char(ref value) = array.elements.get(index).unwrap() {
        trace!("caload: Popped two values from stack, read array at index {} and push Int '{}' to stack",
               index, value);

        frame.stack_push(Primitive::Int(value.clone() as i32));
    }
    else {
        panic!("Unexpected array value.");
    }

    Some(pc + 1)
}