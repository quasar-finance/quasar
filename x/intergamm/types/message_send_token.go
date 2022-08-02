package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgSendToken = "send_token"

var _ sdk.Msg = &MsgSendToken{}

func NewMsgSendToken(destinationLocalZoneId string, connectionId string, timoutTimestamp, id uint64, coins []sdk.Coin) *MsgSendToken {
	return &MsgSendToken{
		DestinationLocalZoneId: destinationLocalZoneId,
	}
}

func (msg *MsgSendToken) Route() string {
	return RouterKey
}

func (msg *MsgSendToken) Type() string {
	return TypeMsgTransmitIbcJoinPool
}

func (msg *MsgSendToken) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgSendToken) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgSendToken) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	if msg.DestinationLocalZoneId == "" {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "destinationLocalZoneId cannot be empty")
	}
	if msg.Coin == nil {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidCoins, "coins cannot be nil")
	}
	if msg.Sender == "" {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "sender cannot be empty")
	}
	if msg.Receiver == "" {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "receiver cannot be empty")
	}
	return nil
}
