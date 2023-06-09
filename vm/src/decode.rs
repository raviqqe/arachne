use core::mem::size_of;

pub fn read_u64(instructions: &[u8]) -> u64 {
    const SIZE: usize = size_of::<u64>();
    let mut bytes = [0u8; SIZE];

    bytes.copy_from_slice(&instructions[..SIZE]);

    u64::from_le_bytes(bytes)
}

pub fn read_u32(instructions: &[u8]) -> u32 {
    const SIZE: usize = size_of::<u32>();
    let mut bytes = [0u8; SIZE];

    bytes.copy_from_slice(&instructions[0..SIZE]);

    u32::from_le_bytes(bytes)
}

pub fn read_u8(instructions: &[u8]) -> u8 {
    instructions[0]
}
