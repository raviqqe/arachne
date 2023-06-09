use core::mem::size_of;

pub fn decode_u64(instructions: &[u8], index: &mut usize) -> u64 {
    const SIZE: usize = size_of::<u64>();
    let mut bytes = [0u8; SIZE];

    bytes.copy_from_slice(&instructions[*index..*index + SIZE]);

    *index += SIZE;

    u64::from_le_bytes(bytes)
}

pub fn decode_u32(instructions: &[u8], index: &mut usize) -> u32 {
    const SIZE: usize = size_of::<u32>();
    let mut bytes = [0u8; SIZE];

    bytes.copy_from_slice(&instructions[*index..*index + SIZE]);

    *index += SIZE;

    u32::from_le_bytes(bytes)
}

pub fn decode_u8(codes: &[u8], index: &mut usize) -> u8 {
    let value = codes[*index];

    *index += 1;

    value
}

pub fn decode_bytes<'a>(codes: &'a [u8], len: usize, index: &mut usize) -> &'a [u8] {
    let value = &codes[*index..*index + len];

    *index += len;

    value
}
