package types

// ValidateBasic is used for validating the packet
func (p IbcJoinPoolPacketData) ValidateBasic() error {

	// TODO: Validate the packet data

	return nil
}

// GetBytes is a helper for serialising
func (p IbcJoinPoolPacketData) GetBytes() ([]byte, error) {
	var modulePacket IntergammPacketData

	modulePacket.Packet = &IntergammPacketData_IbcJoinPoolPacket{&p}

	return modulePacket.Marshal()
}
