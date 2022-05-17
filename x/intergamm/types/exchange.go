package types

import (
	"encoding/hex"

	"github.com/tendermint/tendermint/crypto"
)

func HashPacketData(packetData []byte) []byte {
	return crypto.Sha256(packetData)

}

func HashPacketDataStr(packetData []byte) string {
	return hex.EncodeToString(HashPacketData(packetData))
}
