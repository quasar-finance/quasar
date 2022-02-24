package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const (
	TypeMsgCreatePoolRanking = "create_pool_ranking"
	TypeMsgUpdatePoolRanking = "update_pool_ranking"
	TypeMsgDeletePoolRanking = "delete_pool_ranking"
)

var _ sdk.Msg = &MsgCreatePoolRanking{}

func NewMsgCreatePoolRanking(creator string, poolIdsSortedByAPY []string, poolIdsSortedByTVL []string, lastUpdatedTime uint64) *MsgCreatePoolRanking {
	return &MsgCreatePoolRanking{
		Creator:            creator,
		PoolIdsSortedByAPY: poolIdsSortedByAPY,
		PoolIdsSortedByTVL: poolIdsSortedByTVL,
		LastUpdatedTime:    lastUpdatedTime,
	}
}

func (msg *MsgCreatePoolRanking) Route() string {
	return RouterKey
}

func (msg *MsgCreatePoolRanking) Type() string {
	return TypeMsgCreatePoolRanking
}

func (msg *MsgCreatePoolRanking) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgCreatePoolRanking) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgCreatePoolRanking) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}

var _ sdk.Msg = &MsgUpdatePoolRanking{}

func NewMsgUpdatePoolRanking(creator string, poolIdsSortedByAPY []string, poolIdsSortedByTVL []string, lastUpdatedTime uint64) *MsgUpdatePoolRanking {
	return &MsgUpdatePoolRanking{
		Creator:            creator,
		PoolIdsSortedByAPY: poolIdsSortedByAPY,
		PoolIdsSortedByTVL: poolIdsSortedByTVL,
		LastUpdatedTime:    lastUpdatedTime,
	}
}

func (msg *MsgUpdatePoolRanking) Route() string {
	return RouterKey
}

func (msg *MsgUpdatePoolRanking) Type() string {
	return TypeMsgUpdatePoolRanking
}

func (msg *MsgUpdatePoolRanking) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgUpdatePoolRanking) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgUpdatePoolRanking) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}

var _ sdk.Msg = &MsgDeletePoolRanking{}

func NewMsgDeletePoolRanking(creator string) *MsgDeletePoolRanking {
	return &MsgDeletePoolRanking{
		Creator: creator,
	}
}
func (msg *MsgDeletePoolRanking) Route() string {
	return RouterKey
}

func (msg *MsgDeletePoolRanking) Type() string {
	return TypeMsgDeletePoolRanking
}

func (msg *MsgDeletePoolRanking) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgDeletePoolRanking) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgDeletePoolRanking) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}
