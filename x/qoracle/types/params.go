package types

import (
	"fmt"
	time "time"

	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	"gopkg.in/yaml.v2"
)

var _ paramtypes.ParamSet = (*Params)(nil)

var (
	// DefaultDenomPricesExpDuration is the default duration in which denom prices are valid
	DefaultDenomPricesExpDuration = uint64(time.Minute * 6)
	// ds1                           = DenomSymbolMapping{Denom: "", OracleSymbol: "", Multiplier: types.ZeroDec()}
	DefaultDenomSymbolMapping = []DenomSymbolMapping{}
)

var (
	// KeyDenomPricesExpDuration is store's key for DenomPricesExpDuration
	KeyDenomPricesExpDuration = []byte("DenomPricesExpDuration")
	KeyDenomSymbolMapping     = []byte("DenomSymbolMapping")
)

// ParamKeyTable the param key table for launch module
func ParamKeyTable() paramtypes.KeyTable {
	return paramtypes.NewKeyTable().RegisterParamSet(&Params{})
}

// NewParams creates a new Params instance
func NewParams(expDuration uint64, mapping []DenomSymbolMapping) Params {
	return Params{
		DenomPricesExpDuration: expDuration,
		Mappings:               mapping, // TODO - Is deep copy a better solution here.
	}
}

// DefaultParams returns a default set of parameters
func DefaultParams() Params {
	return NewParams(DefaultDenomPricesExpDuration, DefaultDenomSymbolMapping)
}

// ParamSetPairs get the params.ParamSet
func (p *Params) ParamSetPairs() paramtypes.ParamSetPairs {
	return paramtypes.ParamSetPairs{
		paramtypes.NewParamSetPair(KeyDenomPricesExpDuration, &p.DenomPricesExpDuration, validateDuration),
		paramtypes.NewParamSetPair(KeyDenomSymbolMapping, &p.Mappings, validateDenomSymbolMapping),
	}
}

// Validate validates the set of params
func (p Params) Validate() error {
	return nil
}

// String implements the Stringer interface.
func (p Params) String() string {
	out, _ := yaml.Marshal(p)
	return string(out)
}

func validateDuration(i interface{}) error {
	_, ok := i.(uint64)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", i)
	}

	return nil
}

func validateDenomSymbolMapping(m interface{}) error {
	_, ok := m.([]DenomSymbolMapping)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", m)
	}
	// TODO - Should also check if the param is sorted or not
	return nil
}
