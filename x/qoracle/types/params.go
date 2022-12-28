package types

import (
	"fmt"
	time "time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	"gopkg.in/yaml.v2"
)

var _ paramtypes.ParamSet = (*Params)(nil)

const (
	// DefaultDenomPricesExpDuration is the default duration in which denom prices are valid
	DefaultDenomPricesExpDuration = uint64(time.Minute * 6)
)

var (
	DefaultDenomPriceMappings = []DenomPriceMapping{
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
)

var (
	KeyDenomPriceMappings = []byte("DenomPriceMappings")
	// KeyDenomPricesExpDuration is store's key for DenomPricesExpDuration
	KeyDenomPricesExpDuration = []byte("DenomPricesExpDuration")
)

// ParamKeyTable the param key table for launch module
func ParamKeyTable() paramtypes.KeyTable {
	return paramtypes.NewKeyTable().RegisterParamSet(&Params{})
}

// NewParams creates a new Params instance
func NewParams(
	denomPriceMappings []DenomPriceMapping,
	expDuration uint64,
) Params {
	return Params{
		DenomPriceMappings:     denomPriceMappings,
		DenomPricesExpDuration: expDuration,
	}
}

// DefaultParams returns a default set of parameters
func DefaultParams() Params {
	return NewParams(
		DefaultDenomPriceMappings,
		DefaultDenomPricesExpDuration,
	)
}

// ParamSetPairs get the params.ParamSet
func (p *Params) ParamSetPairs() paramtypes.ParamSetPairs {
	return paramtypes.ParamSetPairs{
		paramtypes.NewParamSetPair(KeyDenomPriceMappings, &p.DenomPriceMappings, validateDenomPriceMappings),
		paramtypes.NewParamSetPair(KeyDenomPricesExpDuration, &p.DenomPricesExpDuration, validateDuration),
	}
}

// Validate validates the set of params
func (p Params) Validate() error {
	if err := validateDenomPriceMappings(p.DenomPriceMappings); err != nil {
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

func validateDuration(i interface{}) error {
	_, ok := i.(uint64)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", i)
	}

	return nil
}
