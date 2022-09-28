// This file contains dummy implementation of ValidateBasic and GetSigners method for Msg types
// so that they implement sdk.Msg interface.
package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (msg MsgSwapExactAmountIn) ValidateBasic() error {
	panic("not implemented")
}

func (msg MsgSwapExactAmountIn) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}

func (msg MsgSwapExactAmountOut) ValidateBasic() error {
	panic("not implemented")
}

func (msg MsgSwapExactAmountOut) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}

func (msg MsgJoinPool) ValidateBasic() error {
	panic("not implemented")
}

func (msg MsgJoinPool) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}

func (msg MsgExitPool) ValidateBasic() error {
	panic("not implemented")
}

func (msg MsgExitPool) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}

func (msg MsgJoinSwapExternAmountIn) ValidateBasic() error {
	panic("not implemented")
}

func (msg MsgJoinSwapExternAmountIn) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}

func (msg MsgJoinSwapShareAmountOut) ValidateBasic() error {
	panic("not implemented")
}

func (msg MsgJoinSwapShareAmountOut) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}

func (msg MsgExitSwapExternAmountOut) ValidateBasic() error {
	panic("not implemented")
}

func (msg MsgExitSwapExternAmountOut) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}

func (msg MsgExitSwapShareAmountIn) ValidateBasic() error {
	panic("not implemented")
}

func (msg MsgExitSwapShareAmountIn) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}
