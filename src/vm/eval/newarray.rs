use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let count = frame.stack_pop_int();
    let atype = *code.get(pc as usize + 1).unwrap();

    if count < 0 {
        panic!("Not implemented: Throw NegativeArraySizeException");
    }

    trace!("newarray: Create new Array of length {} and push Arrayref to stack", count);

    let default_value = match atype {
        4 => Primitive::Boolean(false),
        5 => Primitive::Char(0),
        6 => Primitive::Float(0.0),
        7 => Primitive::Double(0.0),
        8 => Primitive::Byte(0),
        9 => Primitive::Short(0),
        10 => Primitive::Int(0),
        11 => Primitive::Long(0),
        _ => panic!("Array atype {} not implemented!", atype),
    };

    let mut array = Vec::with_capacity(count as usize);
    for _ in 0..count {
        array.push(default_value.clone());
    }

    frame.stack_push(Primitive::Arrayref(atype, Box::new(array)));

    Some(pc + 2)
}