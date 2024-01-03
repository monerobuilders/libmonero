use blake2::Blake2s256;
use digest::{consts::U32, Digest};
use groestl::Groestl256;
use jh::Jh256;
use skein::Skein256;
use tiny_keccak::{Hasher, Keccak};

use super::utility::{aesu::{e10rk, aes_round}, otheru::{eight_byte_mul, eight_byte_add, to_scratchpad_address, eight_byte_xor}};

const SCRATCHPAD_SIZE: usize = 2 * 1024 * 1024;  // 2 MiB

pub fn cn_slow_hash(input: &[u8]) -> [u8; 32] {
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

    // ORIGINAL CODE DOC:

    //  * Computes the hash of <data> (which consists of <length> bytes), returning the
    //  * hash in <hash>.  The CryptoNight hash operates by first using Keccak 1600,
    //  * the 1600 bit variant of the Keccak hash used in SHA-3, to create a 200 byte
    //  * buffer of pseudorandom data by hashing the supplied data.  It then uses this
    //  * random data to fill a large 2MB buffer with pseudorandom data by iteratively
    //  * encrypting it using 10 rounds of AES per entry.  After this initialization,
    //  * it executes 524,288 rounds of mixing through the random 2MB buffer using
    //  * AES (typically provided in hardware on modern CPUs) and a 64 bit multiply.
    //  * Finally, it re-mixes this large buffer back into
    //  * the 200 byte "text" buffer, and then hashes this buffer using one of four
    //  * pseudorandomly selected hash functions (Blake, Groestl, JH, or Skein)
    //  * to populate the output.

    // Step 1A: Keccak hash and key expansion
    let mut keccak: Keccak = Keccak::v256();
    let mut final_state: [u8; 200] = [0; 200];
    keccak.update(input);
    keccak.finalize(&mut final_state);

    // Step 1B: First 32 bytes of Keccak output are used as AES key and expanded to 10 round keys
    let first32: [u8; 32] = final_state[0..32].try_into().unwrap();
    let round_keys = e10rk(&first32);

    // Step 1C: Initialize empty scratchpad
    let mut scratchpad = [0; SCRATCHPAD_SIZE];

    // Step 1D: Take bytes 64..191 of Keccak output and split into 8 blocks of 16 bytes each
    let bytes64to191: [u8; 128] = final_state[64..192].try_into().unwrap();
    let mut blocks: [[u8; 16]; 8] = [[0; 16]; 8];
    for i in 0..8 {
        blocks[i] = bytes64to191[i * 16..(i + 1) * 16].try_into().unwrap();
    }

    // Step 1E: Encrypt each block with AES using crates, write to scratchpad, and repeat
    for i in 0..8 {
        let mut block = blocks[i];
        for j in 0..10 {
            block = aes_round(&block, &round_keys[j]);
        }
        scratchpad[i * 16..(i + 1) * 16].copy_from_slice(&block);
    }

    for i in 8..(SCRATCHPAD_SIZE / 16) {
        let mut block = [0; 16];
        for j in 0..16 {
            block[j] = scratchpad[(i - 8) * 16 + j];
        }
        for j in 0..10 {
            block = aes_round(&block, &round_keys[j]);
        }
        scratchpad[i * 16..(i + 1) * 16].copy_from_slice(&block);
    }

    // CryptoNight Step 2: Memory-hard Loop

    //     Prior to the main loop, bytes 0..31 and 32..63 of the Keccak state
    //    are XORed, and the resulting 32 bytes are used to initialize
    //    variables a and b, 16 bytes each. These variables are used in the
    //    main loop. The main loop is iterated 524,288 times. When a 16-byte
    //    value needs to be converted into an address in the scratchpad, it is
    //    interpreted as a little-endian integer, and the 21 low-order bits are
    //    used as a byte index. However, the 4 low-order bits of the index are
    //    cleared to ensure the 16-byte alignment. The data is read from and
    //    written to the scratchpad in 16-byte blocks. Each iteration can be
    //    expressed with the following pseudo-code:

    //       scratchpad_address = to_scratchpad_address(a)
    //       scratchpad[scratchpad_address] = aes_round(scratchpad 
    //         [scratchpad_address], a)
    //       b, scratchpad[scratchpad_address] = scratchpad[scratchpad_address],
    //         b xor scratchpad[scratchpad_address]
    //       scratchpad_address = to_scratchpad_address(b)
    //       a = 8byte_add(a, 8byte_mul(b, scratchpad[scratchpad_address]))
    //       a, scratchpad[scratchpad_address] = a xor 
    //         scratchpad[scratchpad_address], a

    //    Where, the 8byte_add function represents each of the arguments as a
    //    pair of 64-bit little-endian values and adds them together,
    //    component-wise, modulo 2^64. The result is converted back into 16
    //    bytes.

    //    The 8byte_mul function, however, uses only the first 8 bytes of each
    //    argument, which are interpreted as unsigned 64-bit little-endian
    //    integers and multiplied together. The result is converted into 16
    //    bytes, and finally the two 8-byte halves of the result are swapped.

    // Step 2A: XOR bytes 0..31 and 32..63 of Keccak state
    let mut first32: [u8; 32] = final_state[0..32].try_into().unwrap();
    let second32: [u8; 32] = final_state[32..64].try_into().unwrap();
    for i in 0..32 {
        first32[i] ^= second32[i];
    }

    // Step 2B: Initialize variables a and b
    let mut a: [u8; 16] = [0; 16];
    let mut b: [u8; 16] = [0; 16];
    a.copy_from_slice(&first32[0..16]);
    b.copy_from_slice(&first32[16..32]);

    // Step 2C: Main loop, 524288 iterations as described above
    for _ in 0..524288 {
        let scratchpad_address = to_scratchpad_address(a);
        let mut block = [0; 16];
        for i in 0..16 {
            block[i] = scratchpad[scratchpad_address + i];
        }
        for _ in 0..10 {
            block = aes_round(&block, &round_keys[0]);
        }
        scratchpad[scratchpad_address..scratchpad_address + 16].copy_from_slice(&block);
        b = eight_byte_xor(b, scratchpad[scratchpad_address..scratchpad_address + 16].try_into().unwrap());
        let scratchpad_address = to_scratchpad_address(b);
        let mut block = [0; 16];
        for i in 0..16 {
            block[i] = scratchpad[scratchpad_address + i];
        }
        a = eight_byte_add(a, eight_byte_mul(b, block));
        a = eight_byte_xor(a, scratchpad[scratchpad_address..scratchpad_address + 16].try_into().unwrap());
    }

    // CryptoNight Step 3: Result Calculation

    //    After the memory-hard part, bytes 32..63 from the Keccak state are
    //    expanded into 10 AES round keys in the same manner as in the first
    //    part.

    //    Bytes 64..191 are extracted from the Keccak state and XORed with the
    //    first 128 bytes of the scratchpad. Then the result is encrypted in
    //    the same manner as in the first part, but using the new keys. The
    //    result is XORed with the second 128 bytes from the scratchpad,
    //    encrypted again, and so on. 

    //    After XORing with the last 128 bytes of the scratchpad, the result is
    //    encrypted the last time, and then the bytes 64..191 in the Keccak
    //    state are replaced with the result. Then, the Keccak state is passed
    //    through Keccak-f (the Keccak permutation) with b = 1600. 

    //    Then, the 2 low-order bits of the first byte of the state are used to
    //    select a hash function: 0=BLAKE-256 [BLAKE], 1=Groestl-256 [GROESTL],
    //    2=JH-256 [JH], and 3=Skein-256 [SKEIN]. The chosen hash function is
    //    then applied to the Keccak state, and the resulting hash is the
    //    output of CryptoNight.

    // Step 3A: Expand bytes 32..63 of Keccak state into 10 AES round keys
    let round_keys = e10rk(&second32);

    // Step 3B: XOR bytes 64..191 of Keccak state with first 128 bytes of scratchpad
    let mut bytes64to191: [u8; 128] = final_state[64..192].try_into().unwrap();
    for i in 0..128 {
        bytes64to191[i] ^= scratchpad[i];
    }

    // Step 3C: Encrypt bytes 64..191 of Keccak state in the same manner as in the first part, but using the new keys
    let mut blocks: [[u8; 16]; 8] = [[0; 16]; 8];
    for i in 0..8 {
        blocks[i] = bytes64to191[i * 16..(i + 1) * 16].try_into().unwrap();
    }

    for i in 0..8 {
        let mut block = blocks[i];
        for j in 0..10 {
            block = aes_round(&block, &round_keys[j]);
        }
        scratchpad[i * 16..(i + 1) * 16].copy_from_slice(&block);
    }

    for i in 8..(SCRATCHPAD_SIZE / 16) {
        let mut block = [0; 16];
        for j in 0..16 {
            block[j] = scratchpad[(i - 8) * 16 + j];
        }
        for j in 0..10 {
            block = aes_round(&block, &round_keys[j]);
        }
        scratchpad[i * 16..(i + 1) * 16].copy_from_slice(&block);
    }

    // Step 3D: XOR result with second 128 bytes of scratchpad, encrypt again, and repeat till end of scratchpad
    for i in 0..(SCRATCHPAD_SIZE / 16) {
        let mut block = [0; 16];
        for j in 0..16 {
            block[j] = scratchpad[i * 16 + j];
        }
        for j in 0..10 {
            block = aes_round(&block, &round_keys[j]);
        }
        scratchpad[i * 16..(i + 1) * 16].copy_from_slice(&block);
    }

    // Step 3E: Replace bytes 64..191 of Keccak state with result
    final_state[64..192].copy_from_slice(&scratchpad[0..128]);

    // Step 3F: Pass Keccak state through Keccak-f with b = 1600
    let mut keccak: Keccak = Keccak::v256();
    keccak.update(&final_state);
    keccak.finalize(&mut final_state);

    // Step 3G: Use 2 low-order bits of first byte of state to select hash function
    let hash_function = final_state[0] & 0b11;

    // Step 3H: Apply hash function to Keccak state
    let hash: [u8; 32];
    match hash_function {
        0 => hash = blake256(&final_state),
        1 => hash = groestl256(&final_state),
        2 => hash = jh256(&final_state),
        3 => hash = skein256(&final_state),
        _ => panic!("Invalid hash function index"),
    }

    // Return hash
    return hash;
}

fn blake256(data: &[u8]) -> [u8; 32] {
    // Via blake2 crate
    let mut hasher = Blake2s256::new();
    hasher.update(data);
    hasher.finalize().as_slice().try_into().expect("Invalid length of the hash")
}

fn groestl256(data: &[u8]) -> [u8; 32] {
    // Via groestl-hash crate
    let mut hasher = Groestl256::new();
    hasher.update(data);
    hasher.finalize().as_slice().try_into().expect("Invalid length of the hash")
}

fn jh256(data: &[u8]) -> [u8; 32] {
    // Via jh crate
    let mut hasher = Jh256::new();
    hasher.update(data);
    hasher.finalize().as_slice().try_into().expect("Invalid length of the hash")
}

fn skein256(data: &[u8]) -> [u8; 32] {
    // Via skein crate
    let mut hasher = Skein256::<U32>::new();
    hasher.update(data);
    hasher.finalize().as_slice().try_into().expect("Invalid length of the hash")
}