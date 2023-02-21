package types

const (
	// SubModuleName defines the sub module name
	SubModuleName = "qosmosisoracle"

	// StoreKey defines the primary module store key
	StoreKey = SubModuleName

	// RouterKey is the message route for slashing
	RouterKey = SubModuleName

	// QuerierRoute defines the module's query routing key
	QuerierRoute = SubModuleName

	// PortID is the default port id that qoracle module binds to
	PortID = SubModuleName

	// OracleSource defines the source of oracle data
	OracleSource = "osmosis"
)

var (
	// PortKey defines the key to store the port ID in store
	PortKey = []byte{0x01}
	// KeyParamsRequestState defines the key to store state of chain params request
	KeyParamsRequestState = []byte{0x02}
	// KeyIncentivizedPoolsRequestState defines the key to store state of osmosis incentivized pools request
	KeyIncentivizedPoolsRequestState = []byte{0x03}
	// KeyPoolsRequestState defines the key to store state of osmosis pools request
	KeyPoolsRequestState = []byte{0x04}
	// KeyEpochsInfo defines the key to store osmosis epochs info in store
	KeyEpochsInfo = []byte{0x05}
	// KeyPoolPrefix defines the prefix key to store osmosis pools in store
	KeyPoolPrefix = []byte{0x06}
	// KeyPoolsUpdatedAt defines the key to store osmosis pools updated at in store
	KeyPoolsUpdatedAt = []byte{0x07}
	// KeyLockableDurations defines the key to store lockable durations in store
	KeyLockableDurations = []byte{0x08}
	// KeyMintParams defines the key to store mint params in store
	KeyMintParams = []byte{0x09}
	// KeyMintEpochProvisions defines the key to store mint epoch provisions in store
	KeyMintEpochProvisions = []byte{0x0A}
	// KeyIncentivizedPools defines the key to store incentivized pools in store
	KeyIncentivizedPools = []byte{0x0B}
	// KeyDistrInfo defines the key to store distribution info in store
	KeyDistrInfo = []byte{0x0C}
)
