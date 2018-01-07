use vm::primitive::Primitive;

#[derive(Debug, Clone)]
pub struct Array {
    pub atype: Option<u8>,
    pub class_path: Option<String>,
    pub elements: Vec<Primitive>,
}

impl Array {
    pub fn new_primitive(count: usize, atype: u8) -> Array {
        let default_value = match atype {
            4 => Primitive::Boolean(false),
            5 => Primitive::Char(0),
            6 => Primitive::Float(0.0),
            7 => Primitive::Double(0.0),
            8 => Primitive::Byte(0),
            9 => Primitive::Short(0),
            10 => Primitive::Int(0),
            11 => Primitive::Long(0),
            _ => panic!("Array atype {} not implemented!", atype),
        };

        let mut elements = Vec::with_capacity(count as usize);
        for _ in 0..count {
            elements.push(default_value.clone());
        }

        Array {
            atype: Some(atype),
            class_path: None,
            elements,
        }
    }

    pub fn new_complex(count: usize, class_path: String) -> Array {
        let default_value = Primitive::Null;

        let mut elements = Vec::with_capacity(count);
        for _ in 0..count {
            elements.push(default_value.clone());
        }

        Array {
            atype: None,
            class_path: Some(class_path),
            elements,
        }
    }
}