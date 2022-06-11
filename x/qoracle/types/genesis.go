package types

// this line is used by starport scaffolding # genesis/types/import

import (
	"fmt"

	host "github.com/cosmos/ibc-go/v3/modules/core/24-host"
)

// DefaultIndex is the default capability global index
const DefaultIndex uint64 = 1

// DefaultGenesis returns the default Capability genesis state
func DefaultGenesis() *GenesisState {
	return &GenesisState{
		PortId:            PortID,
		PoolPositionList:  []PoolPosition{},
		PoolRanking:       nil,
		PoolSpotPriceList: []PoolSpotPrice{},
		PoolInfoList:      []PoolInfo{},
		// this line is used by starport scaffolding # genesis/types/default
		Params: DefaultParams(),
	}
}

// Validate performs basic genesis state validation returning an error upon any
// failure.
func (gs GenesisState) Validate() error {
	if err := host.PortIdentifierValidator(gs.PortId); err != nil {
		return err
	}

	// Check for duplicated index in poolPosition
	poolPositionIndexMap := make(map[string]struct{})

	for _, elem := range gs.PoolPositionList {
		b := CreatePoolPositionKey(elem.PoolId)
		index := string(b)
		if _, ok := poolPositionIndexMap[index]; ok {
			return fmt.Errorf("duplicated index for poolPosition")
		}
		poolPositionIndexMap[index] = struct{}{}
	}
	// Check for duplicated index in poolSpotPrice
	poolSpotPriceIndexMap := make(map[string]struct{})

	for _, elem := range gs.PoolSpotPriceList {
		index := string(CreatePoolSpotPriceKey(elem.PoolId, elem.DenomIn, elem.DenomOut))
		if _, ok := poolSpotPriceIndexMap[index]; ok {
			return fmt.Errorf("duplicated index for poolSpotPrice")
		}
		poolSpotPriceIndexMap[index] = struct{}{}
	}
	// Check for duplicated index in poolInfo
	poolInfoIndexMap := make(map[string]struct{})

	for _, elem := range gs.PoolInfoList {
		index := string(CreatePoolInfoKey(elem.PoolId))
		if _, ok := poolInfoIndexMap[index]; ok {
			return fmt.Errorf("duplicated index for poolInfo")
		}
		poolInfoIndexMap[index] = struct{}{}
	}
	// this line is used by starport scaffolding # genesis/types/validate

	return gs.Params.Validate()
}
