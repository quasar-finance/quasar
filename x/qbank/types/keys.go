package types

import (
	"bytes"
	"encoding/binary"
	"strconv"
)

const (
	// ModuleName defines the module name
	ModuleName = "qbank"

	// StoreKey defines the primary module store key
	StoreKey = ModuleName

	// RouterKey is the message route for slashing
	RouterKey = ModuleName

	// QuerierRoute defines the module's query routing key
	QuerierRoute = ModuleName

	// MemStoreKey defines the in-memory store key
	MemStoreKey = "mem_qbank"
)

func KeyPrefix(p string) []byte {
	return []byte(p)
}

const (
	Sep = ":" // Separator used in the keys
)

var (
	// KBP - short of KeyBytePrefix, Byte prefix for the key used in KV store
	UserDenomDepositKBP            = []byte{0x01}
	EpochLockupUserDenomDepositKBP = []byte{0x02}
	UserDepositKBP                 = []byte{0x03}
	WithdrawableKeyKBP             = []byte{0x04}
	ActualWithdrawableKeyKBP       = []byte{0x05}
	TotalWithdrawKeyKBP            = []byte{0x06}
	UserClaimKBP                   = []byte{0x07}
	UserClaimedKBP                 = []byte{0x08}
)

var SepByte = []byte(":")

func SplitKeyBytes(kb []byte) [][]byte {
	return bytes.Split(kb, SepByte)
}

// store key use the byte as key
func createStoreKey(k string) []byte {
	return []byte(k)
}

// CreateIDKey create the prefix store key for specific deposit or withdraw object id
// Input param - deposit id or withdraw id
func CreateIDKey(id uint64) []byte {
	bz := make([]byte, 8)
	binary.BigEndian.PutUint64(bz, id)
	return bz
}

// CreateIDFromByteKey create the deposit or withdraw id of uint type from input byte
func CreateIDFromByteKey(bzKey []byte) uint64 {
	return binary.BigEndian.Uint64(bzKey)
}

// Deposit specific function

// CreateUserDenomDepositKey create the prefix store key for the user denom wise deposit storage
func CreateUserDenomDepositKey(uid, sep, denom string) []byte {
	var b bytes.Buffer
	b.WriteString(uid)
	b.WriteString(sep)
	b.WriteString(denom)
	return b.Bytes()
}

// CreateUserDenomLockupDepositKey create the prefix store key for the user denom wise deposit storage
// with lockup periods.
// Ex. {uid} + ":" + {denom} + ":" + "{lockupString}"
func CreateUserDenomLockupDepositKey(uid, sep, denom string, lockupPeriod LockupTypes) []byte {
	var b bytes.Buffer
	b.WriteString(uid)
	b.WriteString(sep)
	b.WriteString(denom)
	b.WriteString(sep)
	lockupPeriodStr := LockupTypes_name[int32(lockupPeriod)]
	b.WriteString(lockupPeriodStr)
	return b.Bytes()
}

// CreateUserDenomEpochLockupDepositKey create the prefix store key for the user denom epoch lockup wise deposit storage
// Ex. {uid} + ":" + {denom} + ":" + {epochDay} + ":" + {lockupString}
func CreateUserDenomEpochLockupDepositKey(uid, sep, denom string, epochDay uint64, lockupPeriod LockupTypes) []byte {
	var b bytes.Buffer
	b.WriteString(uid)
	b.WriteString(sep)
	b.WriteString(denom)
	b.WriteString(sep)
	strEpochDay := strconv.FormatUint(epochDay, 10)
	b.WriteString(strEpochDay)
	b.WriteString(sep)
	lockupPeriodStr := LockupTypes_name[int32(lockupPeriod)]
	b.WriteString(lockupPeriodStr)
	return b.Bytes()
}

// CreateEpochLockupUserDenomDepositKey create the prefix store key for the epochDay lockup wise user denom wise deposit storage
// Ex.  {epochDay} + ":" + {lockupString} + ":" + {uid} + ":" + {denom}
func CreateEpochLockupUserDenomDepositKey(uid, sep, denom string, epochDay uint64, lockupPeriod LockupTypes) []byte {
	var b bytes.Buffer
	strEpochDay := strconv.FormatUint(epochDay, 10)
	b.WriteString(strEpochDay)
	b.WriteString(sep)
	lockupPeriodStr := LockupTypes_name[int32(lockupPeriod)]
	b.WriteString(lockupPeriodStr)
	b.WriteString(sep)
	b.WriteString(uid)
	b.WriteString(sep)
	b.WriteString(denom)

	return b.Bytes()
}

