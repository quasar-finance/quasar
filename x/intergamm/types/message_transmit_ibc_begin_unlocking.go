package types

import (
	"cosmossdk.io/errors"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgTransmitIbcBeginUnlocking = "transmit_ibc_begin_unlocking"

var _ sdk.Msg = &MsgTransmitIbcBeginUnlocking{}

func NewMsgTransmitIbcBeginUnlocking(creator string, connectionId string, timoutTimestamp, id uint64, coins []sdk.Coin) *MsgTransmitIbcBeginUnlocking {
	return &MsgTransmitIbcBeginUnlocking{
		Creator:          creator,
		ConnectionId:     connectionId,
		TimeoutTimestamp: timoutTimestamp,
		Id:               id,
		Coins:            coins,
	}
}

func (msg *MsgTransmitIbcBeginUnlocking) Route() string {
	return RouterKey
}

func (msg *MsgTransmitIbcBeginUnlocking) Type() string {
	return TypeMsgTransmitIbcBeginUnlocking
}

func (msg *MsgTransmitIbcBeginUnlocking) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgTransmitIbcBeginUnlocking) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgTransmitIbcBeginUnlocking) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return errors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	if msg.ConnectionId == "" {
		return errors.Wrap(sdkerrors.ErrInvalidRequest, "connectionID cannot be empty")
	}
	if len(msg.Coins) == 0 {
		return errors.Wrap(sdkerrors.ErrInvalidCoins, "coins cannot be empty")
	}
	if msg.TimeoutTimestamp == 0 {
		return errors.Wrap(sdkerrors.ErrInvalidRequest, "TimeoutTimestamp cannot be 0")
	}
	return nil
}
