package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgRequestWithdraw = "request_withdraw"

var _ sdk.Msg = &MsgRequestWithdraw{}

func NewMsgRequestWithdraw(creator string, riskProfile string, vaultID string, coin sdk.Coin) *MsgRequestWithdraw {
	return &MsgRequestWithdraw{
		Creator:     creator,
		RiskProfile: riskProfile,
		VaultID:     vaultID,
		Coin:        coin,
	}
}

func (msg *MsgRequestWithdraw) Route() string {
	return RouterKey
}

func (msg *MsgRequestWithdraw) Type() string {
	return TypeMsgRequestWithdraw
}

func (msg *MsgRequestWithdraw) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgRequestWithdraw) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgRequestWithdraw) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}
