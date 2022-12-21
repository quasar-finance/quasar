package types

import (
	"errors"
	"fmt"
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	clienttypes "github.com/cosmos/ibc-go/v5/modules/core/02-client/types"
	host "github.com/cosmos/ibc-go/v5/modules/core/24-host"
	"gopkg.in/yaml.v2"
)

const (
	// DefaultEnabled determines that the module is enabled by default
	DefaultEnabled = true
	// DefaultAuthorizedChannel is empty string which means no authorized channel
	DefaultAuthorizedChannel = ""
	// DefaultPacketTimeoutHeight is 0-0 which means no timeout based on height
	DefaultPacketTimeoutHeight = "0-0"
	// DefaultPacketTimeoutTimestamp is 3 mins
	DefaultPacketTimeoutTimestamp = uint64(time.Minute * 3)
	// DefaultPriceListExpDuration is the default duration in which price list is valid
	DefaultPriceListExpDuration = uint64(time.Minute * 6)
)

var (
	// DefaultCoinRatesParams is the default configuration for coin rates (a.k.a token prices) based on bandchain testnet
	DefaultCoinRatesParams = CoinRatesParams{
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
	}
)

var (
	// KeyEnabled is store's key for Enabled
	KeyEnabled = []byte("Enabled")
	// KeyAuthorizedChannel is store's key for AuthorizedChannel
	KeyAuthorizedChannel = []byte("AuthorizedChannel")
	// KeyPacketTimeoutHeight is store's key for PacketTimeoutHeight
	KeyPacketTimeoutHeight = []byte("PacketTimeoutHeight")
	// KeyPacketTimeoutTimestamp is store's key for PacketTimeoutTimestamp
	KeyPacketTimeoutTimestamp = []byte("PacketTimeoutTimestamp")
	// KeyPriceListExpDuration is store's key for PriceListExpDuration
	KeyPriceListExpDuration = []byte("PriceListExpDuration")
	// KeyCoinRatesParams is store's key for CoinRatesParams
	KeyCoinRatesParams = []byte("CoinRatesParams")
)

// ParamKeyTable the param key table for launch module
func ParamKeyTable() paramtypes.KeyTable {
	return paramtypes.NewKeyTable().RegisterParamSet(&Params{})
}

// NewParams creates a new Params instance
func NewParams(
	enabled bool,
	authorizedChan string,
	timeoutHeight clienttypes.Height,
	TimeoutTimestamp uint64,
	expDuration uint64,
	coinRatesParams CoinRatesParams,
) Params {
	return Params{
		Enabled:                enabled,
		AuthorizedChannel:      authorizedChan,
		PacketTimeoutHeight:    timeoutHeight,
		PacketTimeoutTimestamp: TimeoutTimestamp,
		PriceListExpDuration:   expDuration,
		CoinRatesParams:        coinRatesParams,
	}
}

// DefaultParams returns a default set of parameters
func DefaultParams() Params {
	return NewParams(
		DefaultEnabled,
		DefaultAuthorizedChannel,
		clienttypes.MustParseHeight(DefaultPacketTimeoutHeight),
		DefaultPacketTimeoutTimestamp,
		DefaultPriceListExpDuration,
		DefaultCoinRatesParams,
	)
}

// ParamSetPairs get the params.ParamSet
func (p *Params) ParamSetPairs() paramtypes.ParamSetPairs {
	return paramtypes.ParamSetPairs{
		paramtypes.NewParamSetPair(KeyEnabled, &p.Enabled, validateEnabled),
		paramtypes.NewParamSetPair(KeyAuthorizedChannel, &p.AuthorizedChannel, validateAuthorizedChannel),
		paramtypes.NewParamSetPair(KeyPacketTimeoutHeight, &p.PacketTimeoutHeight, validateClientHeight),
		paramtypes.NewParamSetPair(KeyPacketTimeoutTimestamp, &p.PacketTimeoutTimestamp, validateDuration),
		paramtypes.NewParamSetPair(KeyPriceListExpDuration, &p.PriceListExpDuration, validateDuration),
		paramtypes.NewParamSetPair(KeyCoinRatesParams, &p.CoinRatesParams, validateCoinRatesParams),
	}
}

// Validate validates the set of params
func (p Params) Validate() error {
	err := validateAuthorizedChannel(p.AuthorizedChannel)
	if err != nil {
		return err
	}

	if p.PacketTimeoutHeight.IsZero() && p.PacketTimeoutTimestamp == 0 {
		return errors.New("packet timeout height and packet timeout timestamp cannot both be 0")
	}

	err = p.CoinRatesParams.Validate()
	if err != nil {
		return err
	}

	return nil
}

// String implements the Stringer interface.
func (p Params) String() string {
	out, _ := yaml.Marshal(p)
	return string(out)
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

func validateEnabled(i interface{}) error {
	_, ok := i.(bool)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", i)
	}

	return nil
}

func validateAuthorizedChannel(i interface{}) error {
	channelID, ok := i.(string)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", i)
	}

	if channelID != "" {
		if err := host.ChannelIdentifierValidator(channelID); err != nil {
			return fmt.Errorf("invalid authorized channel: %w", err)
		}
	}

	return nil
}

func validateClientHeight(i interface{}) error {
	_, ok := i.(clienttypes.Height)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", i)
	}

	return nil
}

func validateDuration(i interface{}) error {
	_, ok := i.(uint64)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", i)
	}

	return nil
}

func validateCoinRatesParams(v interface{}) error {
	params, ok := v.(CoinRatesParams)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", v)
	}

	err := params.Validate()
	if err != nil {
		return err
	}

	return nil
}
