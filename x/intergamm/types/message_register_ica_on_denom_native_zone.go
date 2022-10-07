package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgRegisterICAOnDenomNativeZone = "register_ica_on_denom_native_zone"

var _ sdk.Msg = &MsgRegisterICAOnDenomNativeZone{}

func NewMsgRegisterICAOnDenomNativeZone(ownerAddress string, denom string) *MsgRegisterICAOnDenomNativeZone {
	return &MsgRegisterICAOnDenomNativeZone{
		OwnerAddress: ownerAddress,
		Denom:        denom,
	}
}

func (msg *MsgRegisterICAOnDenomNativeZone) Route() string {
	return RouterKey
}

func (msg *MsgRegisterICAOnDenomNativeZone) Type() string {
	return TypeMsgRegisterICAOnDenomNativeZone
}

func (msg *MsgRegisterICAOnDenomNativeZone) GetSigners() []sdk.AccAddress {
	ownerAddress, err := sdk.AccAddressFromBech32(msg.OwnerAddress)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{ownerAddress}
}

func (msg *MsgRegisterICAOnDenomNativeZone) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgRegisterICAOnDenomNativeZone) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.OwnerAddress)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid ownerAddress address (%s)", err)
	}
	if sdk.ValidateDenom(msg.Denom) != nil {
		return sdkerrors.Wrapf(ErrInvalidDenom, "invalid denom (%s)", msg.Denom)
	}
	return nil
}
