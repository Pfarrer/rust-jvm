use anyhow::{bail, Result};

use crate::class::ClassConstant;

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
