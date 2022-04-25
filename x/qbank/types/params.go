package types

import (
	fmt "fmt"
	"strings"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	paramtypes "github.com/cosmos/cosmos-sdk/x/params/types"
	ibctransfertypes "github.com/cosmos/ibc-go/v3/modules/apps/transfer/types"

	"gopkg.in/yaml.v2"
)

var _ paramtypes.ParamSet = (*Params)(nil)
var (
	KeyMinOrionEpochDenomDollarDeposit = []byte("MinOrionEpochDenomDollarDeposit")
	KeyWhiteListedDenomsInOrion        = []byte("WhiteListedDenomsInOrion")

	DefaultMinOrionEpochDenomDollarDeposit sdk.Dec = sdk.NewDecWithPrec(100, 0) // 100.0 Dollar

	// AUDIT NOTE - Below commented value are used for local testing -with different values of ibc hexh hash.
	// And should be uncommented in the final production code.
	denom1 WhiteListedDenomInOrion = WhiteListedDenomInOrion{
		OriginName:   "uatom",
		OnehopQuasar: "ibc/BE1BB42D4BE3C30D50B68D7C41DB4DFCE9678E8EF8C539F6E6A9345048894FCC",
		OnehopOsmo:   "ibc/BE1BB42D4BE3C30D50B68D7C41DB4DFCE9678E8EF8C539F6E6A9345048894FCC",
	}
	//denom2 WhiteListedDenomInOrion = WhiteListedDenomInOrion{OriginName: "uosmo", OnehopQuasar: "IBC/TESTQSRATOM", OnehopOsmo: "IBC/TESTOSMOOSMO"}
	//DefaultWhiteListedDenomsInOrion = []WhiteListedDenomInOrion{denom1, denom2}
	DefaultWhiteListedDenomsInOrion = []WhiteListedDenomInOrion{denom1}
)

// ParamKeyTable the param key table for launch module
func ParamKeyTable() paramtypes.KeyTable {
	return paramtypes.NewKeyTable().RegisterParamSet(&Params{})
}

// NewParams creates a new Params instance
func NewParams(minOrionEpochDenomDollarDeposit sdk.Dec,
	whiteListedDenomsInOrion []WhiteListedDenomInOrion) Params {
	return Params{
		MinOrionEpochDenomDollarDeposit: minOrionEpochDenomDollarDeposit,
		WhiteListedDenomsInOrion:        whiteListedDenomsInOrion}
}

// DefaultParams returns a default set of parameters
func DefaultParams() Params {
	return NewParams(DefaultMinOrionEpochDenomDollarDeposit, DefaultWhiteListedDenomsInOrion)
}

// ParamSetPairs get the params.ParamSet
func (p *Params) ParamSetPairs() paramtypes.ParamSetPairs {
	return paramtypes.ParamSetPairs{
		paramtypes.NewParamSetPair(KeyMinOrionEpochDenomDollarDeposit, &p.MinOrionEpochDenomDollarDeposit, validateMinOrionEpochDenomDollarDeposit),
		paramtypes.NewParamSetPair(KeyWhiteListedDenomsInOrion, &p.WhiteListedDenomsInOrion, validateWhiteListedDenomsInOrion),
	}
}

// Validate validates the set of params
func (p Params) Validate() error {

	if err := validateMinOrionEpochDenomDollarDeposit(p.MinOrionEpochDenomDollarDeposit); err != nil {
		return err
	}

	if err := validateWhiteListedDenomsInOrion(p.WhiteListedDenomsInOrion); err != nil {
		return err
	}
	return nil
}

// String implements the Stringer interface.
func (p Params) String() string {
	out, _ := yaml.Marshal(p)
	return string(out)
}

// validateMinOrionEpochDenomDollarDeposit validates the MinOrionEpochDenomDollarDeposit param
func validateMinOrionEpochDenomDollarDeposit(i interface{}) error {
	v, ok := i.(sdk.Dec)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", i)
	}

	if v.IsNil() {
		return fmt.Errorf("minOrionEpochDenomDollarDeposit must be not nil")
	}
	if v.IsNegative() {
		return fmt.Errorf("minOrionEpochDenomDollarDeposit must be positive: %s", v)
	}

	return nil
}

// validateWhiteListedDenomsInOrion validates the WhiteListedDenomsInOrion param
func validateWhiteListedDenomsInOrion(v interface{}) error {

	WhiteListedDenomsInOrion, ok := v.([]WhiteListedDenomInOrion)
	if !ok {
		return fmt.Errorf("invalid parameter type: %T", v)
	}
	var is_invalid_denom bool
	for _, v := range WhiteListedDenomsInOrion {
		ibcPrefix := ibctransfertypes.DenomPrefix + "/"
		if strings.HasPrefix(v.OnehopQuasar, ibcPrefix) {
			hexHash := v.OnehopQuasar[len(ibcPrefix):]
			_, err := ibctransfertypes.ParseHexHash(hexHash)
			if err != nil {
				is_invalid_denom = true
			}
		} else {
			is_invalid_denom = true
		}

		if strings.HasPrefix(v.OnehopOsmo, ibcPrefix) {
			hexHash := v.OnehopQuasar[len(ibcPrefix):]
			_, err := ibctransfertypes.ParseHexHash(hexHash)
			if err != nil {
				is_invalid_denom = true
			}
		} else {
			is_invalid_denom = true
		}
	}
	// This failure will indicates to lookinto the qbank params.
	if is_invalid_denom {
		return sdkerrors.Wrap(ibctransfertypes.ErrInvalidDenomForTransfer,
			"incorrect once hop ibc param is set")
	}
	return nil
}
