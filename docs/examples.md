# Examples

This file contains various examples of how to use the library.
Here is a list of the examples:

- [Creating a new wallet](#creating-a-new-wallet)

## Creating a new wallet

You can create a new wallet using the `new` function in the `Wallet` struct.

```rust
use libmonero::wallet::Wallet;

fn main() {
    // This will create a new original (25-word) english mnemonic wallet in Monero mainnet with a random seed.
    let wallet = Wallet::new("en", "original", 0);
    // Now, for example get the private spend key and print it.
    println!("{}", wallet.priv_sk);
}
```

The above example will print something like this to console: \
`059fd750b44f957d338f3a6aa09cd2eb669df6463c782217b22917f84b9b9109`

You can use the generated wallet to access the wallet's keys, address, etc.

---