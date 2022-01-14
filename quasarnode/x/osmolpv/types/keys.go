package types

import (
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
)

func KeyPrefix(p string) []byte {
	return []byte(p)
}

var (
	UserReceiptCoinsKBP = []byte{0x01}
)

func CreateUserReceiptCoinsKey(addr sdk.AccAddress) []byte {
	return addr.Bytes()
}
