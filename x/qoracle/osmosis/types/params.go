package types

import (
	"errors"
	"fmt"
	"time"

	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	clienttypes "github.com/cosmos/ibc-go/v7/modules/core/02-client/types"
	host "github.com/cosmos/ibc-go/v7/modules/core/24-host"
	epochtypes "github.com/quasarlabs/quasarnode/x/epochs/types"
	"gopkg.in/yaml.v2"
)

const (
	// DefaultEnabled determines that the module is enabled by default
	DefaultEnabled = true
	// DefaultEpochIdentifier is set to minute
	DefaultEpochIdentifier = "minute"
	// DefaultAuthorizedChannel is empty string which means no authorized channel
	DefaultAuthorizedChannel = ""
	// DefaultPacketTimeoutHeight is 0-0 which means no timeout based on height
	DefaultPacketTimeoutHeight = "0-0"
	// DefaultPacketTimeoutTimestamp is 3 mins
	DefaultPacketTimeoutTimestamp = uint64(time.Minute * 3)
)

var (
	// KeyEnabled is store's key for Enabled
	KeyEnabled = []byte("Enabled")
	// KeyEpochIdentifier is the store's key for EpochIdentifier
	KeyEpochIdentifier = []byte("EpochIdentifier")
	// KeyAuthorizedChannel is store's key for AuthorizedChannel
	KeyAuthorizedChannel = []byte("AuthorizedChannel")
	// KeyPacketTimeoutHeight is store's key for PacketTimeoutHeight
	KeyPacketTimeoutHeight = []byte("PacketTimeoutHeight")
	// KeyPacketTimeoutTimestamp is store's key for PacketTimeoutTimestamp
	KeyPacketTimeoutTimestamp = []byte("PacketTimeoutTimestamp")
)

// ParamKeyTable the param key table for launch module
func ParamKeyTable() paramtypes.KeyTable {
	return paramtypes.NewKeyTable().RegisterParamSet(&Params{})
}

// NewParams creates a new Params instance
func NewParams(
	enabled bool,
	epochIdentifier string,
	authorizedChan string,
	timeoutHeight clienttypes.Height,
	TimeoutTimestamp uint64,
) Params {
	return Params{
		Enabled:                enabled,
		EpochIdentifier:        epochIdentifier,
		AuthorizedChannel:      authorizedChan,
		PacketTimeoutHeight:    timeoutHeight,
		PacketTimeoutTimestamp: TimeoutTimestamp,
	}
}

// DefaultParams returns a default set of parameters
func DefaultParams() Params {
	return NewParams(
		DefaultEnabled,
		DefaultEpochIdentifier,
		DefaultAuthorizedChannel,
		clienttypes.MustParseHeight(DefaultPacketTimeoutHeight),
		DefaultPacketTimeoutTimestamp,
	)
}

// ParamSetPairs get the params.ParamSet
func (p *Params) ParamSetPairs() paramtypes.ParamSetPairs {
	return paramtypes.ParamSetPairs{
		paramtypes.NewParamSetPair(KeyEnabled, &p.Enabled, validateEnabled),
		paramtypes.NewParamSetPair(KeyEpochIdentifier, &p.EpochIdentifier, epochtypes.ValidateEpochIdentifierInterface),
		paramtypes.NewParamSetPair(KeyAuthorizedChannel, &p.AuthorizedChannel, validateAuthorizedChannel),
		paramtypes.NewParamSetPair(KeyPacketTimeoutHeight, &p.PacketTimeoutHeight, validateClientHeight),
		paramtypes.NewParamSetPair(KeyPacketTimeoutTimestamp, &p.PacketTimeoutTimestamp, validateDuration),
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

	return nil
}

// String implements the Stringer interface.
func (p Params) String() string {
	out, _ := yaml.Marshal(p)
	return string(out)
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
