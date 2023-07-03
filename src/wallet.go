package monero

import (
	"crypto/rand"
	"errors"
	"fmt"
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
// Returns error if there has been an error
func GenerateMnemonicSeed(language string) (string, error) {
	wordList := wordSets[language].words
	prefixLen := wordSets[language].prefixLen
	if wordList == nil {
		return "", errors.New("invalid language")
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

// Implements swapEndian4Byte that is used in DeriveHexSeedFromMnemonicSeed
func swapEndian4Byte(str string) (string, error) {
	if len(str) != 8 {
		return "", errors.New("invalid input length: " + string(rune(len(str))))
	}
	return str[6:8] + str[4:6] + str[2:4] + str[0:2], nil
}

// Returns index of given word in the given array
// Returns -1 if not found in the array
func findIndex(array []string, word string) int {
	for i, val := range array {
		if val == word {
			return i
		}
	}
	return -1
}

// DeriveHexSeedFromMnemonicSeed : Derives and returns hex seed from given mnemonic
// Returns error if there has been an error
func DeriveHexSeedFromMnemonicSeed(mnemonic string, language string) (string, error) {
	wordset := wordSets[language]
	if wordset.words == nil {
		return "", errors.New("invalid language")
	}
	out := ""
	n := len(wordset.words)
	wordsList := strings.Split(mnemonic, " ")
	checksumWord := ""
	if (wordset.prefixLen == 0 && len(wordsList)%3 != 0) ||
		(wordset.prefixLen > 0 && len(wordsList)%3 == 2) {
		return "", errors.New("you've entered too few words, please try again")
	}
	if wordset.prefixLen > 0 && len(wordsList)%3 == 0 {
		return "", errors.New("you seem to be missing the last word in your private key, please try again")
	}
	if wordset.prefixLen > 0 {
		checksumWord = wordsList[len(wordsList)-1]
		wordsList = wordsList[:len(wordsList)-1]
	}
	for i := 0; i < len(wordsList); i += 3 {
		var w1, w2, w3 int
		if wordset.prefixLen == 0 {
			w1 = findIndex(wordset.words, wordsList[i])
			w2 = findIndex(wordset.words, wordsList[i+1])
			w3 = findIndex(wordset.words, wordsList[i+2])
		} else {
			w1 = findIndex(wordset.truncWords, wordsList[i][:wordset.prefixLen])
			w2 = findIndex(wordset.truncWords, wordsList[i+1][:wordset.prefixLen])
			w3 = findIndex(wordset.truncWords, wordsList[i+2][:wordset.prefixLen])
		}
		if w1 == -1 || w2 == -1 || w3 == -1 {
			return "", errors.New("invalid word in mnemonic")
		}
		x := w1 + n*((n-w1+w2)%n) + n*n*((n-w2+w3)%n)
		if x%n != w1 {
			return "", errors.New("something went wrong when decoding your private key, please try again")
		}
		swapped, err := swapEndian4Byte(fmt.Sprintf("%08x", x))
		if err != nil {
			return "", err
		}
		out += swapped
	}
	if wordset.prefixLen > 0 {
		index := getChecksumIndex(wordsList, wordset.prefixLen)
		expectedChecksumWord := wordsList[index]
		if expectedChecksumWord[:wordset.prefixLen] != checksumWord[:wordset.prefixLen] {
			return "", errors.New("your private key could not be verified, please try again")
		}
	}
	return out, nil
}
