package types

import (
	"bytes"
	"strconv"
)

const (
	// ModuleName defines the module name
	ModuleName = "qoracle"

	// StoreKey defines the primary module store key
	StoreKey = ModuleName

	// RouterKey is the message route for slashing
	RouterKey = ModuleName

	// QuerierRoute defines the module's query routing key
	QuerierRoute = ModuleName

	// MemStoreKey defines the in-memory store key
	MemStoreKey = "mem_qoracle"
)

func KeyPrefix(p string) []byte {
	return []byte(p)
}

const (
	PoolPositionKey = "PoolPosition-value-"
)

var (
	PoolPositionKBP = []byte{0x01}
)

func CreatePoolPositionKey(poolID uint64) []byte {
	var b bytes.Buffer
	strEpochday := strconv.FormatUint(poolID, 10)
	b.WriteString(strEpochday)
	return b.Bytes()
}
