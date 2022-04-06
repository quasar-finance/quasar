package types

import (
	balancer "github.com/abag/quasarnode/x/gamm/pool-models/balancer"
	types "github.com/abag/quasarnode/x/gamm/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgSendIbcCreatePool = "send_ibc_create_pool"

var _ sdk.Msg = &MsgSendIbcCreatePool{}

func NewMsgSendIbcCreatePool(
	creator string,
	port string,
	channelID string,
	timeoutTimestamp uint64,
	poolParams *balancer.BalancerPoolParams,
	poolAssets []types.PoolAsset,
	futurePoolGovernor string,
) *MsgSendIbcCreatePool {
	return &MsgSendIbcCreatePool{
		Creator:            creator,
		Port:               port,
		ChannelID:          channelID,
		TimeoutTimestamp:   timeoutTimestamp,
		PoolParams:         poolParams,
		PoolAssets:         poolAssets,
		FuturePoolGovernor: futurePoolGovernor,
	}
}

func (msg *MsgSendIbcCreatePool) Route() string {
	return RouterKey
}

func (msg *MsgSendIbcCreatePool) Type() string {
	return TypeMsgSendIbcCreatePool
}

func (msg *MsgSendIbcCreatePool) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgSendIbcCreatePool) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgSendIbcCreatePool) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	if msg.Port == "" {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "invalid packet port")
	}
	if msg.ChannelID == "" {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "invalid packet channel")
	}
	if msg.TimeoutTimestamp == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "invalid packet timeout")
	}
	return nil
}
