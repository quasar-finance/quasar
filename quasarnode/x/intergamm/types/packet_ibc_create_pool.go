package types

// ValidateBasic is used for validating the packet
func (p IbcCreatePoolPacketData) ValidateBasic() error {

	// TODO: Validate the packet data

	return nil
}

// GetBytes is a helper for serialising
func (p IbcCreatePoolPacketData) GetBytes() ([]byte, error) {
	var modulePacket IntergammPacketData

	modulePacket.Packet = &IntergammPacketData_IbcCreatePoolPacket{&p}

	return modulePacket.Marshal()
}
