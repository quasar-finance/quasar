package types

import (
	"errors"
	"fmt"

	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	host "github.com/cosmos/ibc-go/v3/modules/core/24-host"
	"gopkg.in/yaml.v2"
)

var _ paramtypes.ParamSet = (*Params)(nil)

var (
	KeyBandchainIBCParams = []byte("BandchainIBCParams")
	KeyOracleAccounts     = []byte("OracleAccounts")
	KeyStableDenoms       = []byte("stableDenoms")
	KeyOneHopDenomMap     = []byte("oneHopDenomMap")
	// TODO: Determine the default value
	DefaultBandchainIBCParams = BandchainIBCParams{
		OraclePortId:     "oracle",
		OracleIBCVersion: "bandchain-1",
		ChannelId:        "",
	}
	DefaultOracleAccounts string                = "oracle_accounts"
	DefaultStableDenoms                         = []string{"UST", "USTTESTA"}
	denom1                OneHopIbcDenomMapping = OneHopIbcDenomMapping{OriginName: "uatom", Quasar: "IBC/TESTATOM", Osmo: "IBC/TESTOSMO"}
	denom2                OneHopIbcDenomMapping = OneHopIbcDenomMapping{OriginName: "uosmo", Quasar: "IBC/TESTOSMO", Osmo: "uosmo"}

	DefaultOneHopDenomMap = []*OneHopIbcDenomMapping{&denom1, &denom2}
)

// ParamKeyTable the param key table for launch module
func ParamKeyTable() paramtypes.KeyTable {
	return paramtypes.NewKeyTable().RegisterParamSet(&Params{})
}

// NewParams creates a new Params instance
func NewParams(
	bandchainIBCParams BandchainIBCParams,
	oracleAccounts string,
	stableDenoms []string,
	onehopDenoms []*OneHopIbcDenomMapping,
) Params {
	return Params{
		BandchainIBCParams: bandchainIBCParams,
		OracleAccounts:     oracleAccounts,
		StableDenoms:       stableDenoms, // AUDIT slice copy
		OneHopDenomMap:     onehopDenoms,
	}
}

// DefaultParams returns a default set of parameters
func DefaultParams() Params {
	return NewParams(
		DefaultBandchainIBCParams,
		DefaultOracleAccounts,
		DefaultStableDenoms,
		DefaultOneHopDenomMap,
	)
}

// ParamSetPairs get the params.ParamSet
func (p *Params) ParamSetPairs() paramtypes.ParamSetPairs {
	return paramtypes.ParamSetPairs{
		paramtypes.NewParamSetPair(KeyBandchainIBCParams, &p.BandchainIBCParams, validateBandchainIBCParams),
		paramtypes.NewParamSetPair(KeyOracleAccounts, &p.OracleAccounts, validateOracleAccounts),
		paramtypes.NewParamSetPair(KeyStableDenoms, &p.StableDenoms, validateStableDenoms),
		paramtypes.NewParamSetPair(KeyOneHopDenomMap, &p.OneHopDenomMap, validateOneHopDenomMaps),
	}
}

// Validate validates the set of params
func (p Params) Validate() error {
	if err := validateBandchainIBCParams(p.BandchainIBCParams); err != nil {
		return err
	}

	if err := validateOracleAccounts(p.OracleAccounts); err != nil {
		return err
	}

	if err := validateStableDenoms(p.StableDenoms); err != nil {
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

func validateBandchainIBCParams(v interface{}) error {
	params, ok := v.(BandchainIBCParams)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", v)
	}

	err := host.PortIdentifierValidator(params.OraclePortId)
	if err != nil {
		return err
	}

	if params.OracleIBCVersion == "" {
		return errors.New("oracle IBC version cannot be empty")
	}

	// Only validate channel id if it's set
	if params.ChannelId != "" {
		err = host.ChannelIdentifierValidator(params.ChannelId)
		if err != nil {
			return err
		}
	}

	return nil
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

// validateStableDenoms validates the StableDenoms param
func validateStableDenoms(v interface{}) error {
	stableDenoms, ok := v.([]string)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", v)
	}

	// TODO implement validation
	_ = stableDenoms

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
