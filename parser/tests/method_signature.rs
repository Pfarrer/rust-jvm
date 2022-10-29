use parser::parse_method_signature;

#[test]
fn string_array_return_void() {
    assert_parse_and_format_equal("([Ljava/lang/String;)V");
}

#[test]
fn int_string_array_int_return_long() {
    assert_parse_and_format_equal("(ILjava/lang/String;[I)J");
}

fn assert_parse_and_format_equal(signature_str: &str) {
    let signature = parse_method_signature(signature_str);
    assert_eq!(signature_str, format!("{}", signature));
}
