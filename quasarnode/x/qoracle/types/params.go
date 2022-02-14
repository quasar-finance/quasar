package types

import (
	"fmt"

	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	"gopkg.in/yaml.v2"
)

var _ paramtypes.ParamSet = (*Params)(nil)

var (
	KeyOracleAccounts = []byte("OracleAccounts")
	// TODO: Determine the default value
	DefaultOracleAccounts string = "oracle_accounts"
)

// ParamKeyTable the param key table for launch module
func ParamKeyTable() paramtypes.KeyTable {
	return paramtypes.NewKeyTable().RegisterParamSet(&Params{})
}

// NewParams creates a new Params instance
func NewParams(
	oracleAccounts string,
) Params {
	return Params{
		OracleAccounts: oracleAccounts,
	}
}

// DefaultParams returns a default set of parameters
func DefaultParams() Params {
	return NewParams(
		DefaultOracleAccounts,
	)
}

// ParamSetPairs get the params.ParamSet
func (p *Params) ParamSetPairs() paramtypes.ParamSetPairs {
	return paramtypes.ParamSetPairs{
		paramtypes.NewParamSetPair(KeyOracleAccounts, &p.OracleAccounts, validateOracleAccounts),
	}
}

// Validate validates the set of params
func (p Params) Validate() error {
	if err := validateOracleAccounts(p.OracleAccounts); err != nil {
		return err
	}

	return nil
}

// String implements the Stringer interface.
func (p Params) String() string {
	out, _ := yaml.Marshal(p)
	return string(out)
}

// validateOracleAccounts validates the OracleAccounts param
func validateOracleAccounts(v interface{}) error {
	oracleAccounts, ok := v.(string)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", v)
	}

	// TODO implement validation
	_ = oracleAccounts

	return nil
}
