package types

const (
	// EventTypePacket is the general event for IBC layer before getting into icq processing
	EventTypePacket = "packet"
	// EventTypeOsmosisRequest is the type for the osmosis ICQ request events
	EventTypeOsmosisRequest = "osmosis_request"
	// EventTypeOsmosisPacketAcknowledgement is the type for the event osmosis ICQ acknowledgement
	EventTypeOsmosisPacketAcknowledgement = "osmosis_packet_acknowledgement"
)

const (
	// AttributeKeyPacketChannelId is the attribute for the packet channel id
	AttributeKeyPacketChannelID = "packet_channel_id"
	// AttributePacketSequence is the attribute for the packet sequence
	AttributeKeyPacketSequence = "packet_sequence"
	// AttributeKeyTitle is the attribute for icq request titles
	AttributeKeyTitle = "title"
	// AttributeKeyAckSuccess is the attribute which indicates whether IBC ack is successful
	AttributeKeyAckSuccess = "ack_success"
	// AttributeError is the attribute key for the error
	AttributeKeyError = "error"
)
