package types

import (
	"bytes"

	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

const (
	// ModuleName defines the module name
	ModuleName = "osmolpv"

	// StoreKey defines the primary module store key
	StoreKey = ModuleName

	// RouterKey is the message route for slashing
	RouterKey = ModuleName

	// QuerierRoute defines the module's query routing key
	QuerierRoute = ModuleName

	// MemStoreKey defines the in-memory store key
	MemStoreKey = "mem_osmolpv"

	//OsmoLPV vault reserve module account name
	OsmoLPVReserveMaccName = "osmolpv_rsv"

	// Management fee collector module account name
	MgmtFeeCollectorMaccName = "orion_mgmtfee_cltr"

	// Performance fee collector module account name
	PerfFeeCollectorMaccName = "orion_perffee_cltr"

	// Entry fee collector module account name
	EntryFeeCollectorMaccName = "orion_entryfee_cltr"

	// Exit fee collector module account name
	ExitFeeCollectorMaccName = "orion_exitfee_cltr"

	// Strategy Key
	StrategyKey = "orion_strategies"
	// Meissa strategy name
	MeissaStrategyName = "meissa"

	// Rigel strategy name
	RigelStrategyName = "rigel"

	// Current Position constant
	CurrentPositionName = "current_position"
)

// store key use the byte as key
func createStoreKey(k string) []byte {
	return []byte(k)
}

func KeyPrefix(p string) []byte {
	return []byte(p)
}

var (
	UserReceiptCoinsKBP      = []byte{0x01}
	StrategyKBP              = []byte{0x02}
	MeissaStrategyKBP        = []byte{0x03}
	MeissaStrategyPoolPosKBP = []byte{0x04}
)

func CreateUserReceiptCoinsKey(addr sdk.AccAddress) []byte {
	return addr.Bytes()
}

const (
	FeeDataKey = "FeeData-value-"
)

// CreateStrategyKey will create store key for the storage of base strategies in orion vault
// return Key for prefix key store.
func CreateStrategyKey() []byte {
	return []byte(StrategyKey)
}

// CreateMeissaStrategyKey will create store key for the storage of strategy names
// return Key for prefix key store.
func CreateMeissaStrategyKey() []byte {
	return []byte(MeissaStrategyName)
}

// CreateRigelStrategyKey will create store key for the storage of strategy names
// return Key for prefix key store.
func CreateRigelStrategyKey() []byte {
	return []byte(RigelStrategyName)
}

// CreateOrionStakingMaccName will create account name string for the staking.
// return "Orion.stake.[LockupTypes string]"
func CreateOrionStakingMaccName(lockupPeriod qbanktypes.LockupTypes) string {
	var b bytes.Buffer
	b.WriteString(ModuleName)
	b.WriteString(".stake.")
	// b.WriteString("stake")
	b.WriteString(qbanktypes.LockupTypes_name[int32(lockupPeriod)])
	return b.String()
}

// CreateOrionRewardMaccName will create account name string for the reward collector.
// return "Orion.reward.[LockupTypes string]"
func CreateOrionRewardMaccName(lockupPeriod qbanktypes.LockupTypes) string {
	var b bytes.Buffer
	b.WriteString(ModuleName)
	b.WriteString(".reward.")
	b.WriteString(qbanktypes.LockupTypes_name[int32(lockupPeriod)])
	return b.String()
}

// CreateMeissaStakingMaccName will create account name string for meissa strategy staking.
// return "Orion.Meissa.stake.[LockupTypes string]"
func CreateMeissaStakingMaccName(lockupPeriod qbanktypes.LockupTypes) string {
	var b bytes.Buffer
	b.WriteString(ModuleName)
	b.WriteString(".meissa.stake.")
	b.WriteString(qbanktypes.LockupTypes_name[int32(lockupPeriod)])
	return b.String()
}

// CreateMeissaMaccName will create account name string for meissa strategy for its global generic usage.
// return "Orion.Meissa.global"
func CreateMeissaMaccName() string {
	var b bytes.Buffer
	b.WriteString(ModuleName)
	b.WriteString(".meissa.global")
	return b.String()
}

// CreateMeissaRewardMaccName will create account name string for the reward collector.
// return "Orion.meissa.reward.[LockupTypes string]"
func CreateMeissaRewardMaccName(lockupPeriod qbanktypes.LockupTypes) string {
	var b bytes.Buffer
	b.WriteString(ModuleName)
	b.WriteString(".meissa.reward.")
	b.WriteString(qbanktypes.LockupTypes_name[int32(lockupPeriod)])
	return b.String()
}

// CreateMeissaPositionKey will create key for storing the current position of the strategy.
// Key format -  {0x03} + {“current_pos”}
func CreateMeissaPositionKey() []byte {
	return []byte(CurrentPositionName)
}

// @desc Function will create key for storing the epoch wise current position of the strategy
// Key format -  {0x03} + {“current_pos”} + "/" + {epochdays}
func CreateMeissaEpochPositionKey(epochday uint64) []byte {
	var b bytes.Buffer
	b.WriteString(CurrentPositionName)
	b.WriteString("/")
	b.Write(qbanktypes.CreateIDKey(epochday))
	return b.Bytes()
}

// CreateMeissaPoolPositionKey  create key for storing the epoch wise  deployed position on a pool ID
// Key format -  {0x04} + {epochdays} + "/" + {lockupPeriodStr} + "/" + "PoolID"
func CreateMeissaPoolPositionKey(epochday uint64, lockupPeriod qbanktypes.LockupTypes, poolID uint64) []byte {
	var b bytes.Buffer
	b.Write(qbanktypes.CreateIDKey(epochday))
	b.WriteString("/")
	b.WriteString(qbanktypes.LockupTypes_name[int32(lockupPeriod)])
	b.WriteString("/")
	b.Write(qbanktypes.CreateIDKey(poolID))
	return b.Bytes()
}
