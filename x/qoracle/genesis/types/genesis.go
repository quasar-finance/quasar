package types

// this line is used by starport scaffolding # genesis/types/import

import (
	fmt "fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
	host "github.com/cosmos/ibc-go/v5/modules/core/24-host"
	qbandtypes "github.com/quasarlabs/quasarnode/x/qoracle/bandchain/types"
	qosmotypes "github.com/quasarlabs/quasarnode/x/qoracle/osmosis/types"
	types "github.com/quasarlabs/quasarnode/x/qoracle/types"
)

// DefaultGenesis returns the default Capability genesis state
func DefaultGenesis() *GenesisState {
	return &GenesisState{
		Params:                types.DefaultParams(),
		DenomSymbolMappings:   DefaultSymbolMappings(),
		BandchainGenesisState: DefaultBandchainGenesis(),
		OsmosisGenesisState:   DefaultOsmosisGenesis(),
	}
}

// NewGenesisState creates and returns a new GenesisState instance from the provided controller and host genesis state types
func NewGenesisState(params types.Params, bandchainGenesisState BandchainGenesisState, osmosisGenesisState OsmosisGenesisState) *GenesisState {
	return &GenesisState{
		Params:                params,
		BandchainGenesisState: bandchainGenesisState,
		OsmosisGenesisState:   osmosisGenesisState,
	}
}

// Validate performs basic genesis state validation returning an error upon any
// failure.
func (gs GenesisState) Validate() error {
	if err := gs.Params.Validate(); err != nil {
		return err
	}

	// validateDenomPriceMappings validates the denom price mappings
	for i, mapping := range gs.DenomSymbolMappings {
		if err := mapping.Validate(); err != nil {
			return fmt.Errorf("invalid denom price mapping at index %d: %w", i, err)
		}
	}

	if err := gs.BandchainGenesisState.Validate(); err != nil {
		return err
	}

	if err := gs.OsmosisGenesisState.Validate(); err != nil {
		return err
	}

	return nil
}

// DefaultSymbolMappings creates and returns the default qoracle DefaultSymbolMappings
func DefaultSymbolMappings() []types.DenomSymbolMapping {
	return []types.DenomSymbolMapping{
		{
			Denom:        "uatom",
			OracleSymbol: "ATOM",
			Multiplier:   sdk.NewDecWithPrec(1, 6),
		},
		{
			Denom:        "uosmo",
			OracleSymbol: "OSMO",
			Multiplier:   sdk.NewDecWithPrec(1, 6),
		},
	}
}

// DefaultBandchainGenesis creates and returns the default qoracle DefaultBandchainGenesis
func DefaultBandchainGenesis() BandchainGenesisState {
	return BandchainGenesisState{
		Port:   qbandtypes.PortID,
		Params: qbandtypes.DefaultParams(),
	}
}

// NewBandchainGenesisState creates a returns a new BandchainGenesisState instance
func NewBandchainGenesisState(port string, params qbandtypes.Params) BandchainGenesisState {
	return BandchainGenesisState{
		Port:   port,
		Params: params,
	}
}

// Validate performs basic validation of the BandchainGenesisState
func (gs BandchainGenesisState) Validate() error {
	if err := host.PortIdentifierValidator(gs.Port); err != nil {
		return err
	}

	if err := gs.Params.Validate(); err != nil {
		return err
	}

	return nil
}

// DefaultOsmosisGenesis creates and returns the default qoracle DefaultOsmosisGenesis
func DefaultOsmosisGenesis() OsmosisGenesisState {
	return OsmosisGenesisState{
		Port:   qosmotypes.PortID,
		Params: qosmotypes.DefaultParams(),
	}
}

// NewOsmosisGenesisState creates a returns a new OsmosisGenesisState instance
func NewOsmosisGenesisState(port string, params qosmotypes.Params) OsmosisGenesisState {
	return OsmosisGenesisState{
		Port:   port,
		Params: params,
	}
}

// Validate performs basic validation of the OsmosisGenesisState
func (gs OsmosisGenesisState) Validate() error {
	if err := host.PortIdentifierValidator(gs.Port); err != nil {
		return err
	}

	if err := gs.Params.Validate(); err != nil {
		return err
	}

	return nil
}
