use model::class::ClassConstant;

pub fn unwrap_string(constants: &Vec<ClassConstant>, index: u16) -> String {
    match constants.get(index as usize).unwrap() {
        &ClassConstant::Utf8(ref val) => val.to_string(),
        it => panic!("Expected Utf8 but found {:?}", it),
    }
}

pub fn unwrap_class(constants: &Vec<ClassConstant>, index: u16) -> Option<String> {
    match constants.get(index as usize).unwrap() {
        &ClassConstant::Class(ref val) => Some(val.to_string()),
        &ClassConstant::None() => None,
        it => panic!("Expected Class or None but found {:?}", it),
    }
}
