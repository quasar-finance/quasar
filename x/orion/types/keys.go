package types

import (
	"bytes"
	"strconv"

	sdk "github.com/cosmos/cosmos-sdk/types"
	qbanktypes "github.com/quasarlabs/quasarnode/x/qbank/types"
)

const (

	// ModuleName defines the module name
	ModuleName = "orion"

	// StoreKey defines the primary module store key
	StoreKey = ModuleName

	// RouterKey is the message route for slashing
	RouterKey = ModuleName

	// QuerierRoute defines the module's query routing key
	QuerierRoute = ModuleName

	// MemStoreKey defines the in-memory store key
	MemStoreKey = "mem_orion"

	//Orion vault reserve module account name
	OrionReserveMaccName = "orion_rsv"

	// Management fee collector module account name
	MgmtFeeCollectorMaccName = "orion_mgmtfee_cltr"

	// Performance fee collector module account name
	PerfFeeCollectorMaccName = "orion_perffee_cltr"

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

func KeyPrefix(p string) []byte {
	return []byte(p)
}

var (
	UserReceiptCoinsKBP            = []byte{0x01}
	StrategyKBP                    = []byte{0x02}
	MeissaStrategyKBP              = []byte{0x03}
	MeissaStrategyPoolPosKBP       = []byte{0x04}
	LPPositionKBP                  = []byte{0x05}
	RewardCollectionKBP            = []byte{0x06}
	LastRewardCollectionKBP        = []byte{0x07}
	EpochLPUserKBP                 = []byte{0x08}
	LPUserInfoKBP                  = []byte{0x09}
	LPEpochKBP                     = []byte{0x10}
	EpochLPInfoKBP                 = []byte{0x11}
	EpochDayInfoKBP                = []byte{0x12}
	LPCountKBP                     = []byte{0x13}
	LPStatKBP                      = []byte{0x14}
	LPEpochDenomKBP                = []byte{0x15}
	DayMapKBP                      = []byte{0x16}
	ExitKBP                        = []byte{0x17}
	JoinPoolKBP                    = []byte{0x18}
	IBCTokenTransferKBP            = []byte{0x19}
	AvailableInterchainFundKBP     = []byte{0x20}
	IBCTokenTransferredKBP         = []byte{0x21}
	IBCTokenTransferSentKBP        = []byte{0x22}
	LockTokensKBP                  = []byte{0x23}
	SeqLockTokensKBP               = []byte{0x24}
	SeqTokenWithdrawFromOsmosisKBP = []byte{0x25}
)

func CreateUserReceiptCoinsKey(addr sdk.AccAddress) []byte {
	return addr.Bytes()
}

const (
	FeeDataKey     = "FeeData-value-"
	LPCount        = "lpcount-"
	InterchainFund = "interchainfund-"
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

// CreateMeissaPositionKey create key for storing the current position of the strategy.
// Key format -  types.MeissaStrategyKBP + {“current_pos”}
func CreateMeissaPositionKey() []byte {
	return []byte(CurrentPositionName)
}

// CreateMeissaEpochPositionKey create key for storing the epoch wise current position of the strategy
// Key format -  types.MeissaStrategyKBP + {“current_pos”} + ":" + {epochdays}
func CreateMeissaEpochPositionKey(epochday uint64) []byte {
	var b bytes.Buffer
	b.WriteString(CurrentPositionName)
	b.WriteString(Sep)
	strEpochday := strconv.FormatUint(epochday, 10)
	b.WriteString(strEpochday)
	return b.Bytes()
}

// AUDIT NOTE - Probably a redundant method
// CreateMeissaPoolPositionKey  create key for storing the epoch wise  deployed position on a pool ID
// Key format -  types.MeissaStrategyPoolPosKBP + {epochdays} + ":" + {lockupPeriodStr} + ":" + "PoolID"
func CreateMeissaPoolPositionKey(epochday uint64, lockupPeriod qbanktypes.LockupTypes, poolID uint64) []byte {
	var b bytes.Buffer
	strEpochday := strconv.FormatUint(epochday, 10)
	b.WriteString(strEpochday)
	b.WriteString(Sep)
	b.WriteString(qbanktypes.LockupTypes_name[int32(lockupPeriod)])
	b.WriteString(Sep)
	strPoolID := strconv.FormatUint(poolID, 10)
	b.WriteString(strPoolID)
	return b.Bytes()
}

// EpochDayKey create key for the list of all lp position
// created on an epoch day. Ex. {LPPositionKBP} + {epochday}
func EpochDayKey(epochday uint64) []byte {
	var b bytes.Buffer
	strEpochday := strconv.FormatUint(epochday, 10)
	b.WriteString(strEpochday)
	return b.Bytes()
}

//CreateSeqKey creates key for the sequence number to Lp mapping
func CreateSeqKey(seqNo uint64) []byte {
	var b bytes.Buffer
	strSeqNo := strconv.FormatUint(seqNo, 10)
	b.WriteString(strSeqNo)
	return b.Bytes()
}

// TODO - {seqNo} + {:} + {lockupPeriod} And also {lockupPeriod} + {:} + {seqNo}
func CreateInterchainFundKey() []byte {
	return []byte(InterchainFund)
}

// CreateLPIDKey create key for the LP ID
func CreateLPIDKey(lpID uint64) []byte {
	var b bytes.Buffer
	strlpID := strconv.FormatUint(lpID, 10)
	b.WriteString(strlpID)
	return b.Bytes()
}

// EpochLPIDKey create key for the particular lp position created on an epoch day.
// Ex. Prefixed Key = {LPPositionKBP} + {epochday} + ":" + {lpID}
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
}

// AUDIT NOTE - Probably a redundant method
// CreateEpochLPUserKey create key for storing the user records on a specific lp position.
// Ex. Prefixed Key = {EpochLPUserKBP} + {epochday} + {":"} + {lpID} + {":"} + {userAcc} + {":"} + {denom}
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

// AUDIT NOTE - Probably a redundant method
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

// AUDIT NOTE - Probably a redundant method
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

// CreateLPEpochKey create key for keeping the record of lpID and epoch pair
// Ex. Prefixed Key = = {LPEpochKBP} +   {lpID} + {":"} + {epochday}
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
// Ex. Prefixed Key = = {types.LPEpochDenomKBP/types.ExitKBP} +   {epochday} + {":"} + {denom}
func CreateEpochDenomKey(epochday uint64, denom string) []byte {
	var b bytes.Buffer
	strEpochday := strconv.FormatUint(epochday, 10)
	b.WriteString(strEpochday)
	b.WriteString(Sep)
	b.WriteString(denom)
	return b.Bytes()
}

// CreateDayMappingKey create the mapping key for expected reward day, actual deposit day and lockup
// Ex. Prefixed Key = = {DayMapKBP} +   {rewardday/exitday} + {":"} + {depositday} + {":"} + {lockupPeriod}
func CreateDayMappingKey(exitday uint64,
	depositday uint64, lockupPeriod qbanktypes.LockupTypes) []byte {

	var b bytes.Buffer
	strExitDay := strconv.FormatUint(exitday, 10)
	b.WriteString(strExitDay)
	b.WriteString(Sep)
	strDepositDay := strconv.FormatUint(depositday, 10)
	b.WriteString(strDepositDay)
	b.WriteString(Sep)
	lockupPeriodStr := qbanktypes.LockupTypes_name[int32(lockupPeriod)]
	b.WriteString(lockupPeriodStr)
	return b.Bytes()
}

func CreateEpochLockupKey(epochDay uint64,
	lockupPeriod qbanktypes.LockupTypes) []byte {
	var b bytes.Buffer
	strEpochDay := strconv.FormatUint(epochDay, 10)
	b.WriteString(strEpochDay)
	b.WriteString(Sep)
	lockupPeriodStr := qbanktypes.LockupTypes_name[int32(lockupPeriod)]
	b.WriteString(lockupPeriodStr)
	return b.Bytes()
}
