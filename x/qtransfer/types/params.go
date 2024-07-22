package types

import (
	"fmt"
	"gopkg.in/yaml.v2"

	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
)

var _ paramtypes.ParamSet = (*Params)(nil)

var (
	// DefaultWasmHooksEnabled is the default value for WasmHooksEnabled
	DefaultWasmHooksEnabled = true
)

var (
	// KeyWasmHooksEnabled is parameter store key for WasmHooksEnabled
	KeyWasmHooksEnabled = []byte("WasmHooksEnabled")
)

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
	if err := validateEnabled(p.WasmHooksEnabled); err != nil {
		return err
	}
	return nil
}

// validateEnabled is used to validate the enabled param type.
func validateEnabled(i interface{}) error {
	_, ok := i.(bool)
	if !ok {
		return fmt.Errorf("invalid qtransfer enabled parameter type: %T", i)
	}
	return nil
}
