use super::aesu::derive_key;
use crate::crypt::cryptonight::aesu::aes_round;
use tiny_keccak::{Hasher, Keccak};

const SCRATCHPAD_SIZE: usize = 2 * 1024 * 1024; // 2 MiB

pub fn cn_slow_hash(input: &[u8]) -> [u8; 10] {
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

    // Step 1B: Keccak hash the input to get a 200 byte hash
    let mut keccak = Keccak::v256();
    keccak.update(input);
    let mut keccak_hash = [0u8; 200];
    keccak.finalize(&mut keccak_hash);

    // Step 1C: Use the first 32 bytes of the Keccak hash as an AES-256 key
    let aes_key = &keccak_hash[0..32];

    // Step 1D: Expand the AES-256 key into 10 round keys
    let round_keys = derive_key(aes_key);

    // Step 1E: Use bytes 64..191 of the Keccak hash as 8 blocks of 16 bytes each
    let mut blocks = [0u8; 128];
    blocks.copy_from_slice(&keccak_hash[64..192]);

    for scratchpad_chunk in scratchpad.chunks_exact_mut(blocks.len()) {
        for block in blocks.chunks_exact_mut(16) {
            for key in round_keys.chunks_exact(16) {
                aes_round(block, key);
            }
        }

        scratchpad_chunk.copy_from_slice(&blocks);
    }

    // return first 10 bytes of scratchpad
    let mut result = [0u8; 10];
    result.copy_from_slice(&scratchpad[0..10]);
    result
}