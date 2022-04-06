package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgSendIbcWithdraw = "send_ibc_withdraw"

var _ sdk.Msg = &MsgSendIbcWithdraw{}

func NewMsgSendIbcWithdraw(
	creator string,
	port string,
	channelID string,
	timeoutTimestamp uint64,
	transferPort string,
	transferChannel string,
	receiver string,
	assets []sdk.Coin,
) *MsgSendIbcWithdraw {
	return &MsgSendIbcWithdraw{
		Creator:          creator,
		Port:             port,
		ChannelID:        channelID,
		TimeoutTimestamp: timeoutTimestamp,
		TransferPort:     transferPort,
		TransferChannel:  transferChannel,
		Receiver:         receiver,
		Assets:           assets,
	}
}

func (msg *MsgSendIbcWithdraw) Route() string {
	return RouterKey
}

func (msg *MsgSendIbcWithdraw) Type() string {
	return TypeMsgSendIbcWithdraw
}

func (msg *MsgSendIbcWithdraw) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgSendIbcWithdraw) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgSendIbcWithdraw) ValidateBasic() error {
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
