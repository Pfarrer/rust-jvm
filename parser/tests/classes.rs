mod test_utils;

#[test]
fn parse_file_empty_class() {
    let class = test_utils::parse_class_in_testdata("EmptyClass.class");

    assert_eq!(55, class.version.major);
    assert_eq!(0, class.version.minor);
    assert_eq!(0, class.fields.len());
    assert_eq!(1, class.methods.len());
}

#[test]
fn parse_file_empty_main() {
    let class = test_utils::parse_class_in_testdata("EmptyMain.class");

    assert_eq!(55, class.version.major);
    assert_eq!(0, class.version.minor);
    assert_eq!(0, class.fields.len());
    assert_eq!(2, class.methods.len());
}

#[test]
fn parse_file_utf8_strings() {
    let class = test_utils::parse_class_in_testdata("Utf8Strings.class");

    assert_eq!(55, class.version.major);
    assert_eq!(0, class.version.minor);
    assert_eq!(1, class.fields.len());
    assert_eq!(2, class.methods.len());
}
