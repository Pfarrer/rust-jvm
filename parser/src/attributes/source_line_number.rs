use std::io::Read;

use crate::util;
use model::class::SourceLineNumber;

pub fn read(reader: &mut impl Read) -> Vec<SourceLineNumber> {
    /*let attribute_length = */
    util::read_u32(reader);

    let line_number_table_length = util::read_u16(reader);
    let mut table = Vec::with_capacity(line_number_table_length as usize);
    for _ in 0..line_number_table_length {
        table.push(read_entry(reader));
    }

    table
}

fn read_entry(reader: &mut impl Read) -> SourceLineNumber {
    let start_pc = util::read_u16(reader);
    let line_number = util::read_u16(reader);

    SourceLineNumber {
        start_pc,
        line_number,
    }
}
