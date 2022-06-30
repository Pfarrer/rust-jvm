mod test_utils;

#[test]
fn parse_file_interface_with_default_and_static() {
    let class = test_utils::parse_class_in_testdata("InterfaceWithDefaultAndStatic.class");

    assert_eq!(55, class.version.major);
    assert_eq!(0, class.version.minor);
    assert_eq!(0, class.fields.len());
    assert_eq!(2, class.methods.len());
}
