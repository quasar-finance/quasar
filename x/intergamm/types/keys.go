package types

import "bytes"

const (
	// ModuleName defines the module name
	ModuleName = "intergamm"

	// StoreKey defines the primary module store key
	StoreKey = ModuleName

	// RouterKey is the message route for slashing
	RouterKey = ModuleName

	// QuerierRoute defines the module's query routing key
	QuerierRoute = ModuleName

	// MemStoreKey defines the in-memory store key
	MemStoreKey = "mem_intergamm"

	// Separator used in the keys
	Sep = ":"
)

func KeyPrefix(p string) []byte {
	return []byte(p)
}

var (
	SepByte     = []byte(":")
	PortInfoKBP = []byte{0x01}
)

func CreatePortIDKey(portID string) []byte {
	return []byte(portID)
}

func CreateChainIDPortIDKey(chainID, portID string) []byte {
	var b bytes.Buffer
	b.WriteString(chainID)
	b.WriteString(Sep)
	b.WriteString(portID)
	return b.Bytes()
}
