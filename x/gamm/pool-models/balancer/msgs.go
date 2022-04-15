package balancer

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

const (
	TypeMsgCreateBalancerPool = "create_balancer_pool"
)

var _ sdk.Msg = &MsgCreateBalancerPool{}

func (msg MsgCreateBalancerPool) ValidateBasic() error {
	// NOTE: this doesn't matter as we only need to send msgs to osmosis
	return nil
}

func (msg MsgCreateBalancerPool) GetSigners() []sdk.AccAddress {
	sender, err := sdk.AccAddressFromBech32(msg.Sender)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{sender}
}
