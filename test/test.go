package main

import (
	"fmt"
	monero "github.com/openmonero/monero.go/src"
)

func main() {
	mne, error := monero.GenerateMnemonic("en")
	if error != nil {
		panic(error)
		return
	}
	fmt.Println(mne)
}
