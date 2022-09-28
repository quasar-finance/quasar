// This file contains dummy implementation of ValidateBasic and GetSigners method for Msg types
// so that they implement sdk.Msg interface.
package balancer

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (msg MsgCreateBalancerPool) ValidateBasic() error {
	panic("not implemented")
}

func (msg MsgCreateBalancerPool) GetSigners() []sdk.AccAddress {
	panic("not implemented")
}
