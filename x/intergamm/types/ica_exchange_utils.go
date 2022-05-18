package types

import (
	"encoding/hex"

	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"
	"github.com/pkg/errors"
	"github.com/tendermint/tendermint/crypto"
)

func HashPacket(packet channeltypes.Packet) []byte {
	packetData, err := packet.Marshal()
	if err != nil {
		panic(errors.Wrap(err, "IBC packet must marshal"))
	}
	return crypto.Sha256(packetData)
}

func HashPacketStr(packet channeltypes.Packet) string {
	return hex.EncodeToString(HashPacket(packet))
}
