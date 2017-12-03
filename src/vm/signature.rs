
pub enum TypeSignature {

    Void,
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Class(String),
    Array(Type),

}

pub struct MethodSignature {

    parameters: Vec<TypeSignature>,
    return_type: Signature,

}

pub fn parse_type(spec: &String) -> TypeSignature {
    /*
    Z	boolean
    B	byte
    C	char
    S	short
    I	int
    J	long
    F	float
    D	double
    L fully-qualified-class ;	fully-qualified-class
    [ type	type[]
    ( arg-types ) ret-type	method type
    */

    panic!("Not implemented...");
}

pub fn parse_method(spec: &String) -> MethodSignature {
    panic!("Not implemented...");
}