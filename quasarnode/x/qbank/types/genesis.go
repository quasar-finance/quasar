package types

import (
	"fmt"
)

// DefaultIndex is the default capability global index
const DefaultIndex uint64 = 1

// DefaultGenesis returns the default Capability genesis state
func DefaultGenesis() *GenesisState {
	return &GenesisState{
		DepositList:  []Deposit{},
		WithdrawList: []Withdraw{},
		FeeData:      nil,
		// this line is used by starport scaffolding # genesis/types/default
		Params: DefaultParams(),
	}
}

// Validate performs basic genesis state validation returning an error upon any
// failure.
func (gs GenesisState) Validate() error {
	// Check for duplicated ID in deposit
	depositIdMap := make(map[uint64]bool)
	depositCount := gs.GetDepositCount()
	for _, elem := range gs.DepositList {
		if _, ok := depositIdMap[elem.Id]; ok {
			return fmt.Errorf("duplicated id for deposit")
		}
		if elem.Id >= depositCount {
			return fmt.Errorf("deposit id should be lower or equal than the last id")
		}
		depositIdMap[elem.Id] = true
	}
	// Check for duplicated ID in withdraw
	withdrawIdMap := make(map[uint64]bool)
	withdrawCount := gs.GetWithdrawCount()
	for _, elem := range gs.WithdrawList {
		if _, ok := withdrawIdMap[elem.Id]; ok {
			return fmt.Errorf("duplicated id for withdraw")
		}
		if elem.Id >= withdrawCount {
			return fmt.Errorf("withdraw id should be lower or equal than the last id")
		}
		withdrawIdMap[elem.Id] = true
	}
	// this line is used by starport scaffolding # genesis/types/validate

	return gs.Params.Validate()
}
