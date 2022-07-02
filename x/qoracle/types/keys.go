package types

const (
	// ModuleName defines the module name
	ModuleName = "qoracle"

	// StoreKey defines the primary module store key
	StoreKey = ModuleName

	// RouterKey is the message route for slashing
	RouterKey = ModuleName

	// QuerierRoute defines the module's query routing key
	QuerierRoute = ModuleName

	// PortID is the default port id that qoracle module binds to
	PortID = ModuleName

	// MemStoreKey defines the in-memory store key
	MemStoreKey = "mem_qoracle"

	// CoinRatesClientID is query request identifier
	CoinRatesClientID = "coin_rates_id"

	// KeyCoinRatesState is the key for the state

	// BandchainOraclePortID is the default port id that bandchain oracle module binds to
	BandchainOraclePortID = "oracle"

	// BandchainOracleVersion is the default version of bandchain oracle
	BandchainOracleVersion = "bandchain-1"
)

func KeyPrefix(p string) []byte {
	return []byte(p)
}

const (
	Sep = "#" // Separater used in the keys
)

var (
	PoolPositionKBP       = []byte{0x01}
	PoolInfoKBP           = []byte{0x02}
	PoolAPYRankedKBP      = []byte{0x03}
	PoolSpotPriceKBP      = []byte{0x05}
	PortKey               = []byte{0x06}
	KeyCoinRatesState     = []byte("coin_rates_state")
	KeyStablePricesPrefix = []byte("stable_prices")
)

var SepByte = []byte("#")

func CreatePoolPositionKey(poolID string) []byte {
	return []byte(poolID)
}

func CreatePoolInfoKey(poolID string) []byte {
	return []byte(poolID)
}

// PoolSpotPriceKey returns the store key to retrieve a PoolSpotPrice from the index fields
func CreatePoolSpotPriceKey(poolId string, denomIn string, denomOut string) []byte {
	var key []byte

	poolIdBytes := []byte(poolId)
	key = append(key, poolIdBytes...)
	key = append(key, SepByte...)

	denomInBytes := []byte(denomIn)
	key = append(key, denomInBytes...)
	key = append(key, SepByte...)

	denomOutBytes := []byte(denomOut)
	key = append(key, denomOutBytes...)
	// key = append(key, SepByte...)

	return key
}
