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
	KeyParamsRequestState = []byte("params_request_state")
	// KeyIncentivizedPoolsRequestState defines the key to store state of osmosis incentivized pools request
	KeyIncentivizedPoolsRequestState = []byte("incentivized_pools_request_state")
	// KeyPoolsRequestState defines the key to store state of osmosis pools request
	KeyPoolsRequestState = []byte("pools_request_state")
	// KeyEpochsInfo defines the key to store osmosis epochs info in store
	KeyEpochsInfo = []byte("epochs_info")
	// KeyPoolPrefix defines the prefix key to store osmosis pools in store
	KeyPoolPrefix = []byte("pool")
	// KeyPoolsUpdatedAt defines the key to store osmosis pools updated at in store
	KeyPoolsUpdatedAt = []byte("pools_updated_at")
	// KeyKeyLockableDurations defines the key to store lockable durations in store
	KeyLockableDurations = []byte("lockable_durations")
	// KeyMintParams defines the key to store mint params in store
	KeyMintParams = []byte("mint_params")
	// KeyMintEpochProvisions defines the key to store mint epoch provisions in store
	KeyMintEpochProvisions = []byte("mint_epoch_provisions")
	// KeyIncentivizedPools defines the key to store incentivized pools in store
	KeyIncentivizedPools = []byte("incentivized_pools")
	// KeyDistrInfo defines the key to store distribution info in store
	KeyDistrInfo = []byte("distr_info")
)
