use std::io::Read;

use classfile::util;

pub type LineNumberTable = Vec<Entry>;

#[derive(Clone, Debug)]
pub struct Entry {
    start_pc: u16,
    line_number: u16,
}

pub fn read(reader: &mut Read) -> LineNumberTable {
    /*let attribute_length = */util::read_u32(reader);

    let line_number_table_length = util::read_u16(reader);
    let mut table = Vec::with_capacity(line_number_table_length as usize);
    for _ in 0..line_number_table_length {
        table.push(read_entry(reader));
    }

    table
}

fn read_entry(reader: &mut Read) -> Entry {
    let start_pc = util::read_u16(reader);
    let line_number = util::read_u16(reader);

    Entry {
        start_pc,
        line_number,
    }
}