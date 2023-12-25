use crate::mnemonics::{WORDSETS1626, Wordset};
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

// Calculates CRC32 checksum index for given array (probably the seed)
fn get_checksum_index(array: &[&str], prefix_length: usize) -> usize {
    let mut trimmed_words: String = String::new();
    for word in array {
        trimmed_words.push_str(&word[0..prefix_length]);
    }
    let mut hasher = Hasher::new();
    hasher.update(trimmed_words.as_bytes());
    usize::try_from(hasher.finalize()).unwrap() % array.len()
}

// Creates a cryptographically secure 1626-word type seed for the given language
pub fn generate_seed(language: &str, is_polyseed: bool) -> Vec<&str> {
    if is_polyseed {
        // TODO: Implement polyseed
        panic!("NOT IMPLEMENTED YET")
    } else {
        let mut seed: Vec<&str> = Vec::new();
        let mut prefix_len: usize = 3;
        for wordset in WORDSETS1626.iter() {
            if wordset.name == language {
                prefix_len = wordset.prefix_len;
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
        seed.push(seed[checksum_index]);
        seed
    }
}

fn swap_endian_4_byte(s: &str) -> String {
    if s.len() != 8 {
        panic!("Invalid length of string");
    }
    format!("{}{}{}{}", &s[6..8], &s[4..6], &s[2..4], &s[0..2])
}

fn find_index(array: &[&str], word: &str) -> isize {
    array.iter().position(|&x| x == word).map(|i| i as isize).unwrap_or(-1)
}


pub fn derive_hex_seed(mut mnemonic_seed: Vec<&str>) -> String {
    // Find the wordset for the given seed
    let mut the_wordset = &Wordset {
        name: "invalid",
        prefix_len: 0,
        words: [""; 1626],
    }; // This is given for checking in future if the wordset was found
    for wordset in WORDSETS1626.iter() {
        for word in wordset.words.iter() {
            if mnemonic_seed.contains(word) {
                the_wordset = wordset;
                break;
            }
        }
    }
    if the_wordset.name == "invalid" {
        panic!("The wordset could not be found for given seed, please check your seed")
    }

    // Declare variables for later use
    let mut out = String::new();
    let n = the_wordset.words.len();
    let mut checksum_word = String::new();

    // Check if seed is valid
    if (the_wordset.prefix_len == 0 && mnemonic_seed.len() % 3 != 0)|| (the_wordset.prefix_len > 0 && mnemonic_seed.len() % 3 == 2) {
        panic!("You have entered too few words, please check your seed")
    } else if the_wordset.prefix_len > 0 && mnemonic_seed.len() % 3 == 0 {
        panic!("You seem to be missing the last word of your seed, please check your seed")
    } else if the_wordset.prefix_len > 0 {
        checksum_word = mnemonic_seed.pop().unwrap().to_string();
    }

    // Get list of truncated words
    let mut trunc_words: Vec<&str> = Vec::new();
    if the_wordset.prefix_len > 0 {
        for word in the_wordset.words.iter() {
            trunc_words.push(&word[..the_wordset.prefix_len]);
        }
    }

    // Derive hex seed
    for i in (0..mnemonic_seed.len()).step_by(3) {
        let w1;
        let w2;
        let w3;
        if the_wordset.prefix_len == 0 {
            w1 = find_index(&the_wordset.words, mnemonic_seed[i]);
            w2 = find_index(&the_wordset.words, mnemonic_seed[i + 1]);
            w3 = find_index(&the_wordset.words, mnemonic_seed[i + 2]);
        } else {
            w1 = find_index(
                &trunc_words,
                &mnemonic_seed[i][..the_wordset.prefix_len],
            );
            w2 = find_index(
                &trunc_words,
                &mnemonic_seed[i + 1][..the_wordset.prefix_len],
            );
            w3 = find_index(
                &trunc_words,
                &mnemonic_seed[i + 2][..the_wordset.prefix_len],
            );
        }

        if w1 == -1 || w2 == -1 || w3 == -1 {
            panic!("Invalid word in seed, please check your seed")
        }

        let x: usize = (w1 + n as isize * ((n as isize - w1 + w2) % n as isize) + n as isize * n as isize * ((n as isize - w2 + w3) % n as isize)).try_into().unwrap();
        if x % n != w1 as usize {
            panic!("An error occured while deriving hex seed, please try again later");
        }
        let swapped = swap_endian_4_byte(&format!("{:08x}", x));
        out += &swapped;
    }

    if the_wordset.prefix_len > 0 {
        let index = get_checksum_index(&mnemonic_seed, the_wordset.prefix_len);
        let expected_checksum_word = &mnemonic_seed[index];
        if expected_checksum_word[..the_wordset.prefix_len] != checksum_word[..the_wordset.prefix_len] {
            panic!("Your seed could not be verified via the last word checksum, please check your seed")
        }
    }
    out
}