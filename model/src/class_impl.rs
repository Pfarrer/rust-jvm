use anyhow::{anyhow, bail, Result};

use crate::{class::ClassConstant, prelude::JvmClass};

impl JvmClass {
    pub fn constants_get_string(&self, index: u16) -> Result<&String> {
        let constant = self.constants.get((index - 1) as usize).ok_or(anyhow!(
            "Cannot get constant {}, constant pool has size {}",
            index - 1,
            self.constants.len()
        ))?;
        match constant {
            &ClassConstant::Utf8(ref string) => Ok(string),
            it => bail!("Expected Utf8 but found {:?} at index {}", it, index),
        }
    }

    pub fn constants_get_class(&self, index: u16) -> Result<&String> {
        assert!(index > 0);
        let constant = self.constants.get((index - 1) as usize).ok_or(anyhow!(
            "Cannot get constant {}, constant pool has size {}",
            index - 1,
            self.constants.len()
        ))?;
        match constant {
            &ClassConstant::Class(ref string) => Ok(string),
            it => bail!("Expected Class but found {:?} at index {}", it, index),
        }
    }
}
