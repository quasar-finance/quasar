package types

import (
	"cosmossdk.io/errors"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgSendToken = "send_token"

var _ sdk.Msg = &MsgSendToken{}

func NewMsgSendToken(fromAddress string, toZoneId string, toAddress string, coin sdk.Coin) *MsgSendToken {
	return &MsgSendToken{
		FromAddress: fromAddress,
		ToZoneId:    toZoneId,
		ToAddress:   toAddress,
		Coin:        coin,
	}
}

func (msg *MsgSendToken) Route() string {
	return RouterKey
}

func (msg *MsgSendToken) Type() string {
	return TypeMsgSendToken
}

func (msg *MsgSendToken) GetSigners() []sdk.AccAddress {
	fromAddress, err := sdk.AccAddressFromBech32(msg.FromAddress)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{fromAddress}
}

func (msg *MsgSendToken) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgSendToken) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.FromAddress)
	if err != nil {
		return errors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid fromAddress address (%s)", err)
	}
	if msg.ToZoneId == "" {
		return errors.Wrap(ErrInvalidZoneId, "toZoneId cannot be empty")
	}
	if msg.ToAddress == "" {
		return errors.Wrap(sdkerrors.ErrInvalidAddress, "toAddress cannot be empty")
	}
	if !msg.Coin.IsValid() {
		return errors.Wrapf(sdkerrors.ErrInvalidCoins, "coin (%s) must be valid", msg.Coin.String())
	}
	if !msg.Coin.IsPositive() {
		return errors.Wrapf(sdkerrors.ErrInvalidCoins, "coin (%s) must be positive", msg.Coin.String())
	}
	return nil
}
