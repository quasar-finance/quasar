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
	Sep = ":" // Separater used in the keys

	// Prefix keys
	DepositKey                = "Deposit-value-"
	DepositCountKey           = "Deposit-count-"
	UserDenomDepositKeyPrefix = "User-denom-deposit-"
	WithdrawKey               = "Withdraw-value-"
	WithdrawCountKey          = "Withdraw-count-"
)

var (
	// KBP - short of KeyBytePrefix, Byte prfix for the key used in KV store
	QbankGlobalKBP           = []byte{0x00} // Used for counts of deposit and withdraw
	DepositKBP               = []byte{0x01}
	UserDenomDepositKBP      = []byte{0x02}
	WithdrawKeyKBP           = []byte{0x03}
	UserDepositKBP           = []byte{0x04}
	WithdrawableKeyKBP       = []byte{0x05}
	UserClaimKBP             = []byte{0x06}
	ActualWithdrawableKeyKBP = []byte{0x07}

	// TODO Vault level prefix to be used.
)

var SepByte = []byte(":")

// Common functions for deposit and withdraw
// TODO - AUDIT | unit test case to be written
func SplitKeyBytes(kb []byte) [][]byte {
	// First byte is used for the byte prefix
	split := bytes.Split(kb[1:], SepByte)
	return split
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

// CreateDepositCountKey create the prefix store key for deposit counts
func CreateDepositCountKey() []byte {
	return createStoreKey(DepositCountKey)
}

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
// Ex. {uid} + ":" + {denom} + ":" + {epochday} + ":" + {lockupString}
func CreateUserDenomEpochLockupDepositKey(uid, sep, denom string, epochday uint64, lockupPeriod LockupTypes) []byte {
	var b bytes.Buffer
	b.WriteString(uid)
	b.WriteString(sep)
	b.WriteString(denom)
	b.WriteString(sep)
	strEpochday := strconv.FormatUint(epochday, 10)
	b.WriteString(strEpochday)
	b.WriteString(sep)
	lockupPeriodStr := LockupTypes_name[int32(lockupPeriod)]
	b.WriteString(lockupPeriodStr)
	return b.Bytes()
}

// CreateEpochLockupUserDenomDepositKey create the prefix store key for the epochday lockup wise user denom wise deposit storage
// Ex.  {epochday} + ":" + {lockupString} + ":" + {uid} + ":" + {denom}
func CreateEpochLockupUserDenomDepositKey(uid, sep, denom string, epochday uint64, lockupPeriod LockupTypes) []byte {
	var b bytes.Buffer
	strEpochday := strconv.FormatUint(epochday, 10)
	b.WriteString(strEpochday)
	b.WriteString(sep)
	lockupPeriodStr := LockupTypes_name[int32(lockupPeriod)]
	b.WriteString(lockupPeriodStr)
	b.WriteString(sep)
	b.WriteString(uid)
	b.WriteString(sep)
	b.WriteString(denom)

	return b.Bytes()
}

// CreateEpochLockupKey create the prefix store key for the epochday lockup wise deposit.
// This key is used for the prefix key iteration to get the deposits done on a given epochday
// Ex.  {epochday} + ":" + "lockupString" + ":"
func CreateEpochLockupUserKey(epochday uint64, lockupPeriod LockupTypes, sep string) []byte {
	var b bytes.Buffer
	strEpochday := strconv.FormatUint(epochday, 10)
	b.WriteString(strEpochday)
	b.WriteString(sep)
	lockupPeriodStr := LockupTypes_name[int32(lockupPeriod)]
	b.WriteString(lockupPeriodStr)
	b.WriteString(sep)
	return b.Bytes()
}

// CreateEpochLockupUserSepKey  create the prefix store key for the iteration.
// This key is used for the prefix key iteration to get the deposits done on a given epochday
// Ex.  {epochday} + ":" + "lockupString" + ":" + {uid} + ":"
func CreateEpochLockupUserSepKey(epochday uint64, lockupPeriodStr, uid, sep string) []byte {
	var b bytes.Buffer
	strEpochday := strconv.FormatUint(epochday, 10)
	b.WriteString(strEpochday)
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

// CreateWithdrableKey create key for the withdrable KV store to fetch current
// withdrable amount by a given user of given denom.
// Key = {denom} + ":" + {uid}
func CreateWithdrableKey(denom, uid, sep string) []byte {
	var b bytes.Buffer
	b.WriteString(denom)
	b.WriteString(sep)
	b.WriteString(uid)
	return b.Bytes()
}

// CreateWithdrableKey create key for the lockup period based withdrable KV store to fetch current
// withdrable amount by a given user, denom and lockup period
// Key = {denom} + ":" + {uid} + ":" + {lockupPeriod}
func CreateLockupWithdrableKey(denom, uid string, lockupPeriod LockupTypes, sep string) []byte {
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

// set of key creation functions for withdraw objects
func CreateWithdrawCountKey() []byte {
	return createStoreKey(WithdrawCountKey)
}

// Claim key

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

// ParseEpochLockupUserDenomDepositKey split the composit key of type " {epochday} + ":" + {lockupString} + ":" + {uid} + ":" + {denom}"
// and split into field variable and return accordingly
func ParseEpochLockupUserDenomDepositKey(key []byte) (epochday uint64, lockupStr, uid, denom string, err error) {
	split := SplitKeyBytes(key)
	epochdayStr := string(split[0])
	epochday, err = strconv.ParseUint(epochdayStr, 10, 64)
	lockupStr = string(split[1])
	uid = string(split[2])
	denom = string(split[3])
	return
}

// EpochDayKey create key  for the epochday
func EpochDaySepKey(epochday uint64, sep string) []byte {
	var b bytes.Buffer
	strEpochday := strconv.FormatUint(epochday, 10)
	b.WriteString(strEpochday)
	b.WriteString(sep)
	return b.Bytes()
}
