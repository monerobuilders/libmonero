package monero

import (
	"crypto/rand"
	"errors"
	"hash/crc32"
	"math/big"
	"strings"
)

// Generates and returns a random word from the mnemonic word list, which is a list of 1626 words
func randomWord(wordList []string) string {
	num, _ := rand.Int(rand.Reader, big.NewInt(int64(len(wordList))))
	return wordList[num.Int64()]
}

// Generates and returns the index of the checksum word (25th word) in the mnemonic
// The checksum is calculated by taking the first prefixLength letters of each word in the mnemonic
// and calculating the CRC32 checksum of the resulting string. The index of the checksum
// word is the checksum modulo the number of words in the mnemonic
func getChecksumIndex(mnemonics []string, prefixLength int) int {
	trimmedWords := ""
	for _, word := range mnemonics {
		trimmedWords += word[:prefixLength]
	}
	checksum := crc32.ChecksumIEEE([]byte(trimmedWords))
	index := int(checksum) % len(mnemonics)
	return index
}

// GenerateMnemonicSeed : Generates and returns a 25 word mnemonic
func GenerateMnemonicSeed(language string) (string, error) {
	wordList := wordSets[language].words
	prefixLen := wordSets[language].prefixLen
	if wordList == nil {
		return "", errors.New("language not supported")
	}
	// Continue if language is supported
	var mnemonic []string
	for i := 0; i < 24; i++ {
		mnemonic = append(mnemonic, randomWord(wordList))
	}
	checksumIndex := getChecksumIndex(mnemonic, prefixLen)
	mnemonic = append(mnemonic, mnemonic[checksumIndex])
	return strings.Join(mnemonic, " "), nil
}
