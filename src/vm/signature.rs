use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
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
    Array(Box<TypeSignature>),
}

#[derive(Debug)]
pub struct MethodSignature {
    pub parameters: Vec<TypeSignature>,
    pub return_type: TypeSignature,
}

pub fn parse_field(spec: &String) -> TypeSignature {
    parse_type(&mut spec.chars().peekable())
}

/// ( arg-types ) ret-type	method-type
pub fn parse_method(spec: &String) -> MethodSignature {
    let mut iterator = spec.chars().peekable();
    assert_eq!('(', iterator.next().unwrap());

    let mut parameters = Vec::new();
    while *iterator.peek().unwrap() != ')' {
        let parameter_type = parse_type(&mut iterator);
        parameters.push(parameter_type);
    }
    assert_eq!(')', iterator.next().unwrap());

    let return_type = parse_type(&mut iterator);
    assert_eq!(None, iterator.next());

    MethodSignature {
        parameters,
        return_type,
    }
}

fn parse_type(iterator: &mut Peekable<Chars>) -> TypeSignature {
    match iterator.next().unwrap() {
        'V' => TypeSignature::Void,
        'Z' => TypeSignature::Boolean,
        'B' => TypeSignature::Byte,
        'C' => TypeSignature::Char,
        'S' => TypeSignature::Short,
        'I' => TypeSignature::Int,
        'J' => TypeSignature::Long,
        'F' => TypeSignature::Float,
        'D' => TypeSignature::Double,
        'L' => TypeSignature::Class(read_class_path(iterator)),
        '[' => TypeSignature::Array(Box::new(parse_type(iterator))),
        c => panic!("Unexpected char of type signature: {}", c),
    }
}

fn read_class_path(iterator: &mut Peekable<Chars>) -> String {
    iterator
        .take_while(|c| *c != ';')
        .collect::<String>()
}
