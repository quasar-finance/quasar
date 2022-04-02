package types

// DefaultIndex is the default capability global index
const DefaultIndex uint64 = 1

// TODO | AUDIT | qbank genesis state to be redefined about the kind of state object/objects it should
// keep in the genesis state/file

// DefaultGenesis returns the default Capability genesis state
func DefaultGenesis() *GenesisState {
	return &GenesisState{

		// this line is used by starport scaffolding # genesis/types/default
		Params: DefaultParams(),
	}
}

// Validate performs basic genesis state validation returning an error upon any
// failure.
// Note - This validation make sense to make sure no duplicate entry exist for any deposit object
// which is live. depositCount and withdrawCount should represent the current live count.

func (gs GenesisState) Validate() error {
	// Check for duplicated ID in deposit

	// this line is used by starport scaffolding # genesis/types/validate

	return gs.Params.Validate()
}
