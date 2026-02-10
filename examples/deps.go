// This file exists only to ensure go.sum includes all required dependencies.
// It can be deleted after running `go mod tidy`.

package main

import (
	_ "github.com/ChefBingbong/viem-go/abi"
	_ "github.com/ethereum/go-ethereum/common"
	_ "golang.org/x/crypto/sha3"
)

func main() {}
