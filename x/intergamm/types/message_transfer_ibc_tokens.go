package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	ibcclienttypes "github.com/cosmos/ibc-go/v3/modules/core/02-client/types"
)

const TypeMsgTransferIbcTokens = "transfer_ibc_tokens"

var _ sdk.Msg = &MsgTransferIbcTokens{}

func NewMsgTransferIbcTokens(
	creator string,
	sourcePort string,
	sourceChannel string,
	token sdk.Coin,
	receiver string,
	timeoutHeight ibcclienttypes.Height,
	timeoutTimestamp uint64,
) *MsgTransferIbcTokens {
	return &MsgTransferIbcTokens{
		Creator:          creator,
		SourcePort:       sourcePort,
		SourceChannel:    sourceChannel,
		Token:            token,
		Receiver:         receiver,
		TimeoutHeight:    timeoutHeight,
		TimeoutTimestamp: timeoutTimestamp,
	}
}

func (msg *MsgTransferIbcTokens) Route() string {
	return RouterKey
}

func (msg *MsgTransferIbcTokens) Type() string {
	return TypeMsgTransferIbcTokens
}

func (msg *MsgTransferIbcTokens) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgTransferIbcTokens) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgTransferIbcTokens) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}
