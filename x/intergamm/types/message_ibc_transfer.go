package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	types1 "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	types2 "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
)

const TypeMsgIbcTransfer = "ibc_transfer"

var _ sdk.Msg = &MsgIbcTransfer{}

func NewMsgIbcTransfer(creator string,
	connectionId string,
	timeoutTimestamp uint64,
	transferPort string,
	transferChannel string,
	token types1.Coin,
	receiver string,
	transferTimeoutHeight types2.Height,
	transferTimeoutTimestamp uint64) *MsgIbcTransfer {
	return &MsgIbcTransfer{
		Creator:                  creator,
		ConnectionId:             connectionId,
		TimeoutTimestamp:         timeoutTimestamp,
		TransferPort:             transferPort,
		TransferChannel:          transferChannel,
		Token:                    token,
		Receiver:                 receiver,
		TransferTimeoutHeight:    transferTimeoutHeight,
		TransferTimeoutTimestamp: transferTimeoutTimestamp,
	}
}

func (msg *MsgIbcTransfer) Route() string {
	return RouterKey
}

func (msg *MsgIbcTransfer) Type() string {
	return TypeMsgIbcTransfer
}

func (msg *MsgIbcTransfer) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgIbcTransfer) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgIbcTransfer) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}
