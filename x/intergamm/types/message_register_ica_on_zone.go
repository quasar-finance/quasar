package types

import (
	"cosmossdk.io/errors"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgRegisterICAOnZone = "register_ica_on_zone"

var _ sdk.Msg = &MsgRegisterICAOnZone{}

func NewMsgRegisterICAOnZone(ownerAddress string, zoneId string) *MsgRegisterICAOnZone {
	return &MsgRegisterICAOnZone{
		OwnerAddress: ownerAddress,
		ZoneId:       zoneId,
	}
}

func (msg *MsgRegisterICAOnZone) Route() string {
	return RouterKey
}

func (msg *MsgRegisterICAOnZone) Type() string {
	return TypeMsgRegisterICAOnZone
}

func (msg *MsgRegisterICAOnZone) GetSigners() []sdk.AccAddress {
	ownerAddress, err := sdk.AccAddressFromBech32(msg.OwnerAddress)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{ownerAddress}
}

func (msg *MsgRegisterICAOnZone) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgRegisterICAOnZone) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.OwnerAddress)
	if err != nil {
		return errors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid ownerAddress address (%s)", err)
	}
	if msg.ZoneId == "" {
		return errors.Wrap(ErrInvalidZoneId, "zoneId cannot be empty")
	}
	return nil
}
