package types

import (
	fmt "fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	"gopkg.in/yaml.v2"
)

var _ paramtypes.ParamSet = (*Params)(nil)

var (
	KeyPerfFeePer             = []byte("PerFeePer")
	KeyMgmtFeePer             = []byte("MgmtFeePer")
	DefaultPerfFeePer sdk.Dec = sdk.NewDecWithPrec(3, 2) // 3.00% , .03
	DefaultmgmtFeePer sdk.Dec = sdk.NewDecWithPrec(5, 3) // 0.5% ,  .05
)

// ParamKeyTable the param key table for launch module
func ParamKeyTable() paramtypes.KeyTable {
	return paramtypes.NewKeyTable().RegisterParamSet(&Params{})
}

// NewParams creates a new Params instance
func NewParams(perFeePer sdk.Dec, mgmtFeePer sdk.Dec) Params {
	return Params{PerfFeePer: perFeePer, MgmtFeePer: mgmtFeePer}
}

// DefaultParams returns a default set of parameters
func DefaultParams() Params {
	return NewParams(DefaultPerfFeePer, DefaultmgmtFeePer)
}

// ParamSetPairs get the params.ParamSet
func (p *Params) ParamSetPairs() paramtypes.ParamSetPairs {
	return paramtypes.ParamSetPairs{
		paramtypes.NewParamSetPair(KeyPerfFeePer, &p.PerfFeePer, validatePerfFeePer),
		paramtypes.NewParamSetPair(KeyMgmtFeePer, &p.MgmtFeePer, validateMgmtFeePer),
	}
}

// Validate validates the set of params
func (p Params) Validate() error {
	if err := validatePerfFeePer(p.PerfFeePer); err != nil {
		return err
	}
	if err := validateMgmtFeePer(p.PerfFeePer); err != nil {
		return err
	}
	return nil
}

// String implements the Stringer interface.
func (p Params) String() string {
	out, _ := yaml.Marshal(p)
	return string(out)
}

func validatePerfFeePer(i interface{}) error {
	v, ok := i.(sdk.Dec)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", i)
	}

	if v.IsNil() {
		return fmt.Errorf("perfFeePer must be not nil")
	}
	if v.IsNegative() {
		return fmt.Errorf("perfFeePer must be positive: %s", v)
	}
	if v.GT(sdk.OneDec()) {
		return fmt.Errorf("perfFeePer too large: %s", v)
	}

	return nil
}

func validateMgmtFeePer(i interface{}) error {
	v, ok := i.(sdk.Dec)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", i)
	}

	if v.IsNil() {
		return fmt.Errorf("mgmtFeePer must be not nil")
	}
	if v.IsNegative() {
		return fmt.Errorf("mgmtFeePer must be positive: %s", v)
	}
	if v.GT(sdk.OneDec()) {
		return fmt.Errorf("mgmtFeePer too large: %s", v)
	}

	return nil
}
