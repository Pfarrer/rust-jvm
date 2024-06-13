use anyhow::{bail, Context, Result};

use crate::{
    class::ClassConstant,
    prelude::{ClassConstants, MethodSignature},
};

impl ClassConstant {
    pub fn ok_and_utf8_or(&self) -> Result<&String> {
        match self {
            &ClassConstant::Utf8(ref string) => Ok(string),
            it => bail!("Expected Utf8 but found {:?}", it),
        }
    }

    pub fn ok_and_class_or(&self) -> Result<&String> {
        match self {
            &ClassConstant::Class(ref string) => Ok(string),
            it => bail!("Expected Class but found {:?}", it),
        }
    }

    pub fn ok_and_methodref_or(&self) -> Result<(&String, &String, &MethodSignature)> {
        match self {
            &ClassConstant::Methodref(ref class_path, ref method_name, ref method_signature) => {
                Ok((class_path, method_name, method_signature))
            }
            it => bail!("Expected Methodref but found {:?}", it),
        }
    }
}

pub trait ClassConstantAccessor {
    fn get_utf8_or(&self, idx: usize) -> Result<&String>;
    fn get_class_or(&self, idx: usize) -> Result<&String>;
    fn get_methodref_or(&self, idx: usize) -> Result<(&String, &String, &MethodSignature)>;
}

impl ClassConstantAccessor for ClassConstants {
    fn get_utf8_or(&self, idx: usize) -> Result<&String> {
        self.get(idx)
            .context(format!("get constant with index {}", idx))?
            .ok_and_utf8_or()
    }

    fn get_class_or(&self, idx: usize) -> Result<&String> {
        self.get(idx)
            .context(format!("get constant with index {}", idx))?
            .ok_and_class_or()
    }

    fn get_methodref_or(&self, idx: usize) -> Result<(&String, &String, &MethodSignature)> {
        self.get(idx)
            .context(format!("get constant with index {}", idx))?
            .ok_and_methodref_or()
    }
}
