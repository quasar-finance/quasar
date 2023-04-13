package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgCreateVestingAccount = "create_vesting_account"

var _ sdk.Msg = &MsgCreateVestingAccount{}

func NewMsgCreateVestingAccount(fromAddress string, toAddress sdk.AccAddress, amount sdk.Coins, startTime int64, endTime int64) *MsgCreateVestingAccount {
	return &MsgCreateVestingAccount{
		FromAddress: fromAddress,
		ToAddress:   toAddress.String(),
		Amount:      amount,
		StartTime:   startTime,
		EndTime:     endTime,
	}
}

func (msg *MsgCreateVestingAccount) Route() string {
	return RouterKey
}

func (msg *MsgCreateVestingAccount) Type() string {
	return TypeMsgCreateVestingAccount
}

func (msg *MsgCreateVestingAccount) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.FromAddress)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgCreateVestingAccount) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgCreateVestingAccount) ValidateBasic() error {
	from, err := sdk.AccAddressFromBech32(msg.FromAddress)
	if err != nil {
		return err
	}
	to, err := sdk.AccAddressFromBech32(msg.ToAddress)
	if err != nil {
		return err
	}
	if err := sdk.VerifyAddressFormat(from); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid sender address: %s", err)
	}

	if err := sdk.VerifyAddressFormat(to); err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid recipient address: %s", err)
	}

	if !msg.Amount.IsValid() {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidCoins, msg.Amount.String())
	}

	if !msg.Amount.IsAllPositive() {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidCoins, msg.Amount.String())
	}

	if msg.StartTime <= 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "invalid start time")
	}

	if msg.EndTime <= 0 {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "invalid end time")
	}

	if msg.StartTime > msg.EndTime {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, "invalid start time higher than end time")
	}

	return nil
}
