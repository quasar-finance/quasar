package types

const (
	// ModuleName defines the module name
	ModuleName = "qbank"

	// StoreKey defines the primary module store key
	StoreKey = ModuleName

	// RouterKey is the message route for slashing
	RouterKey = ModuleName

	// QuerierRoute defines the module's query routing key
	QuerierRoute = ModuleName

	// MemStoreKey defines the in-memory store key
	MemStoreKey = "mem_qbank"
)

func KeyPrefix(p string) []byte {
	return []byte(p)
}

const (

	// TODO - Use Prefix byte as 0x01, 0x02

	DepositKey                = "Deposit-value-"
	DepositCountKey           = "Deposit-count-"
	UserDenomDepositKeyPrefix = "User-denom-deposit-"
)
