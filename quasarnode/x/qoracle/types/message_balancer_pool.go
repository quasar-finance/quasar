package types

import (
	gammbalancertypes "github.com/abag/quasarnode/x/gamm/pool-models/balancer"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgBalancerPool = "balancer_pool"

var _ sdk.Msg = &MsgBalancerPool{}

/*
func NewMsgBalancerPool(creator string, address string, uid uint64) *MsgBalancerPool {
	return &MsgBalancerPool{
		Creator: creator,
		Address: address,
		Uid:     uid,
	}
}
*/

// TODO - Verify if you need to take care of pointers of deep copy due to the fact that
// bp argument is of pointer type.
func NewMsgBalancerPool(creator string, bp *gammbalancertypes.BalancerPool) *MsgBalancerPool {
	return &MsgBalancerPool{
		Creator:      creator,
		BalancerPool: bp,
	}
}
func (msg *MsgBalancerPool) Route() string {
	return RouterKey
}

func (msg *MsgBalancerPool) Type() string {
	return TypeMsgBalancerPool
}

func (msg *MsgBalancerPool) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgBalancerPool) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

// TODO - Add verification of each field
func (msg *MsgBalancerPool) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}
