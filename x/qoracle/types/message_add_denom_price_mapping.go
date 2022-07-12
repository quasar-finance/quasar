package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgAddDenomPriceMapping = "add_denom_price_mapping"

var _ sdk.Msg = &MsgAddDenomPriceMapping{}

func NewMsgAddDenomPriceMapping(creator string) *MsgAddDenomPriceMapping {
	return &MsgAddDenomPriceMapping{
		Creator: creator,
	}
}

func (msg *MsgAddDenomPriceMapping) Route() string {
	return RouterKey
}

func (msg *MsgAddDenomPriceMapping) Type() string {
	return TypeMsgAddDenomPriceMapping
}

func (msg *MsgAddDenomPriceMapping) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgAddDenomPriceMapping) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgAddDenomPriceMapping) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}

	if err := msg.Mapping.ValidateBasic(); err != nil {
		return sdkerrors.Wrap(err, "plan")
	}

	return nil
}
