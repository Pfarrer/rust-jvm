mod test_utils;

#[test]
fn parse_java11_java_lang_object() {
    let class = test_utils::parse_class_in_testdata("java11/java.lang.Object.class");

    assert_eq!(55, class.version.major);
    assert_eq!(0, class.version.minor);
    assert_eq!(0, class.fields.len());
    assert_eq!(14, class.methods.len());
}
