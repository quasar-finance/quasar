package types

// IBC events
const (
	EventTypeTimeout             = "timeout"
	EventTypeIbcCreatePoolPacket = "ibcCreatePool_packet"
	EventTypeIbcJoinPoolPacket   = "ibcJoinPool_packet"
	EventTypeIbcExitPoolPacket   = "ibcExitPool_packet"
	EventTypeIbcWithdrawPacket       = "ibcWithdraw_packet"
// this line is used by starport scaffolding # ibc/packet/event

	AttributeKeyAckSuccess = "success"
	AttributeKeyAck        = "acknowledgement"
	AttributeKeyAckError   = "error"
)
