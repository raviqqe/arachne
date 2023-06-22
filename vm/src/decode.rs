use core::{mem::size_of, slice};

// TODO Make it return option by default?
#[inline]
pub fn decode_u64(codes: &[u64], index: &mut usize) -> u64 {
    let value = codes[*index];

    *index += 1;

    value
}

#[inline]
pub fn decode_u64_option(codes: &[u64], index: &mut usize) -> Option<u64> {
    if let Some(&value) = codes.get(*index) {
        *index += 1;

        Some(value)
    } else {
        None
    }
}

#[inline]
pub fn decode_f64(codes: &[u64], index: &mut usize) -> f64 {
    f64::from_bits(decode_u64(codes, index))
}

#[inline]
pub fn decode_bytes<'a>(codes: &'a [u64], len: usize, index: &mut usize) -> &'a [u8] {
    let ptr = codes[*index..].as_ptr();

    *index += (len + size_of::<u64>() - 1) / size_of::<u64>();

    unsafe { slice::from_raw_parts(ptr as _, len) }
}
