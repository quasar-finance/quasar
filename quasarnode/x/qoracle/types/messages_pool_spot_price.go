package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const (
	TypeMsgCreatePoolSpotPrice = "create_pool_spot_price"
	TypeMsgUpdatePoolSpotPrice = "update_pool_spot_price"
	TypeMsgDeletePoolSpotPrice = "delete_pool_spot_price"
)

var _ sdk.Msg = &MsgCreatePoolSpotPrice{}

func NewMsgCreatePoolSpotPrice(
	creator string,
	poolId string,
	denomIn string,
	denomOut string,
	price string,
	lastUpdatedTime uint64,

) *MsgCreatePoolSpotPrice {
	return &MsgCreatePoolSpotPrice{
		Creator:         creator,
		PoolId:          poolId,
		DenomIn:         denomIn,
		DenomOut:        denomOut,
		Price:           price,
		LastUpdatedTime: lastUpdatedTime,
	}
}

func (msg *MsgCreatePoolSpotPrice) Route() string {
	return RouterKey
}

func (msg *MsgCreatePoolSpotPrice) Type() string {
	return TypeMsgCreatePoolSpotPrice
}

func (msg *MsgCreatePoolSpotPrice) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgCreatePoolSpotPrice) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgCreatePoolSpotPrice) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	if len(msg.PoolId) == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "empty PoolId")
	}
	if err := sdk.ValidateDenom(msg.DenomIn); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "invalid DenomIn '%s': %s", msg.DenomIn, err.Error())
	}
	if err := sdk.ValidateDenom(msg.DenomOut); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "invalid DenomOut '%s': %s", msg.DenomOut, err.Error())
	}
	if price, err := sdk.NewDecFromStr(msg.Price); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "invalid Price '%s': %s", msg.Price, err.Error())
	} else if !price.IsPositive() {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "Price '%s' must be positive", msg.Price)
	}
	if msg.LastUpdatedTime == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "LastUpdatedTime is zero")
	}
	return nil
}

var _ sdk.Msg = &MsgUpdatePoolSpotPrice{}

func NewMsgUpdatePoolSpotPrice(
	creator string,
	poolId string,
	denomIn string,
	denomOut string,
	price string,
	lastUpdatedTime uint64,

) *MsgUpdatePoolSpotPrice {
	return &MsgUpdatePoolSpotPrice{
		Creator:         creator,
		PoolId:          poolId,
		DenomIn:         denomIn,
		DenomOut:        denomOut,
		Price:           price,
		LastUpdatedTime: lastUpdatedTime,
	}
}

func (msg *MsgUpdatePoolSpotPrice) Route() string {
	return RouterKey
}

func (msg *MsgUpdatePoolSpotPrice) Type() string {
	return TypeMsgUpdatePoolSpotPrice
}

func (msg *MsgUpdatePoolSpotPrice) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgUpdatePoolSpotPrice) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgUpdatePoolSpotPrice) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	if len(msg.PoolId) == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "empty PoolId")
	}
	if err := sdk.ValidateDenom(msg.DenomIn); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "invalid DenomIn '%s': %s", msg.DenomIn, err.Error())
	}
	if err := sdk.ValidateDenom(msg.DenomOut); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "invalid DenomOut '%s': %s", msg.DenomOut, err.Error())
	}
	if price, err := sdk.NewDecFromStr(msg.Price); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "invalid Price '%s': %s", msg.Price, err.Error())
	} else if !price.IsPositive() {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "Price '%s' must be positive", msg.Price)
	}
	if msg.LastUpdatedTime == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "LastUpdatedTime is zero")
	}
	return nil
}

var _ sdk.Msg = &MsgDeletePoolSpotPrice{}

func NewMsgDeletePoolSpotPrice(
	creator string,
	poolId string,
	denomIn string,
	denomOut string,

) *MsgDeletePoolSpotPrice {
	return &MsgDeletePoolSpotPrice{
		Creator:  creator,
		PoolId:   poolId,
		DenomIn:  denomIn,
		DenomOut: denomOut,
	}
}
func (msg *MsgDeletePoolSpotPrice) Route() string {
	return RouterKey
}

func (msg *MsgDeletePoolSpotPrice) Type() string {
	return TypeMsgDeletePoolSpotPrice
}

func (msg *MsgDeletePoolSpotPrice) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgDeletePoolSpotPrice) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgDeletePoolSpotPrice) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	if len(msg.PoolId) == 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "empty PoolId")
	}
	if err := sdk.ValidateDenom(msg.DenomIn); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "invalid DenomIn '%s': %s", msg.DenomIn, err.Error())
	}
	if err := sdk.ValidateDenom(msg.DenomOut); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidRequest, "invalid DenomOut '%s': %s", msg.DenomOut, err.Error())
	}
	return nil
}
