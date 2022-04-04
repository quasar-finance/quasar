package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const (
	TypeMsgCreatePoolPosition = "create_pool_position"
	TypeMsgUpdatePoolPosition = "update_pool_position"
	TypeMsgDeletePoolPosition = "delete_pool_position"
)

var _ sdk.Msg = &MsgCreatePoolPosition{}

func NewMsgCreatePoolPosition(
	creator string,
	poolId string,
	metrics *PoolMetrics,
	lastUpdatedTime uint64,

) *MsgCreatePoolPosition {
	return &MsgCreatePoolPosition{
		Creator:         creator,
		PoolId:          poolId,
		Metrics:         metrics,
		LastUpdatedTime: lastUpdatedTime,
	}
}

func (msg *MsgCreatePoolPosition) Route() string {
	return RouterKey
}

func (msg *MsgCreatePoolPosition) Type() string {
	return TypeMsgCreatePoolPosition
}

func (msg *MsgCreatePoolPosition) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgCreatePoolPosition) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgCreatePoolPosition) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}

var _ sdk.Msg = &MsgUpdatePoolPosition{}

func NewMsgUpdatePoolPosition(
	creator string,
	poolId string,
	metrics *PoolMetrics,
	lastUpdatedTime uint64,

) *MsgUpdatePoolPosition {
	return &MsgUpdatePoolPosition{
		Creator:         creator,
		PoolId:          poolId,
		Metrics:         metrics,
		LastUpdatedTime: lastUpdatedTime,
	}
}

func (msg *MsgUpdatePoolPosition) Route() string {
	return RouterKey
}

func (msg *MsgUpdatePoolPosition) Type() string {
	return TypeMsgUpdatePoolPosition
}

func (msg *MsgUpdatePoolPosition) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgUpdatePoolPosition) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgUpdatePoolPosition) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}

var _ sdk.Msg = &MsgDeletePoolPosition{}

func NewMsgDeletePoolPosition(
	creator string,
	poolId string,

) *MsgDeletePoolPosition {
	return &MsgDeletePoolPosition{
		Creator: creator,
		PoolId:  poolId,
	}
}
func (msg *MsgDeletePoolPosition) Route() string {
	return RouterKey
}

func (msg *MsgDeletePoolPosition) Type() string {
	return TypeMsgDeletePoolPosition
}

func (msg *MsgDeletePoolPosition) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgDeletePoolPosition) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgDeletePoolPosition) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}
