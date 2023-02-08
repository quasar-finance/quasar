package types

import (
	sdkerrors "cosmossdk.io/errors"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

const TypeMsgTransmitICATransfer = "transmit_ica_transfer"

var _ sdk.Msg = &MsgTransmitICATransfer{}

func NewMsgTransmitICATransfer(icaOwnerAddress string, toAddress string, coin sdk.Coin) *MsgTransmitICATransfer {
	return &MsgTransmitICATransfer{
		IcaOwnerAddress: icaOwnerAddress,
		ToAddress:       toAddress,
		Coin:            coin,
	}
}

func (msg *MsgTransmitICATransfer) Route() string {
	return RouterKey
}

func (msg *MsgTransmitICATransfer) Type() string {
	return TypeMsgTransmitICATransfer
}

func (msg *MsgTransmitICATransfer) GetSigners() []sdk.AccAddress {
	icaOwnerAddress, err := sdk.AccAddressFromBech32(msg.IcaOwnerAddress)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{icaOwnerAddress}
}

func (msg *MsgTransmitICATransfer) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgTransmitICATransfer) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.IcaOwnerAddress)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid icaOwnerAddress address (%s)", err)
	}
	if msg.ToAddress == "" {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidAddress, "toAddress cannot be empty")
	}
	if !msg.Coin.IsValid() {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidCoins, "coin (%s) must be valid", msg.Coin.String())
	}
	if !msg.Coin.IsPositive() {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidCoins, "coin (%s) must be positive", msg.Coin.String())
	}
	return nil
}
