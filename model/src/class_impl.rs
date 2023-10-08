use anyhow::{anyhow, bail, Result};

use crate::class::{ClassConstant, ClassConstants};

impl ClassConstants {
    pub fn get_string(&self, index: u16) -> Result<&String> {
        let constant = self.0.get((index - 1) as usize).ok_or(anyhow!(
            "Cannot get constant {}, constant pool has size {}",
            index - 1,
            self.0.len()
        ))?;
        match constant {
            &ClassConstant::Utf8(ref string) => Ok(string),
            it => bail!("Expected Utf8 but found {:?} at index {}", it, index),
        }
    }

    pub fn get_class_opt(&self, index: u16) -> Result<Option<&String>> {
        let constant = self.0.get((index - 1) as usize).ok_or(anyhow!(
            "Cannot get constant {}, constant pool has size {}",
            index - 1,
            self.0.len()
        ))?;
        match constant {
            &ClassConstant::Class(ref string) => Ok(Some(string)),
            &ClassConstant::Unused() => Ok(None),
            it => bail!(
                "Expected Class or Unused but found {:?} at index {}",
                it,
                index
            ),
        }
    }

    pub fn get_class(&self, index: u16) -> Result<&String> {
        self.get_class_opt(index)?.ok_or(anyhow!(
            "Class entry with index {} not found in constant pool",
            index
        ))
    }
}
