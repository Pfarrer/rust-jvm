use vm::instance::Instance;

#[derive(Debug, Clone)]
pub enum Primitive {

    Boolean(bool),

    Byte(u8),

    Short(i16),
    Char(u16),

    Int(i32),
    Float(f32),

    Long(i64),
    Double(f64),

    Arrayref(u8, Box<Vec<Primitive>>),
    Objectref(Box<Instance>),
    ReturnAddress(u16),

    Null,

}

//impl Clone for Primitive {
//    fn clone(&self) -> Primitive {
//        match self {
//            &Primitive::Long(value) => Primitive::Long(value),
//            &Primitive::Int(value) => Primitive::Int(value),
//            &Primitive::Reference(_) => panic!("Implement me: How to handle references?"),
//        }
//    }
//}