package types

import sdk "github.com/cosmos/cosmos-sdk/types"

// ValidateBasic is used for validating the packet
func (p IbcJoinPoolPacketData) ValidateBasic() error {

	// TODO: Validate the packet data

	return nil
}

// GetBytes is a helper for serialising
func (p IbcJoinPoolPacketData) GetBytes() ([]byte, error) {
	var modulePacket IntergammPacketData

	modulePacket.Packet = &IntergammPacketData_IbcJoinPoolPacket{&p}
	b, err := ModuleCdc.MarshalJSON(&modulePacket)
	if err != nil {
		return nil, err
	}

	return sdk.MustSortJSON(b), nil
}
