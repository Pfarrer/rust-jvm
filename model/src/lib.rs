pub mod api;
pub mod class;
pub mod class_impl;

pub mod prelude {
    pub use crate::class::{ClassAttribute, JvmClass};
}

#[macro_export]
macro_rules! constant_value(
    ($constant:expr, $expected_type:tt) => {
        match $constant {
            $expected_type(i) => Ok(i),
            value => bail!("Expected to get constant of type {} but found {}", stringify!($expected_type), value),
        }
    }
);

