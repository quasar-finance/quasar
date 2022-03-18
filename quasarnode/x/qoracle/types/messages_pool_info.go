package types

import (
	balancer "github.com/abag/quasarnode/x/gamm/pool-models/balancer"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const (
	TypeMsgCreatePoolInfo = "create_pool_info"
	TypeMsgUpdatePoolInfo = "update_pool_info"
	TypeMsgDeletePoolInfo = "delete_pool_info"
)

var _ sdk.Msg = &MsgCreatePoolInfo{}

func NewMsgCreatePoolInfo(
	creator string,
	poolId string,
	info *balancer.BalancerPool,
	lastUpdatedTime uint64,

) *MsgCreatePoolInfo {
	return &MsgCreatePoolInfo{
		Creator:         creator,
		PoolId:          poolId,
		Info:            info,
		LastUpdatedTime: lastUpdatedTime,
	}
}

func (msg *MsgCreatePoolInfo) Route() string {
	return RouterKey
}

func (msg *MsgCreatePoolInfo) Type() string {
	return TypeMsgCreatePoolInfo
}

func (msg *MsgCreatePoolInfo) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgCreatePoolInfo) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgCreatePoolInfo) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	if len(msg.PoolId) == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "empty PoolId")
	}
	if msg.Info == nil {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "nil Info")
	}
	if err := msg.Info.Validate(); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "invalid Info (%s)", err)
	}
	if msg.LastUpdatedTime == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "LastUpdatedTime is zero")
	}
	return nil
}

var _ sdk.Msg = &MsgUpdatePoolInfo{}

func NewMsgUpdatePoolInfo(
	creator string,
	poolId string,
	info *balancer.BalancerPool,
	lastUpdatedTime uint64,

) *MsgUpdatePoolInfo {
	return &MsgUpdatePoolInfo{
		Creator:         creator,
		PoolId:          poolId,
		Info:            info,
		LastUpdatedTime: lastUpdatedTime,
	}
}

func (msg *MsgUpdatePoolInfo) Route() string {
	return RouterKey
}

func (msg *MsgUpdatePoolInfo) Type() string {
	return TypeMsgUpdatePoolInfo
}

func (msg *MsgUpdatePoolInfo) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgUpdatePoolInfo) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgUpdatePoolInfo) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	if len(msg.PoolId) == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "empty PoolId")
	}
	if msg.Info == nil {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "nil Info")
	}
	if err := msg.Info.Validate(); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "invalid Info (%s)", err)
	}
	if msg.LastUpdatedTime == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "LastUpdatedTime is zero")
	}
	return nil
}

var _ sdk.Msg = &MsgDeletePoolInfo{}

func NewMsgDeletePoolInfo(
	creator string,
	poolId string,

) *MsgDeletePoolInfo {
	return &MsgDeletePoolInfo{
		Creator: creator,
		PoolId:  poolId,
	}
}
func (msg *MsgDeletePoolInfo) Route() string {
	return RouterKey
}

func (msg *MsgDeletePoolInfo) Type() string {
	return TypeMsgDeletePoolInfo
}

func (msg *MsgDeletePoolInfo) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgDeletePoolInfo) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgDeletePoolInfo) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	if len(msg.PoolId) == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "empty PoolId")
	}
	return nil
}
