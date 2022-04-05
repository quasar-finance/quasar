package types

import sdk "github.com/cosmos/cosmos-sdk/types"

// ValidateBasic is used for validating the packet
func (p IbcExitPoolPacketData) ValidateBasic() error {

	// TODO: Validate the packet data

	return nil
}

// GetBytes is a helper for serialising
func (p IbcExitPoolPacketData) GetBytes() ([]byte, error) {
	var modulePacket IntergammPacketData

	modulePacket.Packet = &IntergammPacketData_IbcExitPoolPacket{&p}
	b, err := ModuleCdc.MarshalJSON(&modulePacket)
	if err != nil {
		return nil, err
	}

	return sdk.MustSortJSON(b), nil
}
