package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	types2 "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
)

const TypeMsgForwardIbcTransfer = "forward_ibc_transfer"

var _ sdk.Msg = &MsgForwardIbcTransfer{}

func NewMsgForwardIbcTransfer(creator string,
	connectionId string,
	timeoutTimestamp uint64,
	transferPort string,
	transferChannel string,
	token sdk.Coin,
	fwdTransferPort,
	fwdTransferChannel string,
	intermediateReceiver string,
	receiver string,
	transferTimeoutHeight types2.Height,
	transferTimeoutTimestamp uint64) *MsgForwardIbcTransfer {
	return &MsgForwardIbcTransfer{
		Creator:                  creator,
		ConnectionId:             connectionId,
		TimeoutTimestamp:         timeoutTimestamp,
		TransferPort:             transferPort,
		TransferChannel:          transferChannel,
		Token:                    token,
		ForwardTransferPort:      fwdTransferPort,
		ForwardTransferChannel:   fwdTransferChannel,
		IntermediateReceiver:     intermediateReceiver,
		Receiver:                 receiver,
		TransferTimeoutHeight:    transferTimeoutHeight,
		TransferTimeoutTimestamp: transferTimeoutTimestamp,
	}
}

func (msg *MsgForwardIbcTransfer) Route() string {
	return RouterKey
}

func (msg *MsgForwardIbcTransfer) Type() string {
	return TypeMsgForwardIbcTransfer
}

func (msg *MsgForwardIbcTransfer) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgForwardIbcTransfer) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgForwardIbcTransfer) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}
