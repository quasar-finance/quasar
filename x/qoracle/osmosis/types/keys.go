package types

const (
	// SubModuleName defines the sub module name
	SubModuleName = "qoracle_osmosis"

	// StoreKey defines the primary module store key
	StoreKey = SubModuleName

	// RouterKey is the message route for slashing
	RouterKey = SubModuleName

	// QuerierRoute defines the module's query routing key
	QuerierRoute = SubModuleName

	// PortID is the default port id that qoracle module binds to
	PortID = SubModuleName

	// MemStoreKey defines the in-memory store key
	MemStoreKey = "mem_qoracle_osmosis"
)

var (
	// PortKey defines the key to store the port ID in store
	PortKey = []byte{0x01}
	// ParamsRequestStateKey defines the key to store state of chain params request
	ParamsRequestStateKey = []byte("params_request_state")
	// IncentivizedPoolsRequestStateKey defines the key to store state of osmosis incentivized pools request
	IncentivizedPoolsRequestStateKey = []byte("incentivized_pools_request_state")
	// PoolsRequestStateKey defines the key to store state of osmosis pools request
	PoolsRequestStateKey = []byte("pools_request_state")
	// KeyOsmosisPrefix is the prefix store key of osmosis store
	KeyOsmosisPrefix = []byte("osmosis")
)

var (
	KeyEpochsInfoPrefix = []byte("epochs_info")
	// KeyPoolPrefix defines the prefix key to store osmosis pools in store
	KeyPoolPrefix          = []byte("pools")
	KeyLockableDurations   = []byte("lockable_durations")
	KeyMintParams          = []byte("mint_params")
	KeyMintEpochProvisions = []byte("mint_epoch_provisions")
	KeyIncentivizedPools   = []byte("incentivized_pools")
	KeyPoolGaugeIdsPrefix  = []byte("pool_gauge_ids")
	KeyDistrInfo           = []byte("distr_info")
)
