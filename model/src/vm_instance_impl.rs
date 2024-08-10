use std::fmt::Debug;

use crate::prelude::VmInstance;

impl Debug for VmInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VmInstance({})", self.class_path)
    }
}