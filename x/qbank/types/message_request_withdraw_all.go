package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgRequestWithdrawAll = "request_withdraw_all"

var _ sdk.Msg = &MsgRequestWithdrawAll{}

func NewMsgRequestWithdrawAll(creator string, vaultID string) *MsgRequestWithdrawAll {
	return &MsgRequestWithdrawAll{
		Creator: creator,
		VaultID: vaultID,
	}
}

func (msg *MsgRequestWithdrawAll) Route() string {
	return RouterKey
}

func (msg *MsgRequestWithdrawAll) Type() string {
	return TypeMsgRequestWithdrawAll
}

func (msg *MsgRequestWithdrawAll) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgRequestWithdrawAll) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgRequestWithdrawAll) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}

	if msg.GetVaultID() == "" || !Contains(SupportedVaultTypes, msg.GetVaultID()) {
		return ErrInvalidVaultId
	}
	return nil
}
