use model::class::BootstrapMethod;
use std::io::Read;

use crate::util;

pub fn read(reader: &mut impl Read) -> Vec<BootstrapMethod> {
    /* let attribute_length = */
    util::read_u32(reader);

    let num_bootstrap_methods = util::read_u16(reader);
    (0..num_bootstrap_methods)
        .map(|_| read_bootstrap_method(reader))
        .collect()
}

fn read_bootstrap_method(reader: &mut impl Read) -> BootstrapMethod {
    let method_ref = util::read_u16(reader);

    let num_bootstrap_arguments = util::read_u16(reader);
    let arguments = (0..num_bootstrap_arguments)
        .map(|_| util::read_u16(reader))
        .collect();

    BootstrapMethod {
        method_ref,
        arguments,
    }
}
