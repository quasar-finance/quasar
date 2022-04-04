package types

import "encoding/binary"

var _ binary.ByteOrder

const (
	// PoolPositionKeyPrefix is the prefix to retrieve all PoolPosition
	PoolPositionKeyPrefix = "PoolPosition/value/"
)

// PoolPositionKey returns the store key to retrieve a PoolPosition from the index fields
func PoolPositionKey(
	poolId string,
) []byte {
	var key []byte

	poolIdBytes := []byte(poolId)
	key = append(key, poolIdBytes...)
	key = append(key, []byte("/")...)

	return key
}
