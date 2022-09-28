// This file contains dummy implementation of ValidateBasic and GetSigners method for Msg types
// so that they implement sdk.Msg interface.
package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (msg MsgLockTokens) ValidateBasic() error {
	panic("not implemented")
}

func (msg MsgLockTokens) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}

func (msg MsgBeginUnlocking) ValidateBasic() error {
	panic("not implemented")
}

func (msg MsgBeginUnlocking) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}
