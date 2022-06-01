package types

import (
	fmt "fmt"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// IBC channel sentinel errors
var (
	ErrAcknowledgementHookFailed = sdkerrors.Register(ModuleName, 2, "acknowledgement hook failed")
	ErrTimeoutHookFailed         = sdkerrors.Register(ModuleName, 3, "timeout hook failed")
)

func NewErrAcknowledgementHookFailed(msg sdk.Msg) error {
	return sdkerrors.Wrap(ErrAcknowledgementHookFailed, fmt.Sprintf("handling msg %s", sdk.MsgTypeURL(msg)))
}

func NewErrTimeoutHookFailed(msg sdk.Msg) error {
	return sdkerrors.Wrap(ErrTimeoutHookFailed, fmt.Sprintf("handling msg %s", sdk.MsgTypeURL(msg)))
}
