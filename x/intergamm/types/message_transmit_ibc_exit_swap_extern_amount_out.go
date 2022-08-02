package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgTransmitIbcExitSwapExternAmountOut = "transmit_ibc_exit_swap_extern_amount_ou"

var _ sdk.Msg = &MsgTransmitIbcExitSwapExternAmountOut{}

func NewMsgTransmitIbcExitSwapExternAmountOut(creator string, connectionId string, timoutTimestamp uint64, poolId uint64, shareInAmount int64, tokenOutMins sdk.Coin) *MsgTransmitIbcExitSwapExternAmountOut {
	return &MsgTransmitIbcExitSwapExternAmountOut{
		Creator:          creator,
		ConnectionId:     connectionId,
		TimeoutTimestamp: timoutTimestamp,
		PoolId:           poolId,
		ShareInAmount:    shareInAmount,
		TokenOutMins:     tokenOutMins,
	}
}

func (msg *MsgTransmitIbcExitSwapExternAmountOut) Route() string {
	return RouterKey
}

func (msg *MsgTransmitIbcExitSwapExternAmountOut) Type() string {
	return TypeMsgTransmitIbcExitSwapExternAmountOut
}

func (msg *MsgTransmitIbcExitSwapExternAmountOut) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgTransmitIbcExitSwapExternAmountOut) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgTransmitIbcExitSwapExternAmountOut) ValidateBasic() error {
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
	if msg.TokenOutMins.Amount == sdk.ZeroInt() || msg.TokenOutMins.Denom == "" {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidCoins, "tokenIn cannot have nil field ()")
	}
	if msg.TimeoutTimestamp == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "TimeoutTimestamp cannot be 0")
	}
	return nil
}
