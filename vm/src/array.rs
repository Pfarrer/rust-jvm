use model::prelude::*;

pub trait VmArrayImpl {
    fn new_primitive(count: usize, atype: u8) -> VmArray;
    fn new_complex(count: usize, class_path: String) -> VmArray;
    // fn new_complex_of(elements: Vec<VmPrimitive>, class_path: String) -> VmArray;
}

impl VmArrayImpl for VmArray {
    fn new_primitive(count: usize, atype: u8) -> VmArray {
        let default_value = match atype {
            4 => VmPrimitive::Boolean(false),
            5 => VmPrimitive::Char(0),
            6 => VmPrimitive::Float(0.0),
            7 => VmPrimitive::Double(0.0),
            8 => VmPrimitive::Byte(0),
            9 => VmPrimitive::Short(0),
            10 => VmPrimitive::Int(0),
            11 => VmPrimitive::Long(0),
            _ => panic!("VmArray atype {} not implemented!", atype),
        };

        let mut elements = Vec::with_capacity(count as usize);
        for _ in 0..count {
            elements.push(default_value.clone());
        }

        VmArray {
            atype: Some(atype),
            class_path: None,
            elements,
        }
    }

    fn new_complex(count: usize, class_path: String) -> VmArray {
        let default_value = VmPrimitive::Null;

        let mut elements = Vec::with_capacity(count);
        for _ in 0..count {
            elements.push(default_value.clone());
        }

        VmArray {
            atype: None,
            class_path: Some(class_path),
            elements,
        }
    }

    // fn new_complex_of(elements: Vec<VmPrimitive>, class_path: String) -> VmArray {
    //     VmArray {
    //         atype: None,
    //         class_path: Some(class_path),
    //         elements,
    //     }
    // }
}
