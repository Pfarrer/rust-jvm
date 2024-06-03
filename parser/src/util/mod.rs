pub mod conv;

use std::io::Read;

use anyhow::{anyhow, Result};

pub fn read_u16<T: Read>(reader: &mut T) -> Result<u16> {
    let mut bin = [0u8; 2];
    reader.read_exact(&mut bin)?;

    Ok(conv::to_u16(bin))
}

pub fn read_u32<T: Read>(reader: &mut T) -> Result<u32> {
    let mut bin = [0u8; 4];
    reader.read_exact(&mut bin)?;

    Ok(conv::to_u32(bin))
}

pub fn read_raw<T: Read>(reader: &mut T, length: usize) -> Result<Vec<u8>> {
    let mut bytes = Vec::with_capacity(length);
    reader
        .read_exact(&mut bytes)
        .map_err(|err| anyhow!("Unexpected end of input: {}", err))?;

    Ok(bytes)
}
