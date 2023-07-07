# Functions

Here you can see all functions in this library

Overview list:
- [Utils](#utils)
- [Wallet](#wallet)
---
## Utils
- ### `Version() string`

Returns the version of the library \
Example usage:
```go
fmt.Println(monero.Version())
> "0.1.0"
```

- ### `ValidateAddress(address string) bool`

Checks if the given address is a valid Monero address \
Returns true if valid, returns false if invalid \
Example usage:
```go
validAddress := "42wDfAgKWRYcdB7NtrZtabUx2d4jknPmZBT4KS9gxLP4VYBS4S8zH1nj3aByTHVQL1LRhKzoL1NDhKV3tXEt3KeKR5kR7uw"
invalidAddress := "42wDfAgKWRYcdB7NtrZtabUx2d4jknPmZBT4KS9gxLP4VYBS4S8zH1nj3aByTHVQL1LRhKzoL1NDhKV3tXEt3KeKR5kR7uw123123"
fmt.Println(monero.ValidateAddress(validAddress))
fmt.Println(monero.ValidateAddress(invalidAddress))
> true
> false
```
---

## Wallet

- ### `GenerateMnemonicSeed(language string) (string, error)`

Generates a mnemonic phrase for given language \
Example usage:
```go
mnemonic, _ := monero.GenerateMnemonic("en")
fmt.Println(mnemonic)
> "tissue raking haunted huts afraid volcano howls liar egotistic befit rounded older bluntly imbalance pivot exotic tuxedo amaze mostly lukewarm macro vocal hounded biplane rounded"
```

- ### `DeriveHexSeedFromMnemonicSeed(mnemonic string, language string) (string, error)`

Derives the hexadecimal seed from the given mnemonic seed and language \
Example usage:
```go
theMnemonic := "tissue raking haunted huts afraid volcano howls liar egotistic befit rounded older bluntly imbalance pivot exotic tuxedo amaze mostly lukewarm macro vocal hounded biplane rounded"
hexadecimalSeed, _ := monero.DeriveHexSeedFromMnemonicSeed(theMnemonic, "en")
fmt.Println(hexadecimalSeed)
> "f7b3beabc9bd6ced864096c0891a8fdf94dc714178a09828775dba01b4df9ab8"
```

- ### `DerivePrivateKeysFromHexSeed(hexSeed string) (string, string, error)`

Derives private spend key and private view key from given hexadecimal seed \
Example usage:
```go
hexadecimalSeed := "f7b3beabc9bd6ced864096c0891a8fdf94dc714178a09828775dba01b4df9ab8"
privateSpendKey, privateViewKey, _ := monero.DerivePrivateSpendKeyFromHexSeed(hexadecimalSeed)
fmt.Println(privateSpendKey)
fmt.Println(privateViewKey)
> "c8982eada77ba2245183f2bff85dfaf993dc714178a09828775dba01b4df9a08"
> "0d13a94c82d7a60abb54d2217d38935c3f715295e30378f8848a1ca1abc8d908"
```

- ### `DerivePrivVKFromPrivSK(privateSpendKey string) (string, error)`

Derives private view key from given private spend key \
Example usage:
```go
privateSpendKey := "c8982eada77ba2245183f2bff85dfaf993dc714178a09828775dba01b4df9a08"
privateViewKey, _ := monero.DerivePrivateViewKeyFromPrivateSpendKey(privateSpendKey)
fmt.Println(privateViewKey)
> "0d13a94c82d7a60abb54d2217d38935c3f715295e30378f8848a1ca1abc8d908"
```

- ### `DerivePublicKeyFromPrivateKey(privateKey string) (string, error)`

Derives public key from given private key (can be spend key or view key) \
Example usage:
```go
privSpendKey := "c8982eada77ba2245183f2bff85dfaf993dc714178a09828775dba01b4df9a08"
pubSpendKey := monero.DerivePublicKeyFromPrivateKey(privSpendKey)
fmt.Println(pubSpendKey)
> "e78d891dd2be407f24e6470caad956e1b746ae0b41cd8252f96684090bc05d95"
---
privViewKey := "0d13a94c82d7a60abb54d2217d38935c3f715295e30378f8848a1ca1abc8d908"
pubViewKey := monero.DerivePublicKeyFromPrivateKey(privViewKey)
fmt.Println(pubViewKey)
> "157d278aa3aee4e11c5a8243a43a78527a2691009562b8c18654975f1347cb47"
```

- ### `DeriveAddressFromPubKeys(publicSpendKey string, publicViewKey string, network string) (string, error)`

Derives address from given public spend key and public view key and network \
Network can either be "moneromainnet" or "monerotestnet" \
Example usage:
```go
pubSpendKey := "e78d891dd2be407f24e6470caad956e1b746ae0b41cd8252f96684090bc05d95"
pubViewKey := "157d278aa3aee4e11c5a8243a43a78527a2691009562b8c18654975f1347cb47"
address, _ := monero.DeriveAddressFromPubKeys(pubSpendKey, pubViewKey, "moneromainnet")
fmt.Println(address)
> "4AQ3jTJg91yNGTXjo9iWr1ekjBGJ5mM6HEsxKqoKddHnRwJTVJYnyLXeerff6iTys5Eo8dyG87tfqZNS5CcSd7U694YiR8J"
```




