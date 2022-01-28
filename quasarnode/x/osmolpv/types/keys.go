package types

import (
	"bytes"

	"github.com/abag/quasarnode/x/qbank/types"
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
)

// store key use the byte as key
func createStoreKey(k string) []byte {
	return []byte(k)
}

func KeyPrefix(p string) []byte {
	return []byte(p)
}

var (
	UserReceiptCoinsKBP = []byte{0x01}
	StrategyKBP         = []byte{0x02}
)

func CreateUserReceiptCoinsKey(addr sdk.AccAddress) []byte {
	return addr.Bytes()
}

const (
	FeeDataKey = "FeeData-value-"
)

// @desc Function will create store key for the storage of list of base
// strategies in orion vault
// @return Key for prefix key store.
func CreateStrategyKey() []byte {
	return []byte(StrategyKey)
}

func CreateMeissaStrategyKey() []byte {
	return []byte(MeissaStrategyName)
}

func CreateRigelStrategyKey() []byte {
	return []byte(RigelStrategyName)
}

// @desc Function will create account name string for the staking.
// Calling function should take care of providing a valid input param.
// @return "Orion.stake.[LockupTypes string]"
func CreateOrionStakingMaccName(lockupPeriod qbanktypes.LockupTypes) string {
	var b bytes.Buffer
	b.WriteString(types.ModuleName)
	b.WriteString(".stake.")
	b.WriteString(qbanktypes.LockupTypes_name[int32(lockupPeriod)])
	return b.String()
}

// @desc Function will create account name string for the reward collector.
// Calling function should take care of providing a valid input param.
// @return "Orion.reward.[LockupTypes string]"
func CreateOrionRewardMaccName(lockupPeriod qbanktypes.LockupTypes) string {
	var b bytes.Buffer
	b.WriteString(types.ModuleName)
	b.WriteString(".reward.")
	b.WriteString(qbanktypes.LockupTypes_name[int32(lockupPeriod)])
	return b.String()
}

// @desc Function will create account name string for meissa strategy staking.
// Calling function should take care of providing a valid input param.
// @return "Orion.Meissa.stake.[LockupTypes string]"
func CreateMeissaStakingMaccName(lockupPeriod qbanktypes.LockupTypes) string {
	var b bytes.Buffer
	b.WriteString(types.ModuleName)
	b.WriteString(".meissa.stake.")
	b.WriteString(qbanktypes.LockupTypes_name[int32(lockupPeriod)])
	return b.String()
}

// @desc Function will create account name string for the reward collector.
// Calling function should take care of providing a valid input param.
// @return "Orion.meissa.reward.[LockupTypes string]"
func CreateMeissaRewardMaccName(lockupPeriod qbanktypes.LockupTypes) string {
	var b bytes.Buffer
	b.WriteString(types.ModuleName)
	b.WriteString(".meissa.reward.")
	b.WriteString(qbanktypes.LockupTypes_name[int32(lockupPeriod)])
	return b.String()
}
