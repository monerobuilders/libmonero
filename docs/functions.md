# Functions

Here is a list of all functions in this project:

- [Utils](#utils)
    - [is_valid_addr(address: &str) -> bool](#is_valid_addraddress-str---bool)
- [Keys](#keys)
    - [generate_seed(language: &str, seed_type: &str) -> Vec<&str>](#generate_seedlanguage-str-seed_type-str---vecstr)
    - [derive_hex_seed(mnemonic_seed: Vec<&str>) -> String](#derive_hex_seedmnemonic_seed-vecstr---string)


## Utils

- ### `is_valid_addr(address: &str) -> bool`

Checks if the given address is a valid Monero address \
Returns true if valid, returns false if invalid \
Example usage:
```rust
use libmonero::is_valid_addr;

libmonero::is_valid_addr("42wDfAgKWRYcdB7NtrZtabUx2d4jknPmZBT4KS9gxLP4VYBS4S8zH1nj3aByTHVQL1LRhKzoL1NDhKV3tXEt3KeKR5kR7uw");
> true
libmonero::is_valid_addr("42wDfAgKWRYcdB7NtrZtabUx2d4jknPmZBT4KS9gxLP4VYBS4S8zH1nj3aByTHVQL1LRhKzoL1NDhKV3tXEt3KeKR5kR7uw123123");
> false
```

## Keys

- ### `generate_seed(language: &str, seed_type: &str) -> Vec<&str>`

Generates a mnemonic phrase for given language and type \
Available types for now:
- `1626`: `en` (24-word mnemonic)
- `polyseed` (16-word mnemonic)

> DISCLAIMER: polyseed is not implemented yet

Example usage:
```rust
use libmonero::generate_seed;

let mnemonic = generate_seed("en", "1626");
println!("{:?}", mnemonic);
> ["tissue", "raking", "haunted", "huts", "afraid", "volcano", "howls", "liar", "egotistic", "befit", "rounded", "older", "bluntly", "imbalance", "pivot", "exotic", "tuxedo", "amaze", "mostly", "lukewarm", "macro", "vocal", "hounded", "biplane", "rounded"]
```

- ### `derive_hex_seed(mnemonic_seed: Vec<&str>) -> String`

Derives the hexadecimal seed from the given mnemonic seed \
Example usage:
```rust
use libmonero::derive_hex_seed;

let mnemonic = vec!["tissue", "raking", "haunted", "huts", "afraid", "volcano", "howls", "liar", "egotistic", "befit", "rounded", "older", "bluntly", "imbalance", "pivot", "exotic", "tuxedo", "amaze", "mostly", "lukewarm", "macro", "vocal", "hounded", "biplane", "rounded"];
let hex_seed = derive_hex_seed(mnemonic);
println!("{}", hex_seed);
> "f7b3beabc9bd6ced864096c0891a8fdf94dc714178a09828775dba01b4df9ab8"
```