// CreateEpochLockupKey create the prefix store key for the epochDay lockup wise deposit.
// This key is used for the prefix key iteration to get the deposits done on a given epochDay
// Ex.  {epochDay} + ":" + "lockupString" + ":"
func CreateEpochLockupUserKey(epochDay uint64, lockupPeriod LockupTypes, sep string) []byte {
	var b bytes.Buffer
	strEpochDay := strconv.FormatUint(epochDay, 10)
	b.WriteString(strEpochDay)
	b.WriteString(sep)
	lockupPeriodStr := LockupTypes_name[int32(lockupPeriod)]
	b.WriteString(lockupPeriodStr)
	b.WriteString(sep)
	return b.Bytes()
}

// CreateEpochLockupUserSepKey  create the prefix store key for the iteration.
// This key is used for the prefix key iteration to get the deposits done on a given epochDay
// Ex.  {epochDay} + ":" + "lockupString" + ":" + {uid} + ":"
func CreateEpochLockupUserSepKey(epochDay uint64, lockupPeriodStr, uid, sep string) []byte {
	var b bytes.Buffer
	strEpochDay := strconv.FormatUint(epochDay, 10)
	b.WriteString(strEpochDay)
	b.WriteString(sep)
	b.WriteString(lockupPeriodStr)
	b.WriteString(sep)
	b.WriteString(uid)
	b.WriteString(sep)
	return b.Bytes()
}

// create the prefix store key for the user account
func CreateUserDepositKey(uid string) []byte {
	return createStoreKey(uid)
}

// CreateWithdrawableKey create key for the withdrawable KV store to fetch current
// withdrawable amount by a given user of given denom.
// Key = {uid} + ":" + {denom}
func CreateWithdrawableKey(uid, denom, sep string) []byte {
	var b bytes.Buffer
	b.WriteString(uid)
	b.WriteString(sep)
	b.WriteString(denom)
	return b.Bytes()
}

// CreateWithdrawableKey create key for the total withdraw KV store to fetch current
// total value of coins that users have successfully withdraw
// Key = {uid} + ":" + {vault}
func CreateTotalWithdrawKey(uid, vault, sep string) []byte {
	var b bytes.Buffer
	b.WriteString(uid)
	b.WriteString(sep)
	b.WriteString(vault)
	return b.Bytes()
}

// CreateWithdrawableKey create key for the lockup period based withdrawable KV store to fetch current
// withdrawable amount by a given user, denom and lockup period
// Key = {denom} + ":" + {uid} + ":" + {lockupPeriod}
func CreateLockupWithdrawableKey(denom, uid string, lockupPeriod LockupTypes, sep string) []byte {
	var b bytes.Buffer
	b.WriteString(denom)
	b.WriteString(sep)
	b.WriteString(uid)
	b.WriteString(sep)
	lockupPeriodStr := LockupTypes_name[int32(lockupPeriod)]
	b.WriteString(lockupPeriodStr)

	return b.Bytes()
}

// Withdraw specific functions

// CreateUsersClaimKey create keys for users claim/claimed amount
// Key = {uid} + ":" + {vaultID}
func CreateUsersClaimKey(uid, vaultID, sep string) []byte {
	var b bytes.Buffer
	b.WriteString(uid)
	b.WriteString(sep)
	b.WriteString(vaultID)
	return b.Bytes()
}

// Note : Not used now
const (
	FeeDataKey = "FeeData-value-"
)

// ParseEpochLockupUserDenomDepositKey split the composite key of type " {epochDay} + ":" + {lockupString} + ":" + {uid} + ":" + {denom}"
// and split into field variable and return accordingly
func ParseEpochLockupUserDenomDepositKey(key []byte) (epochDay uint64, lockupStr, uid, denom string, err error) {
	split := SplitKeyBytes(key)
	epochDayStr := string(split[0])
	epochDay, err = strconv.ParseUint(epochDayStr, 10, 64)
	lockupStr = string(split[1])
	uid = string(split[2])
	denom = string(split[3])
	return
}

// EpochDayKey create key  for the epochDay
func EpochDaySepKey(epochDay uint64, sep string) []byte {
	var b bytes.Buffer
	strEpochDay := strconv.FormatUint(epochDay, 10)
	b.WriteString(strEpochDay)
	b.WriteString(sep)
	return b.Bytes()
}
