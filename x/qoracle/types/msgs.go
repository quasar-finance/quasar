package types

import (
	"cosmossdk.io/errors"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgAddDenomSymbolMappings = "add_denom_symbol_mappings"

var _ sdk.Msg = &MsgAddDenomSymbolMappings{}

func NewMsgAddDenomSymbolMappings(creator string, mappings ...DenomSymbolMapping) *MsgAddDenomSymbolMappings {
	return &MsgAddDenomSymbolMappings{
		Creator:  creator,
		Mappings: mappings,
	}
}

func (msg *MsgAddDenomSymbolMappings) Route() string {
	return RouterKey
}

func (msg *MsgAddDenomSymbolMappings) Type() string {
	return TypeMsgAddDenomSymbolMappings
}

func (msg *MsgAddDenomSymbolMappings) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgAddDenomSymbolMappings) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgAddDenomSymbolMappings) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return errors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}

	for _, mapping := range msg.Mappings {
		if err := mapping.Validate(); err != nil {
			return errors.Wrapf(err, "invalid mapping with denom %s", mapping.Denom)
		}
	}

	return nil
}

const TypeMsgRemoveDenomSymbolMappings = "remove_denom_symbol_mappings"

var _ sdk.Msg = &MsgRemoveDenomSymbolMappings{}

func NewMsgRemoveDenomSymbolMappings(creator string) *MsgRemoveDenomSymbolMappings {
	return &MsgRemoveDenomSymbolMappings{
		Creator: creator,
	}
}

func (msg *MsgRemoveDenomSymbolMappings) Route() string {
	return RouterKey
}

func (msg *MsgRemoveDenomSymbolMappings) Type() string {
	return TypeMsgRemoveDenomSymbolMappings
}

func (msg *MsgRemoveDenomSymbolMappings) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgRemoveDenomSymbolMappings) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgRemoveDenomSymbolMappings) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return errors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}

	for _, denom := range msg.Denoms {
		if err := sdk.ValidateDenom(denom); err != nil {
			return err
		}
	}

	return nil
}
