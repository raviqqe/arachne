use core::mem::size_of;

#[inline(always)]
pub fn decode_f64(codes: &[u8], index: &mut usize) -> f64 {
    f64::from_bits(decode_u64(codes, index))
}

#[inline(always)]
pub fn decode_u64(codes: &[u8], index: &mut usize) -> u64 {
    const SIZE: usize = size_of::<u64>();
    let mut bytes = [0u8; SIZE];

    bytes.copy_from_slice(&codes[*index..*index + SIZE]);

    *index += SIZE;

    u64::from_le_bytes(bytes)
}

#[inline(always)]
pub fn decode_u32(codes: &[u8], index: &mut usize) -> u32 {
    const SIZE: usize = size_of::<u32>();
    let mut bytes = [0u8; SIZE];

    bytes.copy_from_slice(&codes[*index..*index + SIZE]);

    *index += SIZE;

    u32::from_le_bytes(bytes)
}

#[inline(always)]
pub fn decode_u16(codes: &[u8], index: &mut usize) -> u16 {
    const SIZE: usize = size_of::<u16>();
    let mut bytes = [0u8; SIZE];

    bytes.copy_from_slice(&codes[*index..*index + SIZE]);

    *index += SIZE;

    u16::from_le_bytes(bytes)
}

#[inline(always)]
pub fn decode_u8(codes: &[u8], index: &mut usize) -> u8 {
    let value = codes[*index];

    *index += 1;

    value
}

#[inline(always)]
pub fn decode_bytes<'a>(codes: &'a [u8], len: usize, index: &mut usize) -> &'a [u8] {
    let value = &codes[*index..*index + len];

    *index += len;

    value
}
