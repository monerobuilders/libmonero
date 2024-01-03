pub fn to_scratchpad_address(value: [u8; 16]) -> usize {
    // Interpret the 16-byte value as a little-endian integer and use the 21 low-order bits
    let index = u64::from_le_bytes(value[0..8].try_into().unwrap()) & 0x1FFFFF;
    // Clear the 4 low-order bits for 16-byte alignment
    (index >> 4) as usize
}

pub fn eight_byte_add(a: [u8; 16], b: [u8; 16]) -> [u8; 16] {
    let a_low = u64::from_le_bytes(a[0..8].try_into().unwrap());
    let b_low = u64::from_le_bytes(b[0..8].try_into().unwrap());
    let a_high = u64::from_le_bytes(a[8..16].try_into().unwrap());
    let b_high = u64::from_le_bytes(b[8..16].try_into().unwrap());

    let low_result = a_low.wrapping_add(b_low);
    let high_result = a_high.wrapping_add(b_high);

    let low_bytes = low_result.to_le_bytes();
    let high_bytes = high_result.to_le_bytes();

    let mut result = [0; 16];
    result[0..8].copy_from_slice(&low_bytes);
    result[8..16].copy_from_slice(&high_bytes);

    result
}

pub fn eight_byte_mul(a: [u8; 16], b: [u8; 16]) -> [u8; 16] {
    let a_low = u64::from_le_bytes(a[0..8].try_into().unwrap());
    let b_low = u64::from_le_bytes(b[0..8].try_into().unwrap());
    let a_high = u64::from_le_bytes(a[8..16].try_into().unwrap());
    let b_high = u64::from_le_bytes(b[8..16].try_into().unwrap());

    let low_result = a_low.wrapping_mul(b_low);
    let high_result = a_high.wrapping_mul(b_high);

    let low_bytes = low_result.to_le_bytes();
    let high_bytes = high_result.to_le_bytes();

    let mut result = [0; 16];
    result[0..8].copy_from_slice(&low_bytes);
    result[8..16].copy_from_slice(&high_bytes);

    result
}

pub fn eight_byte_xor(a: [u8; 16], b: [u8; 16]) -> [u8; 16] {
    let mut result = [0; 16];
    for i in 0..16 {
        result[i] = a[i] ^ b[i];
    }
    result
}