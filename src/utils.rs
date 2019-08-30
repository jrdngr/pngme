use crate::{Error, Result};

pub fn four_bytes(slice: &[u8], range: std::ops::Range<usize>) -> Result<[u8; 4]> {
    if range.end - range.start != 4 {
        Err(Error::new("Range must contain exactly 4 indices"))
    }
    else if slice.len() < range.end {
        Err(Error::new(&format!("slice length is {} but range ends at {}", slice.len(), range.end)))
    } else {
        Ok([slice[range.start], slice[range.start + 1], slice[range.start + 2], slice[range.start + 3]])
    }
}

pub fn u32_from_slice_range(slice: &[u8], range: std::ops::Range<usize>) -> Result<u32> {
    let bytes = four_bytes(slice, range)?;
    Ok(u32::from_be_bytes(bytes))
}
