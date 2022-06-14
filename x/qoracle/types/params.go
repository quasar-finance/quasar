package types

import (
	"errors"
	"fmt"
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	clienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
	host "github.com/cosmos/ibc-go/v3/modules/core/24-host"
	"gopkg.in/yaml.v2"
)

var _ paramtypes.ParamSet = (*Params)(nil)

var (
	KeyBandchainParams = []byte("BandchainParams")
	KeyOracleAccounts  = []byte("OracleAccounts")
	KeyStableDenoms    = []byte("stableDenoms")
	KeyOneHopDenomMap  = []byte("oneHopDenomMap")

	// TODO: Determine the default value
	DefaultBandchainParams = BandchainParams{
		OracleIbcParams: IBCParams{
			AuthorizedChannel: "",
			TimeoutHeight:     clienttypes.NewHeight(0, 0),
			TimeoutTimestamp:  uint64(time.Minute * 10),
		},
		CoinRatesScriptParams: OracleScriptParams{
			ScriptId:   37,
			AskCount:   4,
			MinCount:   3,
			FeeLimit:   sdk.NewCoins(sdk.NewCoin("uband", sdk.NewInt(10))),
			PrepareGas: 6000,
			ExecuteGas: 6000,
		},
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
	bandchainParams BandchainParams,
	oracleAccounts string,
	stableDenoms []string,
	onehopDenoms []*OneHopIbcDenomMapping,
) Params {
	return Params{
		BandchainParams: bandchainParams,
		OracleAccounts:  oracleAccounts,
		StableDenoms:    stableDenoms, // AUDIT slice copy
		OneHopDenomMap:  onehopDenoms,
	}
}

// DefaultParams returns a default set of parameters
func DefaultParams() Params {
	return NewParams(
		DefaultBandchainParams,
		DefaultOracleAccounts,
		DefaultStableDenoms,
		DefaultOneHopDenomMap,
	)
}

// ParamSetPairs get the params.ParamSet
func (p *Params) ParamSetPairs() paramtypes.ParamSetPairs {
	return paramtypes.ParamSetPairs{
		paramtypes.NewParamSetPair(KeyBandchainParams, &p.BandchainParams, validateBandchainParams),
		paramtypes.NewParamSetPair(KeyOracleAccounts, &p.OracleAccounts, validateOracleAccounts),
		paramtypes.NewParamSetPair(KeyStableDenoms, &p.StableDenoms, validateStableDenoms),
		paramtypes.NewParamSetPair(KeyOneHopDenomMap, &p.OneHopDenomMap, validateOneHopDenomMaps),
	}
}

// Validate validates the set of params
func (p Params) Validate() error {
	if err := validateBandchainParams(p.BandchainParams); err != nil {
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

func validateBandchainParams(v interface{}) error {
	params, ok := v.(BandchainParams)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", v)
	}

	err := params.OracleIbcParams.Validate()
	if err != nil {
		return err
	}

	err = params.CoinRatesScriptParams.Validate()
	if err != nil {
		return err
	}

	return nil
}

func (p IBCParams) Validate() error {
	if err := host.ChannelIdentifierValidator(p.AuthorizedChannel); err != nil {
		return err
	}

	if p.TimeoutHeight.IsZero() && p.TimeoutTimestamp == 0 {
		return errors.New("packet timeout height and packet timeout timestamp cannot both be 0")
	}

	return nil
}

func (p OracleScriptParams) Validate() error {
	if p.ScriptId == 0 {
		return errors.New("script id cannot be 0")
	}

	if p.AskCount == 0 {
		return errors.New("ask count cannot be 0")
	}
	if p.MinCount == 0 {
		return errors.New("min count cannot be 0")
	}
	if p.AskCount < p.MinCount {
		return errors.New("ask count cannot be less than min count")
	}

	if p.FeeLimit.IsAnyNegative() || p.FeeLimit.IsZero() {
		return errors.New("fee limit cannot be negative or zero")
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
