package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"time"
)

const TypeMsgTransmitIbcLockTokens = "transmit_ibc_lock_tokens"

var _ sdk.Msg = &MsgTransmitIbcLockTokens{}

func NewMsgTransmitIbcLockTokens(creator string, connectionId string, timoutTimestamp uint64, duration time.Duration, coins []sdk.Coin) *MsgTransmitIbcLockTokens {
	return &MsgTransmitIbcLockTokens{
		Creator:          creator,
		ConnectionId:     connectionId,
		TimeoutTimestamp: timoutTimestamp,
		Duration:         duration,
		Coins:            coins,
	}
}

func (msg *MsgTransmitIbcLockTokens) Route() string {
	return RouterKey
}

func (msg *MsgTransmitIbcLockTokens) Type() string {
	return TypeMsgTransmitIbcLockTokens
}

func (msg *MsgTransmitIbcLockTokens) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgTransmitIbcLockTokens) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgTransmitIbcLockTokens) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	if msg.ConnectionId == "" {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidAddress, "connectionID cannot be empty")
	}
	if msg.ConnectionId == "" {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidAddress, "connectionID cannot be empty")
	}
	if msg.Duration == time.Duration(0) {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "duration cannot be 0")
	}
	if len(msg.Coins) == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidCoins, "coins cannot be empty")
	}
	if msg.TimeoutTimestamp == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "invalid TimeoutTimestamp, cannot be 0")
	}
	return nil
}
