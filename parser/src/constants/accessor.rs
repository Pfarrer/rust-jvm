use model::class::ClassConstant;

pub fn unwrap_string(constants: &Vec<ClassConstant>, index: u16) -> String {
    match constants.get(index as usize).unwrap() {
        &ClassConstant::Utf8(ref val) => val.to_string(),
        it => panic!("Expected Utf8 but found {:?}", it),
    }
}
