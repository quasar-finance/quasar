package types

import (
	"fmt"

	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	"gopkg.in/yaml.v2"
)

var _ paramtypes.ParamSet = (*Params)(nil)

// DefaultWasmHooksEnabled is the default value for WasmHooksEnabled
var DefaultWasmHooksEnabled = true

// KeyWasmHooksEnabled is parameter store key for WasmHooksEnabled
var KeyWasmHooksEnabled = []byte("WasmHooksEnabled")

// ParamKeyTable for qtransfer module
func ParamKeyTable() paramtypes.KeyTable {
	return paramtypes.NewKeyTable().RegisterParamSet(&Params{})
}

// NewParams creates a new Params instance
func NewParams(wasmHooksEnabled bool) Params {
	return Params{
		WasmHooksEnabled: wasmHooksEnabled,
	}
}

// DefaultParams defines the parameters for this module
func DefaultParams() Params {
	return Params{
		WasmHooksEnabled: DefaultWasmHooksEnabled,
	}
}

// ParamSetPairs implements the ParamSet interface and returns all the key/value pairs
// qtransfer module's parameters.
func (p *Params) ParamSetPairs() paramtypes.ParamSetPairs {
	return paramtypes.ParamSetPairs{
		paramtypes.NewParamSetPair(KeyWasmHooksEnabled, &p.WasmHooksEnabled, validateEnabled),
	}
}

// String implements the Stringer interface.
func (p Params) String() string {
	out, _ := yaml.Marshal(p)
	return string(out)
}

// Validate validates the set of params
func (p Params) Validate() error {
	err := validateEnabled(p.WasmHooksEnabled)
	return err
}

// validateEnabled is used to validate the enabled param type.
func validateEnabled(i any) error {
	_, ok := i.(bool)
	if !ok {
		return fmt.Errorf("invalid qtransfer enabled parameter type: %T", i)
	}
	return nil
}
