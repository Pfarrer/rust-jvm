use std::fmt::Debug;

use itertools::Itertools;

use crate::prelude::*;

impl Debug for VmInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info = if self.class_path == "java/lang/String" {
            format!("\"{}\"", get_java_string_value(self))
        } else { self.class_path.to_owned() };
        write!(f, "VmInstance({})", info)
    }
}

fn get_java_byte_array_string_value(value_array: &VmArray) -> String {
    let element_values: Vec<u16> = value_array
        .elements
        .iter()
        .tuples()
        .map(|(h, l)| match (h, l) {
            (VmPrimitive::Byte(ref hb), VmPrimitive::Byte(ref lb)) => (*hb as u16) << 8 | *lb as u16,
            p => panic!("Unexpected primitives: {:?}", p),
        })
        .collect();

    String::from_utf16_lossy(element_values.as_slice())
}

fn get_java_string_value(string_instance: &VmInstance) -> String {
    match string_instance.fields.get("value").unwrap() {
        &VmPrimitive::Arrayref(ref rc_value_array) => {
            get_java_byte_array_string_value(&*rc_value_array.borrow())
        }
        p => panic!("Unexpected primitive: {:?}", p),
    }
}