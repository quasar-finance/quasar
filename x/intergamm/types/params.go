package types

import (
	"errors"
	"fmt"

	sdktypes "github.com/cosmos/cosmos-sdk/types"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	"gopkg.in/yaml.v2"
)

var _ paramtypes.ParamSet = (*Params)(nil)

var (
	KeyCompleteZoneInfoMap        = []byte("CompleteZoneInfoMap")
	KeyDenomToNativeZoneIdMap     = []byte("DenomToNativeIdMap")
	DefaultDenomToNativeZoneIdMap = map[string]string{}
	DefaultCompleteZoneInfoMap    = map[string]ZoneCompleteInfo{}
)

// ParamKeyTable the param key table for launch module
func ParamKeyTable() paramtypes.KeyTable {
	return paramtypes.NewKeyTable().RegisterParamSet(&Params{})
}

// NewParams creates a new Params instance
func NewParams(denomToNativeZoneIdMap map[string]string,
	completeZoneInfoMap map[string]ZoneCompleteInfo) Params {
	return Params{
		DenomToNativeZoneIdMap: denomToNativeZoneIdMap,
		CompleteZoneInfoMap:    completeZoneInfoMap,
	}
}

// DefaultParams returns a default set of parameters
func DefaultParams() Params {
	return NewParams(DefaultDenomToNativeZoneIdMap,
		DefaultCompleteZoneInfoMap)
}

// ParamSetPairs get the params.ParamSet
func (p *Params) ParamSetPairs() paramtypes.ParamSetPairs {
	return paramtypes.ParamSetPairs{
		paramtypes.NewParamSetPair(KeyDenomToNativeZoneIdMap, &p.DenomToNativeZoneIdMap, validateDenomToNativeZoneIdMap),
		paramtypes.NewParamSetPair(KeyCompleteZoneInfoMap, &p.CompleteZoneInfoMap, validateCompleteZoneInfoMap),
	}
}

// Validate validates the set of params
func (p Params) Validate() error {
	for denom, nativeZoneId := range p.DenomToNativeZoneIdMap {
		if err := sdktypes.ValidateDenom(denom); err != nil {
			return err
		}
		if err := validateIdentifier(nativeZoneId); err != nil {
			return err
		}
	}
	for zoneId, completeZoneInfo := range p.CompleteZoneInfoMap {
		if err := validateIdentifier(zoneId); err != nil {
			return err
		}
		if err := completeZoneInfo.validateCompleteZoneInfo(); err != nil {
			return err
		}
	}
	return nil
}

// String implements the Stringer interface.
func (p Params) String() string {
	out, _ := yaml.Marshal(p)
	return string(out)
}

func validateIdentifier(id string) error {
	if id == "" {
		return errors.New("error: ID can not be empty")
	}
	return nil
}

func (info ZoneRouteInfo) validateZoneRouteInfo() error {
	if err := validateIdentifier(info.ZoneId); err != nil {
		return err
	}
	if err := validateIdentifier(info.ChainId); err != nil {
		return err
	}
	if err := validateIdentifier(info.CounterpartyZoneId); err != nil {
		return err
	}
	if err := validateIdentifier(info.CounterpartyChainId); err != nil {
		return err
	}
	if err := validateIdentifier(info.ConnectionId); err != nil {
		return err
	}
	if err := validateIdentifier(info.PortId); err != nil {
		return err
	}
	if err := validateIdentifier(info.ChannelId); err != nil {
		return err
	}
	if err := validateIdentifier(info.CounterpartyConnectionId); err != nil {
		return err
	}
	if err := validateIdentifier(info.CounterpartyPortId); err != nil {
		return err
	}
	if err := validateIdentifier(info.CounterpartyChannelId); err != nil {
		return err
	}
	return nil
}

func (info ZoneCompleteInfo) validateCompleteZoneInfo() error {
	if err := info.ZoneRouteInfo.validateZoneRouteInfo(); err != nil {
		return err
	}
	for zoneId, zoneRouteInfo := range info.NextZoneRouteMap {
		if err := validateIdentifier(zoneId); err != nil {
			return err
		}
		if err := zoneRouteInfo.validateZoneRouteInfo(); err != nil {
			return err
		}
		if zoneId != zoneRouteInfo.CounterpartyZoneId {
			return errors.New("error: counterparty zone ID of ZoneRouteInfo does not match the map key")
		}
		if QuasarZoneId != zoneRouteInfo.ZoneId {
			return errors.New("error: zone ID of ZoneRouteInfo does not match 'quasar'")
		}
	}
	return nil
}

func validateDenomToNativeZoneIdMap(i interface{}) error {
	if m, ok := i.(map[string]string); !ok {
		return fmt.Errorf("invalid parameter type: %T", i)
	} else {
		for denom, nativeZoneId := range m {
			if err := sdktypes.ValidateDenom(denom); err != nil {
				return err
			}
			if err := validateIdentifier(nativeZoneId); err != nil {
				return err
			}
		}
	}
	return nil
}

func validateCompleteZoneInfoMap(i interface{}) error {
	if m, ok := i.(map[string]ZoneCompleteInfo); !ok {
		return fmt.Errorf("invalid parameter type: %T", i)
	} else {
		for zoneId, completeZoneInfo := range m {
			if err := validateIdentifier(zoneId); err != nil {
				return err
			}
			if err := completeZoneInfo.validateCompleteZoneInfo(); err != nil {
				return err
			}
		}
	}
	return nil
}
