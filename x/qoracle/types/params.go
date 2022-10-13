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
	KeyBandchainParams    = []byte("BandchainParams")
	KeyOsmosisParams      = []byte("OsmosisParams")
	KeyDenomPriceMappings = []byte("DenomPriceMappings")
	KeyOneHopDenomMap     = []byte("oneHopDenomMap")

	// TODO: Determine the default value
	DefaultBandchainParams = BandchainParams{
		OracleIbcParams: IBCParams{
			AuthorizedChannel: "",
			TimeoutHeight:     clienttypes.NewHeight(0, 0),
			TimeoutTimestamp:  uint64(time.Minute * 10),
		},
		CoinRatesParams: CoinRatesParams{
			EpochIdentifier: "minute",
			Symbols:         []string{"BTC", "OSMO", "BNB", "ATOM"},
			ScriptParams: OracleScriptParams{
				ScriptId:   37,
				AskCount:   4,
				MinCount:   3,
				FeeLimit:   sdk.NewCoins(sdk.NewCoin("uband", sdk.NewInt(30))),
				PrepareGas: 600000,
				ExecuteGas: 600000,
			},
		},
	}
	DefaultOsmosisParams = OsmosisParams{
		ICQParams: IBCParams{
			AuthorizedChannel: "",
			TimeoutHeight:     clienttypes.NewHeight(0, 0),
			TimeoutTimestamp:  uint64(time.Minute * 10),
		},
		EpochIdentifier: "minute",
	}
	DefaultDenomPriceMappings = []DenomPriceMapping{
		{
			Denom:       "uatom",
			OracleDenom: "ATOM",
			Multiplier:  sdk.NewDecWithPrec(1, 6),
		},
		{
			Denom:       "uosmo",
			OracleDenom: "OSMO",
			Multiplier:  sdk.NewDecWithPrec(1, 6),
		},
	}
	denom1 OneHopIbcDenomMapping = OneHopIbcDenomMapping{OriginName: "uatom", Quasar: "IBC/TESTATOM", Osmo: "IBC/TESTOSMO"}
	denom2 OneHopIbcDenomMapping = OneHopIbcDenomMapping{OriginName: "uosmo", Quasar: "IBC/TESTOSMO", Osmo: "uosmo"}

	DefaultOneHopDenomMap = []*OneHopIbcDenomMapping{&denom1, &denom2}
)

// ParamKeyTable the param key table for launch module
func ParamKeyTable() paramtypes.KeyTable {
	return paramtypes.NewKeyTable().RegisterParamSet(&Params{})
}

// NewParams creates a new Params instance
func NewParams(
	bandchainParams BandchainParams,
	osmosisParams OsmosisParams,
	denomPriceMappings []DenomPriceMapping,
	onehopDenoms []*OneHopIbcDenomMapping,
) Params {
	return Params{
		BandchainParams:    bandchainParams,
		OsmosisParams:      osmosisParams,
		DenomPriceMappings: denomPriceMappings,
		OneHopDenomMap:     onehopDenoms,
	}
}

// DefaultParams returns a default set of parameters
func DefaultParams() Params {
	return NewParams(
		DefaultBandchainParams,
		DefaultOsmosisParams,
		DefaultDenomPriceMappings,
		DefaultOneHopDenomMap,
	)
}

// ParamSetPairs get the params.ParamSet
func (p *Params) ParamSetPairs() paramtypes.ParamSetPairs {
	return paramtypes.ParamSetPairs{
		paramtypes.NewParamSetPair(KeyBandchainParams, &p.BandchainParams, validateBandchainParams),
		paramtypes.NewParamSetPair(KeyOsmosisParams, &p.OsmosisParams, validateOsmosisParams),
		paramtypes.NewParamSetPair(KeyDenomPriceMappings, &p.DenomPriceMappings, validateDenomPriceMappings),
		paramtypes.NewParamSetPair(KeyOneHopDenomMap, &p.OneHopDenomMap, validateOneHopDenomMaps),
	}
}

// Validate validates the set of params
func (p Params) Validate() error {
	if err := validateBandchainParams(p.BandchainParams); err != nil {
		return err
	}

	if err := validateOsmosisParams(p.OsmosisParams); err != nil {
		return err
	}

	if err := validateDenomPriceMappings(p.DenomPriceMappings); err != nil {
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

	err = params.CoinRatesParams.Validate()
	if err != nil {
		return err
	}

	return nil
}

func (p IBCParams) Validate() error {
	if p.AuthorizedChannel != "" {
		if err := host.ChannelIdentifierValidator(p.AuthorizedChannel); err != nil {
			return fmt.Errorf("invalid authorized channel: %w", err)
		}
	}

	if p.TimeoutHeight.IsZero() && p.TimeoutTimestamp == 0 {
		return errors.New("packet timeout height and packet timeout timestamp cannot both be 0")
	}

	return nil
}

func (p CoinRatesParams) Validate() error {
	if len(p.Symbols) < 1 {
		return errors.New("symbols cannot be empty")
	}

	if err := p.ScriptParams.Validate(); err != nil {
		return fmt.Errorf("invalid oracle script params: %w", err)
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

func validateOsmosisParams(v interface{}) error {
	params, ok := v.(OsmosisParams)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", v)
	}

	err := params.ICQParams.Validate()
	if err != nil {
		return err
	}

	return nil
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
