package types

import (
	fmt "fmt"

	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	"gopkg.in/yaml.v2"
)

var _ paramtypes.ParamSet = (*Params)(nil)

var (
	KeyOsmoTokenTransferChannels        = []byte("OsmoTokenTransferChannels")
	KeyIntrRcvrs                        = []byte("IntrRcvrs")
	KeyDestToIntrZoneMap                = []byte("DestToIntrZoneMap")
	DefaultOsmoTokenTransferChannelsMap = map[string]string{"osmosis-test": "channel-1"}
	DefaultDestToIntrZoneMap            = map[string]string{}
	DefaultIntrRcvrs                    = []IntermediateReceiver{}
)

// ParamKeyTable the param key table for launch module
func ParamKeyTable() paramtypes.KeyTable {
	return paramtypes.NewKeyTable().RegisterParamSet(&Params{})
}

// NewParams creates a new Params instance
func NewParams(m map[string]string,
	destIntrZone map[string]string,
	intrRcvrs []IntermediateReceiver) Params {
	return Params{OsmoTokenTransferChannels: m,
		DestToIntrZoneMap: destIntrZone,
		IntrRcvrs:         intrRcvrs}
}

// DefaultParams returns a default set of parameters
func DefaultParams() Params {
	return NewParams(DefaultOsmoTokenTransferChannelsMap,
		DefaultDestToIntrZoneMap,
		DefaultIntrRcvrs)
}

// ParamSetPairs get the params.ParamSet
func (p *Params) ParamSetPairs() paramtypes.ParamSetPairs {
	return paramtypes.ParamSetPairs{
		paramtypes.NewParamSetPair(KeyOsmoTokenTransferChannels, &p.OsmoTokenTransferChannels, validateOsmoTokenTransferChannels),
		paramtypes.NewParamSetPair(KeyDestToIntrZoneMap, &p.DestToIntrZoneMap, validateDestToIntrZoneMap),
		paramtypes.NewParamSetPair(KeyIntrRcvrs, &p.IntrRcvrs, validateIntermediateReceivers),
	}
}

// Validate validates the set of params
func (p Params) Validate() error {
	if err := validateOsmoTokenTransferChannels(p.OsmoTokenTransferChannels); err != nil {
		return err
	}
	if err := validateIntermediateReceivers(p.IntrRcvrs); err != nil {
		return err
	}
	if err := validateDestToIntrZoneMap(p.DestToIntrZoneMap); err != nil {
		return err
	}
	return nil
}

// String implements the Stringer interface.
func (p Params) String() string {
	out, _ := yaml.Marshal(p)
	return string(out)
}

func validateOsmoTokenTransferChannels(i interface{}) error {
	_, ok := i.(map[string]string)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", i)
	}

	return nil
}

func validateIntermediateReceivers(i interface{}) error {
	_, ok := i.([]IntermediateReceiver)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", i)
	}

	return nil
}

func validateDestToIntrZoneMap(i interface{}) error {
	_, ok := i.(map[string]string)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", i)
	}

	return nil
}
