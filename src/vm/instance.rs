use classfile::Classfile;

#[derive(Debug, Clone)]
pub struct Instance {
    class: Classfile,
}

impl Instance {
    pub fn new(class: Classfile) -> Instance {
        for _field in &class.fields {
            panic!("Not yet implemented");
        }

        Instance {
            class,
        }
    }
}