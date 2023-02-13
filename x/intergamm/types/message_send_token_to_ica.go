package types

import (
	"cosmossdk.io/errors"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgSendTokenToICA = "send_token_to_ica"

var _ sdk.Msg = &MsgSendTokenToICA{}

func NewMsgSendTokenToICA(fromAddress string, toZoneId string, coin sdk.Coin) *MsgSendTokenToICA {
	return &MsgSendTokenToICA{
		FromAddress: fromAddress,
		ToZoneId:    toZoneId,
		Coin:        coin,
	}
}

func (msg *MsgSendTokenToICA) Route() string {
	return RouterKey
}

func (msg *MsgSendTokenToICA) Type() string {
	return TypeMsgSendTokenToICA
}

func (msg *MsgSendTokenToICA) GetSigners() []sdk.AccAddress {
	fromAddress, err := sdk.AccAddressFromBech32(msg.FromAddress)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{fromAddress}
}

func (msg *MsgSendTokenToICA) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgSendTokenToICA) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.FromAddress)
	if err != nil {
		return errors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid fromAddress address (%s)", err)
	}
	if msg.ToZoneId == "" {
		return errors.Wrap(ErrInvalidZoneId, "toZoneId cannot be empty")
	}
	if !msg.Coin.IsValid() {
		return errors.Wrapf(sdkerrors.ErrInvalidCoins, "coin (%s) must be valid", msg.Coin.String())
	}
	if !msg.Coin.IsPositive() {
		return errors.Wrapf(sdkerrors.ErrInvalidCoins, "coin (%s) must be positive", msg.Coin.String())
	}
	return nil
}
