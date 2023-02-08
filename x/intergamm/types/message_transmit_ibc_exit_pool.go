package types

import (
	sdkerrors "cosmossdk.io/errors"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

const TypeMsgTransmitIbcExitPool = "transmit_ibc_exit_pool"

var _ sdk.Msg = &MsgTransmitIbcExitPool{}

func NewMsgTransmitIbcExitPool(creator string, connectionId string, timoutTimestamp uint64, poolId uint64, shareInAmount int64, tokenOutMins []sdk.Coin) *MsgTransmitIbcExitPool {
	return &MsgTransmitIbcExitPool{
		Creator:          creator,
		ConnectionId:     connectionId,
		TimeoutTimestamp: timoutTimestamp,
		PoolId:           poolId,
		ShareInAmount:    shareInAmount,
		TokenOutMins:     tokenOutMins,
	}
}

func (msg *MsgTransmitIbcExitPool) Route() string {
	return RouterKey
}

func (msg *MsgTransmitIbcExitPool) Type() string {
	return TypeMsgTransmitIbcExitPool
}

func (msg *MsgTransmitIbcExitPool) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgTransmitIbcExitPool) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgTransmitIbcExitPool) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	if msg.ConnectionId == "" {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidAddress, "connectionID cannot be empty")
	}
	if msg.ShareInAmount == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "ShareInAmount cannot be 0")
	}
	if len(msg.TokenOutMins) == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidCoins, "TokenOutMins cannot be empty")
	}
	if msg.TimeoutTimestamp == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "TimeoutTimestamp cannot be 0")
	}
	return nil
}
