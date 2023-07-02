# Functions

Here you can see all functions in this library

Overview list:
- [Utils](#utils)
- [Wallet](#wallet)
---
## Utils
- ### `Version() string`

Returns the version of the library 
Example usage:
```go
fmt.Println(monero.Version())
> "0.1.0"
```

---

## Wallet
- ### `GenerateMnemonicSeed(language: string) (string, error)`

Generates a mnemonic phrase for given language \
Supported languages: `en` \
Example usage:
```go
mnemonic, _ := monero.GenerateMnemonic("en")
fmt.Println(mnemonic)
> "tissue raking haunted huts afraid volcano howls liar egotistic befit rounded older bluntly imbalance pivot exotic tuxedo amaze mostly lukewarm macro vocal hounded biplane rounded"
```







