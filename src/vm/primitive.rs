use vm::instance::Instance;

#[derive(Debug, Clone)]
pub enum Primitive {

    Long(i64),

    Int(i32),

    Reference(Box<Instance>),
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