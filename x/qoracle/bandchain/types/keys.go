package types

const (
	// SubModuleName defines the sub module name
	SubModuleName = "qoracle_bandchain"

	// StoreKey defines the primary module store key
	StoreKey = SubModuleName

	// RouterKey is the message route for slashing
	RouterKey = SubModuleName

	// QuerierRoute defines the module's query routing key
	QuerierRoute = SubModuleName

	// PortID is the default port id that qoracle module binds to
	PortID = SubModuleName

	// MemStoreKey defines the in-memory store key
	MemStoreKey = "mem_qoracle_bandchain"

	// CoinRatesClientID is query request identifier
	CoinRatesClientID = "coin_rates_id"

	// BandchainOraclePortID is the default port id that bandchain oracle module binds to
	BandchainOraclePortID = "oracle"

	// BandchainOracleVersion is the default version of bandchain oracle
	BandchainOracleVersion = "bandchain-1"
)

var (
	// PortKey defines the key to store the port ID in store
	PortKey = []byte{0x01}
	// CoinRatesStateKey defines the key to store state of coin rates request
	CoinRatesStateKey = []byte("coin_rates_state")
	// PriceListKey defines the key to store price list in store
	PriceListKey = []byte("price_list")
)
