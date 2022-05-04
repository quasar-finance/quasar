package types

import (
	gammpooltypes "github.com/abag/quasarnode/x/gamm/pool-models/balancer"
	gammtypes "github.com/abag/quasarnode/x/gamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgCreatePool = "create_pool"

var _ sdk.Msg = &MsgCreatePool{}

func NewMsgCreatePool(
	creator string, connectionId string, timeoutTimestamp uint64,
	poolParams *gammpooltypes.BalancerPoolParams,
	poolAssets []gammtypes.PoolAsset,
	futurePoolGovernor string,
) *MsgCreatePool {
	return &MsgCreatePool{
		Creator:            creator,
		ConnectionId:       connectionId,
		TimeoutTimestamp:   timeoutTimestamp,
		PoolParams:         poolParams,
		PoolAssets:         poolAssets,
		FuturePoolGovernor: futurePoolGovernor,
	}
}

func (msg *MsgCreatePool) Route() string {
	return RouterKey
}

func (msg *MsgCreatePool) Type() string {
	return TypeMsgCreatePool
}

func (msg *MsgCreatePool) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgCreatePool) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgCreatePool) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}
