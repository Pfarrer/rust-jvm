use std::fmt::Debug;

use itertools::Itertools;

use crate::prelude::*;

impl Debug for VmInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fields = self
            .fields
            .iter()
            .map(|(k, v)| format!("{}={:?}", k, v))
            .join(", ");
        write!(f, "VmInstance({}, {})", self.class_path, fields)
    }
}
