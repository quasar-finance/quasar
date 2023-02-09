package types

const (
	// EventTypeCoinRatesRequest is the type for the event CoinRatesRequest
	EventTypeCoinRatesRequest = "coin_rates_request"
	// EventTypeOraclePacketAcknowledgement	is the type for the event OraclePacketAcknowledgement
	EventTypeOraclePacketAcknowledgement = "oracle_packet_acknowledgement"
)

const (
	// AttributeKeyPacketChannelId is the attribute for the packet channel id
	AttributeKeyPacketChannelId = "packet_channel_id"
	// AttributePacketSequence is the attribute for the packet sequence
	AttributeKeyPacketSequence = "packet_sequence"
	// AttributeError is the attribute key for the error
	AttributeKeyError = "error"
	// AttributeKeyClientID is the attribute for oracle request client id
	AttributeKeyClientID = "client_id"
	// AttributeKeyScriptID is the attribute for oracle request script id
	AttributeKeyScriptID = "script_id"
	// AttributeKeyCallData is the attribute for oracle request call data
	AttributeKeyCallData = "call_data"
	// AttributeKeyAskCount is the attribute for oracle request ask count
	AttributeKeyAskCount = "ask_count"
	// AttributeKeyMinCount is the attribute for oracle request min count
	AttributeKeyMinCount = "min_count"
	// AttributeKeyFeeLimit is the attribute for oracle request fee limit
	AttributeKeyFeeLimit = "fee_limit"
	// AttributeKeyPrepareGas is the attribute for oracle request prepare gas
	AttributeKeyPrepareGas = "prepare_gas"
	// AttributeKeyExecuteGas is the attribute for oracle request execute gas
	AttributeKeyExecuteGas = "execute_gas"
	// AttributeKeyRequestId is the attribute for oracle request id
	AttributeKeyRequestId = "request_id"
	// AttributeKeyAnsCount is the attribute for answer count
	AttributeKeyAnsCount = "ans_count"
	// AttributeKeyRequestTime is the attribute key for oracle request time
	AttributeKeyRequestTime = "request_time"
	// AttributeKeyResolveTime is the attribute key for oracle resolve time
	AttributeKeyResolveTime = "resolve_time"
	// AttributeKeyResolveStatus is the attribute key for oracle resolve status
	AttributeKeyResolveStatus = "resolve_status"
	// AttributeKeyResult is the attribute key for oracle result
	AttributeKeyResult = "result"
)
