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

	// MemStoreKey defines the in-memory store key
	MemStoreKey = "mem_qoracle"

	// TransientStoreKey defines the transient store key
	TStoreKey = "transient_qoracle"
)

var (
	// KeyMemInitialized defines the key that stores the initialized flag in the memory store
	KeyMemInitialized = []byte{0x01}
	// KeyMemDenomPricePrefix defines the prefix for the denom price key in the memory store
	KeyMemDenomPricePrefix = []byte{0x01}
	// KeyMemDenomPricesUpdatedAt defines the key that stores the denom prices updated at in the memory store
	KeyMemDenomPricesUpdatedAt = []byte{0x02}
	// KeySymbolPriceListUpdateFlag defines the key that stores the symbol price list update flag in the memory store
	KeySymbolPriceListUpdateFlag = []byte{0x03}
	// KeyPoolsUpdateFlag defines the key that stores the pools update flag in the memory store
	KeyPoolsUpdateFlag = []byte{0x04}
	// KeyMemPoolPrefix defines the prefix for the pool key in the memory store
	KeyMemPoolPrefix = []byte{0x05}
	// KeyDenomSymbolMappingPrefix defines the prefix for the denom symbol mapping key in store
	KeyDenomSymbolMappingPrefix = []byte{0x06}
	// KeyOsmosisPoolPrefix defines the prefix osmosis pools stored in the memory store
	KeyOsmosisPoolPrefix = []byte{0x07}
)

// GetDenomPriceKey returns the key for the denom price in the memory store.
func GetDenomPriceKey(denom string) []byte {
	return append(KeyMemDenomPricePrefix, []byte(denom)...)
}

// GetDenomSymbolMappingKey returns the key for the denom symbol mapping in store.
func GetDenomSymbolMappingKey(denom string) []byte {
	return append(KeyDenomSymbolMappingPrefix, []byte(denom)...)
}
