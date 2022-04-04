package types

import "encoding/binary"

var _ binary.ByteOrder

const (
	// PoolInfoKeyPrefix is the prefix to retrieve all PoolInfo
	PoolInfoKeyPrefix = "PoolInfo/value/"
)

// PoolInfoKey returns the store key to retrieve a PoolInfo from the index fields
func PoolInfoKey(
	poolId string,
) []byte {
	var key []byte

	poolIdBytes := []byte(poolId)
	key = append(key, poolIdBytes...)
	key = append(key, []byte("/")...)

	return key
}
