package types

// ValidateBasic is used for validating the packet
func (p IbcWithdrawPacketData) ValidateBasic() error {

	// TODO: Validate the packet data

	return nil
}

// GetBytes is a helper for serialising
func (p IbcWithdrawPacketData) GetBytes() ([]byte, error) {
	var modulePacket IntergammPacketData

	modulePacket.Packet = &IntergammPacketData_IbcWithdrawPacket{&p}

	return modulePacket.Marshal()
}
