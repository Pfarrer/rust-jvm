use anyhow::Result;
use classfile_parser::types::ClassFile;
use model::class::{ClassAttribute, ClassAttributes, ClassConstants};

pub fn map(classfile: &ClassFile, _constants: &ClassConstants) -> Result<ClassAttributes> {
    classfile
        .attributes
        .iter()
        .map(|attribute| {
            Ok(ClassAttribute::NotImplemented)
        })
        .collect()
}
