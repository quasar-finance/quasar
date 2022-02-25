package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgSendIbcJoinPool = "send_ibc_join_pool"

var _ sdk.Msg = &MsgSendIbcJoinPool{}

func NewMsgSendIbcJoinPool(
	creator string,
	port string,
	channelID string,
	timeoutTimestamp uint64,
	poolId uint64,
	shareOutAmount sdk.Int,
	tokenInMaxs []sdk.Coin) *MsgSendIbcJoinPool {
	return &MsgSendIbcJoinPool{
		Creator:          creator,
		Port:             port,
		ChannelID:        channelID,
		TimeoutTimestamp: timeoutTimestamp,
		PoolId:           poolId,
		ShareOutAmount:   shareOutAmount,
		TokenInMaxs:      tokenInMaxs,
	}
}

func (msg *MsgSendIbcJoinPool) Route() string {
	return RouterKey
}

func (msg *MsgSendIbcJoinPool) Type() string {
	return TypeMsgSendIbcJoinPool
}

func (msg *MsgSendIbcJoinPool) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgSendIbcJoinPool) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgSendIbcJoinPool) ValidateBasic() error {
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
