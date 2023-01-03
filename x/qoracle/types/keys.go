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
	KeyMemInitialized = []byte("mem_initialized")
	// KeyMemDenomPricePrefix defines the prefix for the denom price key in the memory store
	KeyMemDenomPricePrefix = []byte("denom_price")
	// KeyMemDenomPricesUpdatedAt defines the key that stores the denom prices updated at in the memory store
	KeyMemDenomPricesUpdatedAt = []byte("denom_prices_updated_at")
	// KeySymbolPriceListUpdateFlag defines the key that stores the symbol price list update flag in the memory store
	KeySymbolPriceListUpdateFlag = []byte("symbol_prices_update_flag")
	// KeyPoolsUpdateFlag defines the key that stores the pools update flag in the memory store
	KeyPoolsUpdateFlag = []byte("pools_update_flag")
	// KeyPoolPrefix defines the prefix for the pool key in the memory store
	KeyMemPoolPrefix = []byte("pool")
	// KeyMemPoolUpdatedAt defines the prefix for the denom symbol mapping key in store
	KeyDenomSymbolMappingPrefix = []byte("denom_symbol_mapping")
)

// GetDenomPriceKey returns the key for the denom price in the memory store.
func GetDenomPriceKey(denom string) []byte {
	return append(KeyMemDenomPricePrefix, []byte(denom)...)
}

// GetPoolKey returns the key for the pool in the memory store.
func GetPoolKey(source string, id string) []byte {
	return append(KeyMemPoolPrefix, []byte(source+"/"+id)...)
}

// GetDenomSymbolMappingKey returns the key for the denom symbol mapping in store.
func GetDenomSymbolMappingKey(denom string) []byte {
	return append(KeyDenomSymbolMappingPrefix, []byte(denom)...)
}
