use vm::primitive::Primitive;

#[derive(Debug, Clone)]
pub struct Array {

    count: usize,
    atype: u8,
    elements: Vec<Primitive>,

}

impl Array {
    pub fn new(count: usize, atype: u8) -> Array {
        Array {
            count,
            atype,
            elements,
        }
    }
}