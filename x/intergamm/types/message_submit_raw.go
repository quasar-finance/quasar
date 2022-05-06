package types

import (
	fmt "fmt"

	codectypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	proto "github.com/gogo/protobuf/proto"
)

const TypeMsgSubmitTx = "submit_tx"

var (
	_ sdk.Msg                            = &MsgSubmitTx{}
	_ codectypes.UnpackInterfacesMessage = &MsgSubmitTx{}
)

// NewMsgSubmitTx creates and returns a new MsgSubmitTx instance
func NewMsgSubmitTx(sdkMsg sdk.Msg, connectionID, owner string) (*MsgSubmitTx, error) {
	any, err := PackTxMsgAny(sdkMsg)
	if err != nil {
		return nil, err
	}

	return &MsgSubmitTx{
		ConnectionId: connectionID,
		Owner:        owner,
		Msg:          any,
	}, nil
}

// PackTxMsgAny marshals the sdk.Msg payload to a protobuf Any type
func PackTxMsgAny(sdkMsg sdk.Msg) (*codectypes.Any, error) {
	msg, ok := sdkMsg.(proto.Message)
	if !ok {
		return nil, fmt.Errorf("can't proto marshal %T", sdkMsg)
	}

	any, err := codectypes.NewAnyWithValue(msg)
	if err != nil {
		return nil, err
	}

	return any, nil
}

// UnpackInterfaces implements codectypes.UnpackInterfacesMessage
func (msg *MsgSubmitTx) UnpackInterfaces(unpacker codectypes.AnyUnpacker) error {
	var (
		sdkMsg sdk.Msg
	)

	return unpacker.UnpackAny(msg.Msg, &sdkMsg)
}

// GetTxMsg fetches the cached any message
func (msg *MsgSubmitTx) GetTxMsg() sdk.Msg {
	sdkMsg, ok := msg.Msg.GetCachedValue().(sdk.Msg)
	if !ok {
		return nil
	}

	return sdkMsg
}

func (msg *MsgSubmitTx) Route() string {
	return RouterKey
}

func (msg *MsgSubmitTx) Type() string {
	return TypeMsgSubmitTx
}

// GetSigners implements sdk.Msg
func (msg *MsgSubmitTx) GetSigners() []sdk.AccAddress {
	accAddr, err := sdk.AccAddressFromBech32(msg.Owner)
	if err != nil {
		panic(err)
	}

	return []sdk.AccAddress{accAddr}
}

// ValidateBasic implements sdk.Msg
func (msg *MsgSubmitTx) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Owner)
	if err != nil {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidAddress, "invalid owner address")
	}

	return nil
}
