use std::iter::Peekable;
use std::str::Chars;

use anyhow::{anyhow, bail, Result};
use model::class::TypeSignature;

pub fn parse_type_signature(spec: &String) -> Result<TypeSignature> {
    parse_type(&mut spec.chars().peekable())
}

pub(crate) fn parse_type(iterator: &mut Peekable<Chars>) -> Result<TypeSignature> {
    match iterator.next().ok_or(anyhow!("Reached end of chars"))? {
        'V' => Ok(TypeSignature::Void),
        'Z' => Ok(TypeSignature::Boolean),
        'B' => Ok(TypeSignature::Byte),
        'C' => Ok(TypeSignature::Char),
        'S' => Ok(TypeSignature::Short),
        'I' => Ok(TypeSignature::Int),
        'J' => Ok(TypeSignature::Long),
        'F' => Ok(TypeSignature::Float),
        'D' => Ok(TypeSignature::Double),
        'L' => Ok(TypeSignature::Class(read_class_path(iterator))),
        '[' => Ok(TypeSignature::Array(Box::new(parse_type(iterator)?))),
        c => bail!("Unexpected char of type signature: {}", c),
    }
}

fn read_class_path(iterator: &mut Peekable<Chars>) -> String {
    iterator.take_while(|c| *c != ';').collect::<String>()
}
