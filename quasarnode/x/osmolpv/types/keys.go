package types

import (
	"bytes"
	"strconv"

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

const (
	Sep            = ":" // Separater used in the keys
	NumDaysPerYear = 365
)

// var sepByte = []byte(":")

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
	LPPositionKBP            = []byte{0x05}
	RewardCollectionKBP      = []byte{0x06}
	LastRewardCollectionKBP  = []byte{0x07}
	EpochLPUserKBP           = []byte{0x08}
	LPUserInfoKBP            = []byte{0x09}
	LPEpochKBP               = []byte{0x10}
	EpochLPInfoKBP           = []byte{0x11}
	EpochDayInfoKBP          = []byte{0x12}
	LPCountKBP               = []byte{0x13}
	LPStatKBP                = []byte{0x14}
	LPEpochDenomKBP          = []byte{0x15}
	DayMapKBP                = []byte{0x16}
	ExitKBP                  = []byte{0x17}
)

func CreateUserReceiptCoinsKey(addr sdk.AccAddress) []byte {
	return addr.Bytes()
}

const (
	FeeDataKey = "FeeData-value-"
	LPCount    = "lpcount-"
)

func CreateLPCountKey() []byte {
	return []byte(LPCount)
}

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

// CreateOrionRewardGloablMaccName will create account name string for the global reward collector.
// return "Orion.reward.global"
func CreateOrionRewardGloablMaccName() string {
	var b bytes.Buffer
	b.WriteString(ModuleName)
	b.WriteString(".reward.global")
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
// Key format -  {0x03} + {“current_pos”} + ":" + {epochdays}
func CreateMeissaEpochPositionKey(epochday uint64) []byte {
	var b bytes.Buffer
	b.WriteString(CurrentPositionName)
	b.WriteString(Sep)
	b.Write(qbanktypes.CreateIDKey(epochday))
	return b.Bytes()
}

// CreateMeissaPoolPositionKey  create key for storing the epoch wise  deployed position on a pool ID
// Key format -  {0x04} + {epochdays} + ":" + {lockupPeriodStr} + ":" + "PoolID"
func CreateMeissaPoolPositionKey(epochday uint64, lockupPeriod qbanktypes.LockupTypes, poolID uint64) []byte {
	var b bytes.Buffer
	b.Write(qbanktypes.CreateIDKey(epochday))
	b.WriteString(Sep)
	b.WriteString(qbanktypes.LockupTypes_name[int32(lockupPeriod)])
	b.WriteString(Sep)
	b.Write(qbanktypes.CreateIDKey(poolID))
	return b.Bytes()
}

// LP Position Keys

const (
	LpPositionKey = "LpPosition-value-"
)

// CreateLPPositionEpochKey create key for the list of all lp position
// created on an epoch day. Ex. {LPPositionKBP} + {epochday}
func EpochDayKey(epochday uint64) []byte {
	var b bytes.Buffer
	strEpochday := strconv.FormatUint(epochday, 10)
	b.WriteString(strEpochday)
	return b.Bytes()
	// return qbanktypes.CreateIDKey(epochday)
}

func CreateLPIDKey(lpID uint64) []byte {
	var b bytes.Buffer
	strlpID := strconv.FormatUint(lpID, 10)
	b.WriteString(strlpID)
	return b.Bytes()
}

// EpochLPIDKey create key for the particular lp position created on an epoch day.
// Ex. Prefixed Key = {LPPositionKBP} + {epochday} + ":" + "lpID"
func EpochLPIDKey(epochday uint64, lpID uint64) []byte {
	var b bytes.Buffer
	strEpochday := strconv.FormatUint(epochday, 10)
	b.WriteString(strEpochday)
	b.WriteString(Sep)
	strlpID := strconv.FormatUint(lpID, 10)
	b.WriteString(strlpID)
	return b.Bytes()
}

// CreateEpochRewardKey create key for the storing reward collected on a specific epoch day
// Ex. Prefixed Key = {RewardCollectionKBP} + {epochday}
func CreateEpochRewardKey(epochday uint64) []byte {
	var b bytes.Buffer
	strEpochday := strconv.FormatUint(epochday, 10)
	b.WriteString(strEpochday)
	return b.Bytes()
	// return qbanktypes.CreateIDKey(epochday)
}

// CreateEpochLPUserKey create key for storing the user records on a specific lp position.
// Ex. Prefixed Key = = {EpochLPUserKBP} + {epochday} + {":"} + {lpID} + {":"} + {userAcc} + {":"} + {denom}
func CreateEpochLPUserKey(epochday uint64, lpID uint64, userAcc string, denom string) []byte {
	var b bytes.Buffer
	strEpochday := strconv.FormatUint(epochday, 10)
	b.WriteString(strEpochday)
	b.WriteString(Sep)
	strlpID := strconv.FormatUint(lpID, 10)
	b.WriteString(strlpID)
	b.WriteString(Sep)
	b.WriteString(userAcc)
	b.WriteString(Sep)
	b.WriteString(denom)
	return b.Bytes()
}

// CreateEpochLPUserKey create key for storing the user records on a specific lp position.
// Ex. Prefixed Key = = {LPUserInfoKBP} + {epochday} + {":"} + {lpID} + {":"} + {userAcc}
func CreateEpochLPUserInfo(epochday uint64, lpID uint64, userAcc string) []byte {
	var b bytes.Buffer
	strEpochday := strconv.FormatUint(epochday, 10)
	b.WriteString(strEpochday)
	b.WriteString(Sep)
	strlpID := strconv.FormatUint(lpID, 10)
	b.WriteString(strlpID)
	b.WriteString(Sep)
	b.WriteString(userAcc)
	return b.Bytes()
}

// CreateEpochLPUserKey create key for fetching the user records on a specific lp position.
// Ex. Prefixed Key = = {EpochLPUserKBP} + {epochday} + {":"} + {lpID} + {":"} +
func CreateEpochLPKey(epochday uint64, lpID uint64) []byte {
	var b bytes.Buffer
	strEpochday := strconv.FormatUint(epochday, 10)
	b.WriteString(strEpochday)
	b.WriteString(Sep)
	strlpID := strconv.FormatUint(lpID, 10)
	b.WriteString(strlpID)
	b.WriteString(Sep)
	return b.Bytes()
}

// ParseUserDenomKey splits the input key {userAcc} + {":"} + {denom} into uid and denom
func ParseUserDenomKey(key []byte) (uid, denom string) {
	split := qbanktypes.SplitKeyBytes(key)
	uid = string(split[0])
	denom = string(split[1])
	return
}

// CreateEpochLPUserKey create key for fetching the user records on a specific lp position.
// Ex. Prefixed Key = = {EpochLPUserKBP} +   {lpID} + {":"} + {epochday}
func CreateLPEpochKey(epochday uint64, lpID uint64) []byte {
	var b bytes.Buffer
	strlpID := strconv.FormatUint(lpID, 10)
	b.WriteString(strlpID)
	b.WriteString(Sep)
	strEpochday := strconv.FormatUint(epochday, 10)
	b.WriteString(strEpochday)
	return b.Bytes()
}

// CreateEpochDenomKey create key for fetching the epoch and denom pairs.
// Ex. Prefixed Key = = {LPEpochDenomKBP} +   {epochday} + {":"} + {denom}
func CreateEpochDenomKey(epochday uint64, denom string) []byte {
	var b bytes.Buffer
	strEpochday := strconv.FormatUint(epochday, 10)
	b.WriteString(strEpochday)
	b.WriteString(Sep)
	b.WriteString(denom)
	return b.Bytes()
}

// CreateDayMappingKey create the mapping key for expected reward day, actual deposit day and lockup
// Ex. Prefixed Key = = {DayMapKBP} +   {rewardday} + {":"} + {depositday} + {":"} + {lockupPeriod}
func CreateDayMappingKey(rewardday uint64,
	depositday uint64, lockupPeriod qbanktypes.LockupTypes) []byte {

	var b bytes.Buffer
	strRewardDay := strconv.FormatUint(rewardday, 10)
	b.WriteString(strRewardDay)
	b.WriteString(Sep)
	strDepositDay := strconv.FormatUint(depositday, 10)
	b.WriteString(strDepositDay)
	b.WriteString(Sep)
	lockupPeriodStr := qbanktypes.LockupTypes_name[int32(lockupPeriod)]
	b.WriteString(lockupPeriodStr)
	return b.Bytes()
}

const (
	EpochLPInfoKey = "EpochLPInfo-value-"
)

const (
	RewardCollectionKey = "RewardCollection-value-"
)

const (
	UserLPInfoKey = "UserLPInfo-value-"
)

const (
	LpStatKey = "LpStat-value-"
)
