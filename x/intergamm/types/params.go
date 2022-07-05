package types

import (
	fmt "fmt"

	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	"gopkg.in/yaml.v2"
)

var _ paramtypes.ParamSet = (*Params)(nil)

var (
	KeyOsmoTokenTransferChannels        = []byte("OsmoTokenTransferChannels")
	DefaultOsmoTokenTransferChannelsMap = map[string]string{"osmosis-test": "channel-1"}
)

// ParamKeyTable the param key table for launch module
func ParamKeyTable() paramtypes.KeyTable {
	return paramtypes.NewKeyTable().RegisterParamSet(&Params{})
}

// NewParams creates a new Params instance
func NewParams(m map[string]string) Params {
	return Params{OsmoTokenTransferChannels: m}
}

// DefaultParams returns a default set of parameters
func DefaultParams() Params {
	return NewParams(DefaultOsmoTokenTransferChannelsMap)
}

// ParamSetPairs get the params.ParamSet
func (p *Params) ParamSetPairs() paramtypes.ParamSetPairs {
	return paramtypes.ParamSetPairs{
		paramtypes.NewParamSetPair(KeyOsmoTokenTransferChannels, &p.OsmoTokenTransferChannels, validateOsmoTokenTransferChannels),
	}
}

// Validate validates the set of params
func (p Params) Validate() error {
	if err := validateOsmoTokenTransferChannels(p.OsmoTokenTransferChannels); err != nil {
		return err
	}
	return nil
}

// String implements the Stringer interface.
func (p Params) String() string {
	out, _ := yaml.Marshal(p)
	return string(out)
}

func validateOsmoTokenTransferChannels(i interface{}) error {
	_, ok := i.(map[string]string)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", i)
	}

	return nil
}
