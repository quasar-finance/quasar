package types

import (
	"fmt"
	"strings"
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

// constants
const (
	TypeMsgSwapExactAmountIn       = "swap_exact_amount_in"
	TypeMsgSwapExactAmountOut      = "swap_exact_amount_out"
	TypeMsgJoinPool                = "join_pool"
	TypeMsgExitPool                = "exit_pool"
	TypeMsgJoinSwapExternAmountIn  = "join_swap_extern_amount_in"
	TypeMsgJoinSwapShareAmountOut  = "join_swap_share_amount_out"
	TypeMsgExitSwapExternAmountOut = "exit_swap_extern_amount_out"
	TypeMsgExitSwapShareAmountIn   = "exit_swap_share_amount_in"
)

func ValidateFutureGovernor(governor string) error {
	// allow empty governor
	if governor == "" {
		return nil
	}

	// validation for future owner
	// "osmo1fqlr98d45v5ysqgp6h56kpujcj4cvsjnjq9nck"
	_, err := sdk.AccAddressFromBech32(governor)
	if err == nil {
		return nil
	}

	lockTimeStr := ""
	splits := strings.Split(governor, ",")
	if len(splits) > 2 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidAddress, fmt.Sprintf("invalid future governor: %s", governor))
	}

	// token,100h
	if len(splits) == 2 {
		lpTokenStr := splits[0]
		if sdk.ValidateDenom(lpTokenStr) != nil {
			return sdkerrors.Wrap(sdkerrors.ErrInvalidAddress, fmt.Sprintf("invalid future governor: %s", governor))
		}
		lockTimeStr = splits[1]
	}

	// 100h
	if len(splits) == 1 {
		lockTimeStr = splits[0]
	}

	// Note that a duration of 0 is allowed
	_, err = time.ParseDuration(lockTimeStr)
	if err != nil {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidAddress, fmt.Sprintf("invalid future governor: %s", governor))
	}
	return nil
}

var _ sdk.Msg = &MsgSwapExactAmountIn{}

func (msg MsgSwapExactAmountIn) ValidateBasic() error {
	return nil
}

func (msg MsgSwapExactAmountIn) GetSigners() []sdk.AccAddress {
	sender, err := sdk.AccAddressFromBech32(msg.Sender)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{sender}
}

var _ sdk.Msg = &MsgSwapExactAmountOut{}

func (msg MsgSwapExactAmountOut) ValidateBasic() error {
	return nil
}

func (msg MsgSwapExactAmountOut) GetSigners() []sdk.AccAddress {
	sender, err := sdk.AccAddressFromBech32(msg.Sender)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{sender}
}

var _ sdk.Msg = &MsgJoinPool{}

func (msg MsgJoinPool) ValidateBasic() error {
	return nil
}

func (msg MsgJoinPool) GetSigners() []sdk.AccAddress {
	sender, err := sdk.AccAddressFromBech32(msg.Sender)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{sender}
}

var _ sdk.Msg = &MsgExitPool{}

func (msg MsgExitPool) ValidateBasic() error {
	return nil
}

func (msg MsgExitPool) GetSigners() []sdk.AccAddress {
	sender, err := sdk.AccAddressFromBech32(msg.Sender)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{sender}
}

var _ sdk.Msg = &MsgJoinSwapExternAmountIn{}

func (msg MsgJoinSwapExternAmountIn) ValidateBasic() error {
	return nil
}

func (msg MsgJoinSwapExternAmountIn) GetSigners() []sdk.AccAddress {
	sender, err := sdk.AccAddressFromBech32(msg.Sender)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{sender}
}

var _ sdk.Msg = &MsgJoinSwapShareAmountOut{}

func (msg MsgJoinSwapShareAmountOut) ValidateBasic() error {
	return nil
}

func (msg MsgJoinSwapShareAmountOut) GetSigners() []sdk.AccAddress {
	sender, err := sdk.AccAddressFromBech32(msg.Sender)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{sender}
}

var _ sdk.Msg = &MsgExitSwapExternAmountOut{}

func (msg MsgExitSwapExternAmountOut) ValidateBasic() error {
	return nil
}

func (msg MsgExitSwapExternAmountOut) GetSigners() []sdk.AccAddress {
	sender, err := sdk.AccAddressFromBech32(msg.Sender)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{sender}
}

var _ sdk.Msg = &MsgExitSwapShareAmountIn{}

func (msg MsgExitSwapShareAmountIn) ValidateBasic() error {
	return nil
}

func (msg MsgExitSwapShareAmountIn) GetSigners() []sdk.AccAddress {
	sender, err := sdk.AccAddressFromBech32(msg.Sender)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{sender}
}
