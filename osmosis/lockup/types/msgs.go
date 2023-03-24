// This file contains dummy implementation of ValidateBasic and GetSigners method for Msg types
// so that they implement sdk.Msg interface.
package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

var (
	_ sdk.Msg = &MsgLockTokens{}
	_ sdk.Msg = &MsgBeginUnlocking{}
)

func (MsgLockTokens) ValidateBasic() error {
	panic("not implemented")
}

func (MsgLockTokens) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}

func (MsgBeginUnlocking) ValidateBasic() error {
	panic("not implemented")
}

func (MsgBeginUnlocking) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}
