// This file contains dummy implementation of ValidateBasic and GetSigners method for Msg types
// so that they implement sdk.Msg interface.
package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

var (
	_ sdk.Msg = &MsgSwapExactAmountIn{}
	_ sdk.Msg = &MsgSwapExactAmountOut{}
	_ sdk.Msg = &MsgJoinPool{}
	_ sdk.Msg = &MsgExitPool{}
	_ sdk.Msg = &MsgJoinSwapExternAmountIn{}
	_ sdk.Msg = &MsgExitSwapExternAmountOut{}
	_ sdk.Msg = &MsgJoinSwapShareAmountOut{}
	_ sdk.Msg = &MsgExitSwapShareAmountIn{}
)

func (MsgSwapExactAmountIn) ValidateBasic() error {
	panic("not implemented")
}

func (MsgSwapExactAmountIn) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}

func (MsgSwapExactAmountOut) ValidateBasic() error {
	panic("not implemented")
}

func (MsgSwapExactAmountOut) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}

func (MsgJoinPool) ValidateBasic() error {
	panic("not implemented")
}

func (MsgJoinPool) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}

func (MsgExitPool) ValidateBasic() error {
	panic("not implemented")
}

func (MsgExitPool) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}

func (MsgJoinSwapExternAmountIn) ValidateBasic() error {
	panic("not implemented")
}

func (MsgJoinSwapExternAmountIn) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}

func (MsgJoinSwapShareAmountOut) ValidateBasic() error {
	panic("not implemented")
}

func (MsgJoinSwapShareAmountOut) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}

func (MsgExitSwapExternAmountOut) ValidateBasic() error {
	panic("not implemented")
}

func (MsgExitSwapExternAmountOut) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}

func (MsgExitSwapShareAmountIn) ValidateBasic() error {
	panic("not implemented")
}

func (MsgExitSwapShareAmountIn) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}
