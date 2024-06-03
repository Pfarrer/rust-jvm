use anyhow::{bail, Context, Result};

use crate::{class::ClassConstant, prelude::ClassConstants};

impl ClassConstant {
    pub fn expect_utf8(&self) -> Result<&String> {
        match self {
            &ClassConstant::Utf8(ref string) => Ok(string),
            it => bail!("Expected Utf8 but found {:?}", it),
        }
    }

    pub fn expect_class(&self) -> Result<&String> {
        match self {
            &ClassConstant::Class(ref string) => Ok(string),
            it => bail!("Expected Class but found {:?}", it),
        }
    }
}

pub trait ClassConstantAccessor {
    fn expect_utf8_at(&self, idx: usize) -> Result<&String>;
    fn expect_class_at(&self, idx: usize) -> Result<&String>;
}

impl ClassConstantAccessor for ClassConstants {
    fn expect_utf8_at(&self, idx: usize) -> Result<&String> {
        self.get(idx)
            .context(format!("get constant with index {}", idx))?
            .expect_utf8()
    }

    fn expect_class_at(&self, idx: usize) -> Result<&String> {
        self.get(idx)
            .context(format!("get constant with index {}", idx))?
            .expect_class()
    }
}
