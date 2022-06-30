use std::cell::RefCell;
use std::rc::Rc;

use vm::array::Array;
use vm::primitive::Primitive;
use vm::Vm;

pub fn eval(vm: &Vm, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let count = frame.stack_pop_int();
    let atype = *code.get(pc as usize + 1).unwrap();

    if count < 0 {
        panic!("Not implemented: Throw NegativeArraySizeException");
    }

    trace!(
        "newarray: Create new Array of length {} and push Arrayref to stack",
        count
    );

    let array = Array::new_primitive(count as usize, atype);
    frame.stack_push(Primitive::Arrayref(Rc::new(RefCell::new(array))));

    Some(pc + 2)
}
