package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgCreateVestingAccount = "create_vesting_account"

var _ sdk.Msg = &MsgCreateVestingAccount{}

func NewMsgCreateVestingAccount(creator string, toAddress string, amount sdk.Coin, startTime int64, endTime int64) *MsgCreateVestingAccount {
	return &MsgCreateVestingAccount{
		Creator:   creator,
		ToAddress: toAddress,
		Amount:    amount,
		StartTime: startTime,
		EndTime:   endTime,
	}
}

func (msg *MsgCreateVestingAccount) Route() string {
	return RouterKey
}

func (msg *MsgCreateVestingAccount) Type() string {
	return TypeMsgCreateVestingAccount
}

func (msg *MsgCreateVestingAccount) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
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
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}
