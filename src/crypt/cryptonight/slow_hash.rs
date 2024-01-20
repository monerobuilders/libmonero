use super::aesu::derive_key;
use crate::crypt::cryptonight::aesu::aes_round;
use sha3::{Digest, Keccak256Full};

const SCRATCHPAD_SIZE: usize = 2 * 1024 * 1024; // 2 MiB

// WORKS CORRECTLY
fn turn_to_u8_16(u64p: [u64; 2]) -> [u8; 16] {
    let mut u8_16 = [0u8; 16];
    u8_16[0..8].copy_from_slice(&u64p[0].to_le_bytes());
    u8_16[8..16].copy_from_slice(&u64p[1].to_le_bytes());
    u8_16
}

// WORKS CORRECTLY
fn turn_to_u64_2(u8_16: [u8; 16]) -> [u64; 2] {
    let mut u64_2 = [0u64; 2];
    u64_2[0] = u64::from_le_bytes(u8_16[0..8].try_into().unwrap());
    u64_2[1] = u64::from_le_bytes(u8_16[8..16].try_into().unwrap());
    u64_2
}

// WORKS CORRECTLY
fn xor_pair_u64_2(a: [u64; 2], b: [u64; 2]) -> [u64; 2] {
    let mut res = [0u64; 2];
    res[0] = a[0] ^ b[0];
    res[1] = a[1] ^ b[1];
    res
}

// WORKS CORRECTLY
fn turn_to_u64(u8_8: &[u8]) -> u64 {
    u64::from_le_bytes(u8_8.try_into().unwrap())
}

// The 8byte_mul function, however, uses only the first 8 bytes of each
// argument, which are interpreted as unsigned 64-bit little-endian
// integers and multiplied together. The result is converted into 16
// bytes, and finally the two 8-byte halves of the result are swapped.
fn mul_pair_u64_2(a: [u64; 2], b: [u64; 2]) -> [u64; 2] {
    let a = u128::from(a[0]);
    let b = u128::from(b[0]);

    let r = a * b;

    let res: [u64; 2];

    res = [(r >> 64) as u64, r as u64];
    return res;
}

// Where, the 8byte_add function represents each of the arguments as a
// pair of 64-bit little-endian values and adds them together,
// component-wise, modulo 2^64. The result is converted back into 16
// bytes.
fn add_pair_u64_2(a: [u64; 2], b: [u64; 2]) -> [u64; 2] {
    let mut res = [0u64; 2];
    res[0] = a[0].wrapping_add(b[0]);
    res[1] = a[1].wrapping_add(b[1]);
    res
}

