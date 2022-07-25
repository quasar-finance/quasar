package types

const (
	// EventTypeCoinRatesRequest is the type for the event CoinRatesRequest
	EventTypeCoinRatesRequest = "coin_rates_request"
	// EventTypeOraclePacketAcknowledgement	is the type for the event OraclePacketAcknowledgement
	EventTypeOraclePacketAcknowledgement = "oracle_packet_acknowledgement"
	// EventTypeOsmosisPacketAcknowledgement is the type for the event OsmosisPacketAcknowledgement
	EventTypeOsmosisPacketAcknowledgement = "osmosis_packet_acknowledgement"

	//AtributePacketSequence is the attribute for the packet sequence
	AtributePacketSequence = "packet_sequence"
	// AttributeError is the attribute key for the error
	AttributeError = "error"
	// AttributeEpochIdentifier is the attribute key for the epoch identifier
	AttributeEpochIdentifier = "epoch_identifier"
	// AttributeEpochNumber is the attribute key for the epoch number
	AttributeEpochNumber = "epoch_number"
)
