package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	ibcclienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
)

const TypeMsgForwardTransferIbcTokens = "forward_transfer_ibc_tokens"

var _ sdk.Msg = &MsgForwardTransferIbcTokens{}

func NewMsgForwardTransferIbcTokens(
	creator string,
	sourcePort string,
	sourceChannel string,
	token sdk.Coin,
	forwardTransferPort string,
	forwardTransferChannel string,
	intermediateReceiver string,
	receiver string,
	timeoutHeight ibcclienttypes.Height,
	timeoutTimestamp uint64,
) *MsgForwardTransferIbcTokens {
	return &MsgForwardTransferIbcTokens{
		Creator:                creator,
		SourcePort:             sourcePort,
		SourceChannel:          sourceChannel,
		Token:                  token,
		ForwardTransferPort:    forwardTransferPort,
		ForwardTransferChannel: forwardTransferChannel,
		IntermediateReceiver:   intermediateReceiver,
		Receiver:               receiver,
		TimeoutHeight:          timeoutHeight,
		TimeoutTimestamp:       timeoutTimestamp,
	}
}

func (msg *MsgForwardTransferIbcTokens) Route() string {
	return RouterKey
}

func (msg *MsgForwardTransferIbcTokens) Type() string {
	return TypeMsgForwardTransferIbcTokens
}

func (msg *MsgForwardTransferIbcTokens) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgForwardTransferIbcTokens) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgForwardTransferIbcTokens) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}
