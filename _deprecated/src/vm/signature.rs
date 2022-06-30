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

impl TypeSignature {

    pub fn as_string(&self) -> String {
        match self {
            TypeSignature::Void => "V".to_string(),
            TypeSignature::Boolean => "Z".to_string(),
            TypeSignature::Byte => "B".to_string(),
            TypeSignature::Char => "C".to_string(),
            TypeSignature::Short => "S".to_string(),
            TypeSignature::Int => "I".to_string(),
            TypeSignature::Long => "J".to_string(),
            TypeSignature::Float => "F".to_string(),
            TypeSignature::Double => "D".to_string(),
            TypeSignature::Class(class_path) => "L".to_string() + class_path,
            TypeSignature::Array(inner_type) => "[".to_string() + &*inner_type.as_string(),
        }
    }
 
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
