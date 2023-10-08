use anyhow::Result;
use model::class::MethodSignature;

use crate::type_signature::parse_type;

/// ( arg-types ) ret-type	method-type
pub fn parse_method_signature(spec: impl AsRef<str>) -> Result<MethodSignature> {
    let mut iterator = spec.as_ref().chars().peekable();
    assert_eq!('(', iterator.next().unwrap());

    let mut parameters = Vec::new();
    while *iterator.peek().unwrap() != ')' {
        let parameter_type = parse_type(&mut iterator)?;
        parameters.push(parameter_type);
    }
    assert_eq!(')', iterator.next().unwrap());

    let return_type = parse_type(&mut iterator)?;
    assert_eq!(None, iterator.next());

    Ok(MethodSignature {
        parameters,
        return_type,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_array_return_void() {
        assert_parse_and_format_equal("([Ljava/lang/String;)V");
    }

    #[test]
    fn int_string_array_int_return_long() {
        assert_parse_and_format_equal("(ILjava/lang/String;[I)J");
    }

    fn assert_parse_and_format_equal(signature_str: &str) {
        let signature = parse_method_signature(signature_str).unwrap();
        assert_eq!(signature_str, format!("{}", signature));
    }
}
