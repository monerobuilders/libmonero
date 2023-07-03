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

---

## Wallet

---

- ### `GenerateMnemonicSeed(language string) (string, error)`

Generates a mnemonic phrase for given language \
Example usage:
```go
mnemonic, _ := monero.GenerateMnemonic("en")
fmt.Println(mnemonic)
> "tissue raking haunted huts afraid volcano howls liar egotistic befit rounded older bluntly imbalance pivot exotic tuxedo amaze mostly lukewarm macro vocal hounded biplane rounded"
```

---

- ### `DeriveHexSeedFromMnemonicSeed(mnemonic string, language string) (string, error)`

Derives the hexadecimal seed from the given mnemonic seed and language \
Example usage:
```go
theMnemonic := "tissue raking haunted huts afraid volcano howls liar egotistic befit rounded older bluntly imbalance pivot exotic tuxedo amaze mostly lukewarm macro vocal hounded biplane rounded"
hexadecimalSeed, _ := monero.DeriveHexSeedFromMnemonicSeed(theMnemonic, "en")
fmt.Println(hexadecimalSeed)
> "f7b3beabc9bd6ced864096c0891a8fdf94dc714178a09828775dba01b4df9ab8"
```






