package types

import "encoding/binary"

var _ binary.ByteOrder

const (
	// PoolSpotPriceKeyPrefix is the prefix to retrieve all PoolSpotPrice
	PoolSpotPriceKeyPrefix = "PoolSpotPrice/value/"
)

// PoolSpotPriceKey returns the store key to retrieve a PoolSpotPrice from the index fields
func PoolSpotPriceKey(
	poolId string,
	denomIn string,
	denomOut string,
) []byte {
	var key []byte

	poolIdBytes := []byte(poolId)
	key = append(key, poolIdBytes...)
	key = append(key, []byte("#")...)

	denomInBytes := []byte(denomIn)
	key = append(key, denomInBytes...)
	key = append(key, []byte("#")...)

	denomOutBytes := []byte(denomOut)
	key = append(key, denomOutBytes...)
	key = append(key, []byte("#")...)

	return key
}
