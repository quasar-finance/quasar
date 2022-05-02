package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgStablePrice = "stable_price"

var _ sdk.Msg = &MsgStablePrice{}

func NewMsgStablePrice(creator string, denom string, price string) *MsgStablePrice {
	return &MsgStablePrice{
		Creator: creator,
		Denom:   denom,
		Price:   price,
	}
}

func (msg *MsgStablePrice) Route() string {
	return RouterKey
}

func (msg *MsgStablePrice) Type() string {
	return TypeMsgStablePrice
}

func (msg *MsgStablePrice) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgStablePrice) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgStablePrice) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}