pub fn cn_slow_hash(input: &[u8]) -> [u8; SCRATCHPAD_SIZE] {
    // CryptoNight Step 1: Initialization Of Scratchpad

    //     First, the input is hashed using Keccak [KECCAK] with parameters b =
    //    1600 and c = 512. The bytes 0..31 of the Keccak final state are
    //    interpreted as an AES-256 key [AES] and expanded to 10 round keys. A
    //    scratchpad of 2097152 bytes (2 MiB) is allocated. The bytes 64..191
    //    are extracted from the Keccak final state and split into 8 blocks of
    //    16 bytes each. Each block is encrypted using the following procedure:

    //       for i = 0..9 do:
    //           block = aes_round(block, round_keys[i])

    //    Where aes_round function performs a round of AES encryption, which
    //    means that SubBytes, ShiftRows and MixColumns steps are performed on
    //    the block, and the result is XORed with the round key. Note that
    //    unlike in the AES encryption algorithm, the first and the last rounds
    //    are not special. The resulting blocks are written into the first 128
    //    bytes of the scratchpad. Then, these blocks are encrypted again in
    //    the same way, and the result is written into the second 128 bytes of
    //    the scratchpad. Each time 128 bytes are written, they represent the
    //    result of the encryption of the previously written 128 bytes. The
    //    process is repeated until the scratchpad is fully initialized.

    // Step 1A: Initialize the scratchpad with empty data
    let mut scratchpad = [0u8; SCRATCHPAD_SIZE];

    // Step 1B: Use Keccak256Full to hash the input
    let mut keccak_hash = [0u8; 200];
    let mut hasher = Keccak256Full::new();
    hasher.update(input);
    keccak_hash.copy_from_slice(&hasher.finalize());

    // Step 1C: Use the first 32 bytes of the Keccak hash as an AES-256 key
    let aes_key = &keccak_hash[0..32];

    // Step 1D: Expand the AES-256 key into 10 round keys
    let round_keys = derive_key(aes_key);

    // Step 1E: Use bytes 64..191 of the Keccak hash as 8 blocks of 16 bytes each
    let mut blocks = [0u8; 128];
    blocks.copy_from_slice(&keccak_hash[64..192]);

    // Step 1F: Loop until scratchpad is fully initialized
    for scratchpad_chunk in scratchpad.chunks_exact_mut(blocks.len()) {
        for block in blocks.chunks_exact_mut(16) {
            for key in round_keys.chunks_exact(16) {
                aes_round(block, key);
            }
        }

        scratchpad_chunk.copy_from_slice(&blocks);
    }

    // Cryptonigth Step 2: Memory-hard Loop

    // Prior to the main loop, bytes 0..31 and 32..63 of the Keccak state
    // are XORed, and the resulting 32 bytes are used to initialize
    // variables a and b, 16 bytes each. These variables are used in the
    // main loop. The main loop is iterated 524,288 times. When a 16-byte
    // value needs to be converted into an address in the scratchpad, it is
    // interpreted as a little-endian integer, and the 21 low-order bits are
    // used as a byte index. However, the 4 low-order bits of the index are
    // cleared to ensure the 16-byte alignment. The data is read from and
    // written to the scratchpad in 16-byte blocks. Each iteration can be
    // expressed with the following pseudo-code:

    //     scratchpad_address = to_scratchpad_address(a)
    //     scratchpad[scratchpad_address] = aes_round(scratchpad
    //     [scratchpad_address], a)
    //     b, scratchpad[scratchpad_address] = scratchpad[scratchpad_address],
    //     b xor scratchpad[scratchpad_address]
    //     scratchpad_address = to_scratchpad_address(b)
    //     a = 8byte_add(a, 8byte_mul(b, scratchpad[scratchpad_address]))
    //     a, scratchpad[scratchpad_address] = a xor
    //     scratchpad[scratchpad_address], a

    // Where, the 8byte_add function represents each of the arguments as a
    // pair of 64-bit little-endian values and adds them together,
    // component-wise, modulo 2^64. The result is converted back into 16
    // bytes.

    // The 8byte_mul function, however, uses only the first 8 bytes of each
    // argument, which are interpreted as unsigned 64-bit little-endian
    // integers and multiplied together. The result is converted into 16
    // bytes, and finally the two 8-byte halves of the result are swapped.

    // Turn [u8; SCRATCHPAD_SIZE] into [[u64; 2]; 131072]
    let mut sp_u64_2 = [[0u64; 2]; 131072];
    for (i, sp_u64_2_chunk) in sp_u64_2.iter_mut().enumerate() {
        let u64_slice = unsafe {
            std::slice::from_raw_parts(scratchpad[i * 16..(i + 1) * 16].as_ptr() as *const u64, 2)
        };
        sp_u64_2_chunk.copy_from_slice(u64_slice);
    }

    // Get a and b variables
    let a_1: u64 = turn_to_u64(&keccak_hash[0..8]) ^ turn_to_u64(&keccak_hash[32..40]);
    let a_2: u64 = turn_to_u64(&keccak_hash[8..16]) ^ turn_to_u64(&keccak_hash[40..48]);
    let b_1: u64 = turn_to_u64(&keccak_hash[16..24]) ^ turn_to_u64(&keccak_hash[48..56]);
    let b_2: u64 = turn_to_u64(&keccak_hash[24..32]) ^ turn_to_u64(&keccak_hash[56..64]);
    let mut a: [u64; 2] = [a_1, a_2];
    let mut b: [u64; 2] = [b_1, b_2];

    // Main loop
    for _ in 0..524_288 {
        // Step 1A: First Transfer
        let addr: usize = (a[0] & 0x1F_FFF0) as usize / 16;
        let block = &mut turn_to_u8_16(sp_u64_2[addr]);
        aes_round(block, &turn_to_u8_16(a));
        sp_u64_2[addr] = turn_to_u64_2(*block);
        let tmp = b;
        b = sp_u64_2[addr];
        let man = xor_pair_u64_2(sp_u64_2[addr], tmp);
        sp_u64_2[addr] = man;

        // Step 1C: Second Transfer
        let addr: usize = (b[0] & 0x1F_FFF0) as usize / 16;
        let tmp = add_pair_u64_2(a, mul_pair_u64_2(b, sp_u64_2[addr]));
        a = xor_pair_u64_2(sp_u64_2[addr], tmp);
        sp_u64_2[addr] = tmp;
    }

    // Works until here

    println!("1st: {:?}, {:?}", sp_u64_2[0][0], sp_u64_2[0][1]);
    println!("2nd: {:?}, {:?}", sp_u64_2[1][0], sp_u64_2[1][1]);
    println!("3rd: {:?}, {:?}", sp_u64_2[2][0], sp_u64_2[2][1]);
    println!("4th: {:?}, {:?}", sp_u64_2[3][0], sp_u64_2[3][1]);
    println!(
        "last-4: {:?}, {:?}",
        sp_u64_2[131068][0], sp_u64_2[131068][1]
    );
    println!(
        "last-3: {:?}, {:?}",
        sp_u64_2[131069][0], sp_u64_2[131069][1]
    );
    println!(
        "last-2: {:?}, {:?}",
        sp_u64_2[131070][0], sp_u64_2[131070][1]
    );
    println!(
        "last-1: {:?}, {:?}",
        sp_u64_2[131071][0], sp_u64_2[131071][1]
    );

    return scratchpad;
}
