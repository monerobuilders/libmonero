# Functions

Here is a list of all functions in this project:

- [Utils](#utils)
    - [is_valid_addr(address: &str) -> bool](#is_valid_addraddress-str---bool)
- [Keys](#keys)
    - [generate_seed(language: &str, seed_type: &str) -> Vec<&str>](#generate_seedlanguage-str-seed_type-str---vecstr)
    - [derive_hex_seed(mnemonic_seed: Vec<&str>) -> String](#derive_hex_seedmnemonic_seed-vecstr---string)
    - [derive_priv_keys(hex_seed: String) -> Vec<String\>](#derive_priv_keyshex_seed-string---vecstring)
    - [derive_priv_vk_from_priv_sk(private_spend_key: String) -> String](#derive_priv_vk_from_priv_skprivate_spend_key-string---string)
    - [derive_pub_key(private_key: String) -> String](#derive_pub_keyprivate_key-string---string)
    - [derive_address(public_spend_key: String, public_view_key: String, network: i8) -> String](#derive_addresspublic_spend_key-string-public_view_key-string-network-i8---string)


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
- `original`: (25-word original type) 
    - `en` (English)
    - `eo` (Esperanto)
    - `fr` (French)
    - `it` (Italian)
    - `jp` (Japanese) (Works but not recommended)
    - `lj` (Lojban)
    - `pt` (Portuguese)
    - `ru` (Russian)
- `mymonero`: (13-word MyMonero type)
    - `en`, `eo`, `fr`, `it`, `jp`, `lj`, `pt`, `ru` (same as original)
- `polyseed` (TO BE IMPLEMENTED)
> DISCLAIMER: polyseed is not implemented yet

Example usage:
```rust
use libmonero::generate_seed;

let mnemonic = generate_seed("en", "original");
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

- ### `derive_priv_keys(hex_seed: String) -> Vec<String>`

Derives the private keys from the given hexadecimal seed \
First one in the Vec is the private spend key, second one is the private view key \
Example usage:
```rust
use libmonero::derive_priv_keys;

let hex_seed = "f7b3beabc9bd6ced864096c0891a8fdf94dc714178a09828775dba01b4df9ab8";
let priv_keys = derive_priv_keys(hex_seed);
println!("{:?}", priv_keys);
> ["c8982eada77ba2245183f2bff85dfaf993dc714178a09828775dba01b4df9a08", "0d13a94c82d7a60abb54d2217d38935c3f715295e30378f8848a1ca1abc8d908"]
```

- ### `derive_priv_vk_from_priv_sk(private_spend_key: String) -> String`

Derives the private view key from the given private spend key \
Example usage:
```rust
use libmonero::derive_priv_vk_from_priv_sk;

let private_spend_key = "c8982eada77ba2245183f2bff85dfaf993dc714178a09828775dba01b4df9a08";
let private_view_key = derive_priv_vk_from_priv_sk(private_spend_key);
println!("{}", private_view_key);
> "0d13a94c82d7a60abb54d2217d38935c3f715295e30378f8848a1ca1abc8d908"
```

- ### `derive_pub_key(private_key: String) -> String`

Derives the public key from the given private key (spend or view) \
Example usage:
```rust
use libmonero::derive_pub_key;

let private_spend_key = "c8982eada77ba2245183f2bff85dfaf993dc714178a09828775dba01b4df9a08"
let private_view_key = "0d13a94c82d7a60abb54d2217d38935c3f715295e30378f8848a1ca1abc8d908"
let public_spend_key = derive_pub_key(private_spend_key);
let public_view_key = derive_pub_key(private_view_key);
println!("{}", public_spend_key);
> "e78d891dd2be407f24e6470caad956e1b746ae0b41cd8252f96684090bc05d95"
println!("{}", public_view_key);
> "157d278aa3aee4e11c5a8243a43a78527a2691009562b8c18654975f1347cb47"
```

- ### `derive_address(public_spend_key: String, public_view_key: String, network: i8) -> String`

Derives the address from the given public spend and view keys and network \
Networks:
- `0`: Mainnet
- `1`: Testnet

Example usage:
```rust
use libmonero::derive_address;

let public_spend_key = "e78d891dd2be407f24e6470caad956e1b746ae0b41cd8252f96684090bc05d95";
let public_view_key = "157d278aa3aee4e11c5a8243a43a78527a2691009562b8c18654975f1347cb47";
let address = derive_address(public_spend_key, public_view_key, 0);
println!("{}", address);
> "4AQ3jTJg91yNGTXjo9iWr1ekjBGJ5mM6HEsxKqoKddHnRwJTVJYnyLXeerff6iTys5Eo8dyG87tfqZNS5CcSd7U694YiR8J"
```