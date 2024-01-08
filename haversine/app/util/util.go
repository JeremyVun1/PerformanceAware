package util

import (
	"fmt"
	"os"
)

func ReadFile(filename string) []byte {
	bytes, err := os.ReadFile(fmt.Sprintf("data/%s", filename))
	Check(err)
	return bytes
}

func Check(err error) {
	if err != nil {
		panic(err)
	}
}
