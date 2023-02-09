package types

const (
	// SubModuleName defines the sub module name
	SubModuleName = "qbandchainoracle"

	// StoreKey defines the primary module store key
	StoreKey = SubModuleName

	// RouterKey is the message route for slashing
	RouterKey = SubModuleName

	// QuerierRoute defines the module's query routing key
	QuerierRoute = SubModuleName

	// PortID is the default port id that qoracle module binds to
	PortID = SubModuleName

	// OracleSource defines the source of oracle data
	OracleSource = "bandchain"

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
	// KeyCoinRatesState defines the key to store state of coin rates request
	KeyCoinRatesState = []byte("coin_rates_state")
	// KeyPriceList defines the key to store price list in store
	KeyPriceList = []byte("price_list")
)
