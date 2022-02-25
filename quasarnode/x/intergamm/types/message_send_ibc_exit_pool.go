package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgSendIbcExitPool = "send_ibc_exit_pool"

var _ sdk.Msg = &MsgSendIbcExitPool{}

func NewMsgSendIbcExitPool(
	creator string,
	port string,
	channelID string,
	timeoutTimestamp uint64,
	poolId uint64,
	shareInAmount sdk.Int,
	tokenOutMins []sdk.Coin) *MsgSendIbcExitPool {
	return &MsgSendIbcExitPool{
		Creator:          creator,
		Port:             port,
		ChannelID:        channelID,
		TimeoutTimestamp: timeoutTimestamp,
		PoolId:           poolId,
		ShareInAmount:    shareInAmount,
		TokenOutMins:     tokenOutMins,
	}
}

func (msg *MsgSendIbcExitPool) Route() string {
	return RouterKey
}

func (msg *MsgSendIbcExitPool) Type() string {
	return TypeMsgSendIbcExitPool
}

func (msg *MsgSendIbcExitPool) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgSendIbcExitPool) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgSendIbcExitPool) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	if msg.Port == "" {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "invalid packet port")
	}
	if msg.ChannelID == "" {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "invalid packet channel")
	}
	if msg.TimeoutTimestamp == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "invalid packet timeout")
	}
	return nil
}
