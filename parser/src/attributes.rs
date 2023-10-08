use anyhow::Result;
use classfile_parser::types::ClassFile;
use model::class::{ClassAttributes, ClassConstants};

pub fn map(classfile: &ClassFile, _constants: &ClassConstants) -> Result<ClassAttributes> {
    classfile
        .attributes
        .iter()
        .flat_map(|attribute| {
            dbg!(attribute);
            None
        })
        .collect()
}
