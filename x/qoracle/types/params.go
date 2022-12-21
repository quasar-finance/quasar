package types

import (
	"fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	"gopkg.in/yaml.v2"
)

var _ paramtypes.ParamSet = (*Params)(nil)

var (
	KeyDenomPriceMappings = []byte("DenomPriceMappings")
	KeyOneHopDenomMap     = []byte("oneHopDenomMap")

	DefaultDenomPriceMappings = []DenomPriceMapping{
		{
			Denom:       "uatom",
			OracleDenom: "ATOM",
			Multiplier:  sdk.NewDecWithPrec(1, 6),
		},
		{
			Denom:       "uosmo",
			OracleDenom: "OSMO",
			Multiplier:  sdk.NewDecWithPrec(1, 6),
		},
	}
	denom1 OneHopIbcDenomMapping = OneHopIbcDenomMapping{OriginName: "uatom", Quasar: "IBC/TESTATOM", Osmo: "IBC/TESTOSMO"}
	denom2 OneHopIbcDenomMapping = OneHopIbcDenomMapping{OriginName: "uosmo", Quasar: "IBC/TESTOSMO", Osmo: "uosmo"}

	DefaultOneHopDenomMap = []*OneHopIbcDenomMapping{&denom1, &denom2}
)

// ParamKeyTable the param key table for launch module
func ParamKeyTable() paramtypes.KeyTable {
	return paramtypes.NewKeyTable().RegisterParamSet(&Params{})
}

// NewParams creates a new Params instance
func NewParams(
	denomPriceMappings []DenomPriceMapping,
	onehopDenoms []*OneHopIbcDenomMapping,
) Params {
	return Params{
		DenomPriceMappings: denomPriceMappings,
		OneHopDenomMap:     onehopDenoms,
	}
}

// DefaultParams returns a default set of parameters
func DefaultParams() Params {
	return NewParams(
		DefaultDenomPriceMappings,
		DefaultOneHopDenomMap,
	)
}

// ParamSetPairs get the params.ParamSet
func (p *Params) ParamSetPairs() paramtypes.ParamSetPairs {
	return paramtypes.ParamSetPairs{
		paramtypes.NewParamSetPair(KeyDenomPriceMappings, &p.DenomPriceMappings, validateDenomPriceMappings),
		paramtypes.NewParamSetPair(KeyOneHopDenomMap, &p.OneHopDenomMap, validateOneHopDenomMaps),
	}
}

// Validate validates the set of params
func (p Params) Validate() error {
	if err := validateDenomPriceMappings(p.DenomPriceMappings); err != nil {
		return err
	}

	if err := validateOneHopDenomMaps(p.OneHopDenomMap); err != nil {
		return err
	}
	return nil
}

// String implements the Stringer interface.
func (p Params) String() string {
	out, _ := yaml.Marshal(p)
	return string(out)
}

// validateDenomPriceMappings validates the denom price mappings
func validateDenomPriceMappings(v interface{}) error {
	mappings, ok := v.([]DenomPriceMapping)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", v)
	}

	for i, mapping := range mappings {
		if err := mapping.Validate(); err != nil {
			return fmt.Errorf("invalid denom price mapping at index %d: %w", i, err)
		}
	}

	return nil
}

// validateOneHopDenomMaps validates the StableDenoms param
func validateOneHopDenomMaps(v interface{}) error {
	oneHopDenomMaps, ok := v.([]*OneHopIbcDenomMapping)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", v)
	}

	// TODO implement validation
	_ = oneHopDenomMaps

	return nil
}
