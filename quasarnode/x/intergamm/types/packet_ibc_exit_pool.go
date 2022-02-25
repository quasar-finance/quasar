package types

// ValidateBasic is used for validating the packet
func (p IbcExitPoolPacketData) ValidateBasic() error {

	// TODO: Validate the packet data

	return nil
}

// GetBytes is a helper for serialising
func (p IbcExitPoolPacketData) GetBytes() ([]byte, error) {
	var modulePacket IntergammPacketData

	modulePacket.Packet = &IntergammPacketData_IbcExitPoolPacket{&p}

	return modulePacket.Marshal()
}
