# Structs And Implementations

This is the documentation for all public structs and implementations in the library. \
Here is a list of all structs and implementations in this project:

- [Wallet](#wallet)
    - [new(language: &str, seed_type: &str, network: i8) -> Wallet](#newlanguage-str-seed_type-str-network-i8---wallet)

## Wallet

This struct is used to store all the information about a wallet. \
It is used to store the mnemonic seed, the private spend key, the private view key, the public spend key, the public view key, and the address. \
You can use these functions via `use libmonero::wallet`
> It can also be used as a normal wallet which can make transactions, but this is not implemented yet and will be implemented in the future.

---

- ### `new(language: &str, seed_type: &str, network: i8) -> Wallet`

Creates a new wallet with the given language, seed type, and network. \
Available languages: `en`, `eo`, `fr`, `it`, `jp`, `lj`, `pt`, `ru`... (same as generate_seed) \
Available seed types: `original`, `mymonero` (same as generate_seed) \
Available networks: `0` (Mainnet), `1` (Testnet) (same as derive_address) \

Example usage:
```rust
let wallet = wallet::Wallet::new("en", "original", 0);
println!("mnemonic: {:?}", wallet.mnemonic);
> mnemonic: ["twofold", "misery", "idols", "worry", "intended", "dunes", "vain", "unwind", "bounced", "enigma", "mural", "asked", "degrees", "react", "nautical", "nomad", "utensils", "remedy", "hockey", "corrode", "eternal", "dizzy", "rally", "jabbed", "utensils"]
println!("hex_seed: {:?}", wallet.hex_seed);
> hex_seed: "dbfa4cde9e2890cd1878c83705a2f283cfc212dc84a006cb93ee1f1adc5814bc"
println!("priv_sk: {:?}", wallet.priv_sk);
> priv_sk: "acdfbcdf7ce6c504e3ba243774e55d9ecec212dc84a006cb93ee1f1adc58140c"
println!("priv_vk: {:?}", wallet.priv_vk);
> priv_vk: "7f65a5f4a4a68af77b3a97770c76d66dd3f8f959121ec8969ebcc2c24ea8e80f"
println!("pub_sk: {:?}", wallet.pub_sk);
> pub_sk: "1ecddc60ba4baacc12fcba53703cbecc8302b0e8703332bd69294fd85ba26ede"
println!("pub_vk: {:?}", wallet.pub_vk);
> pub_vk: "803844e0a71eda67272e2fec385dbc764b82412f1e66fb509c579f54c12b8106"
println!("address: {:?}", wallet.address);
> address: "42npBNptGRPb8mxLhiaYsPbD2B8Qs9z41YgXGf8jtzeHeDY9cndneC1JFiQB5dn291LncLYaje1WaEV2Si65FDsn1kqvwAy"
```
