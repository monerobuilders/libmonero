use crate::mnemonics::WORDSETS1626;
use crc32fast::Hasher;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::convert::TryFrom;

// Returns cryptographically secure random element of the given array
fn secure_random_element<'x>(array: &'x [&'x str]) -> &'x str {
    let seed: [u8; 32] = rand::thread_rng().gen();
    let mut rng = StdRng::from_seed(seed);
    let index = rng.gen_range(0..array.len());
    array[index]
}

// Calculates CRC32 for given array (probably the seed)
fn get_checksum_index(array: &[&str], prefix_length: usize) -> usize {
    let mut trimmed_words: String = String::new();
    for word in array {
        trimmed_words.push_str(&word[0..prefix_length]);
    }
    let mut hasher = Hasher::new();
    hasher.update(trimmed_words.as_bytes());
    let checksum = usize::try_from(hasher.finalize() as u32).unwrap() % array.len();
    checksum
}

// Creates a cryptographically secure 1626-word type seed for the given language
pub fn create_seed(language: &str, is_polyseed: bool) -> Vec<&str> {
    if (is_polyseed) {
        // TODO: Implement polyseed
        panic!("NOT IMPLEMENTED YET")
    } else {
        let mut seed: Vec<&str> = Vec::new();
        let mut prefix_len: usize = 3;
        for wordset in WORDSETS1626.iter() {
            if wordset.name == language {
                prefix_len = wordset.prefix_len as usize;
                for _ in 0..24 {
                    let word = secure_random_element(&wordset.words[..]);
                    seed.push(word);
                }
                break;
            } else {
                continue;
            }
        }
        if seed.is_empty() {
            panic!("Language not found");
        }
        let checksum_index = get_checksum_index(&seed, prefix_len);
        seed.push(&seed[checksum_index]);
        seed
    }
}
