package types

import (
	"gopkg.in/yaml.v2"
	"time"

	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
)

var _ paramtypes.ParamSet = (*Params)(nil)

var (
	// DefaultDenomPricesExpDuration is the default duration in which denom prices are valid
	DefaultDenomPricesExpDuration = uint64(time.Minute * 6)
	DefaultDenomSymbolMapping     = []DenomSymbolMapping{}
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

func NewParams() Params {
	return Params{}
}

// DefaultParams returns a default set of parameters
func DefaultParams() Params {
	return NewParams()
}

// ParamSetPairs get the params.ParamSet
func (p *Params) ParamSetPairs() paramtypes.ParamSetPairs {
	return paramtypes.ParamSetPairs{}
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
