use core::{mem::size_of, slice};

// TODO Make it return option by default?
#[inline]
pub fn decode_word_u64(codes: &[u64], index: &mut usize) -> u64 {
    let value = codes[*index];

    *index += 1;

    value
}

#[inline]
pub fn decode_word_f64(codes: &[u64], index: &mut usize) -> f64 {
    f64::from_bits(decode_word_u64(codes, index))
}

#[inline]
pub fn decode_word_bytes<'a>(codes: &'a [u64], len: usize, index: &mut usize) -> &'a [u8] {
    let ptr = codes[*index..].as_ptr();

    *index += (len + size_of::<u64>() - 1) / size_of::<u64>();

    unsafe { slice::from_raw_parts(ptr as _, len) }
}

#[inline]
pub fn decode_f64(codes: &[u8], index: &mut usize) -> f64 {
    f64::from_bits(decode_u64(codes, index))
}

#[inline]
pub fn decode_u64(codes: &[u8], index: &mut usize) -> u64 {
    const SIZE: usize = size_of::<u64>();
    let mut bytes = [0u8; SIZE];

    bytes.copy_from_slice(&codes[*index..*index + SIZE]);

    *index += SIZE;

    u64::from_le_bytes(bytes)
}

#[inline]
pub fn decode_u32(codes: &[u8], index: &mut usize) -> u32 {
    const SIZE: usize = size_of::<u32>();
    let mut bytes = [0u8; SIZE];

    bytes.copy_from_slice(&codes[*index..*index + SIZE]);

    *index += SIZE;

    u32::from_le_bytes(bytes)
}

#[inline]
pub fn decode_u16(codes: &[u8], index: &mut usize) -> u16 {
    const SIZE: usize = size_of::<u16>();
    let mut bytes = [0u8; SIZE];

    bytes.copy_from_slice(&codes[*index..*index + SIZE]);

    *index += SIZE;

    u16::from_le_bytes(bytes)
}

#[inline]
pub fn decode_u8(codes: &[u8], index: &mut usize) -> u8 {
    let value = codes[*index];

    *index += 1;

    value
}

#[inline]
pub fn decode_u8_option(codes: &[u8], index: &mut usize) -> Option<u8> {
    if let Some(code) = codes.get(*index) {
        *index += 1;

        Some(*code)
    } else {
        None
    }
}

#[inline]
pub fn decode_bytes<'a>(codes: &'a [u8], len: usize, index: &mut usize) -> &'a [u8] {
    let value = &codes[*index..*index + len];

    *index += len;

    value
}
