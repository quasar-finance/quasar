package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgRequestDeposit = "request_deposit"

var _ sdk.Msg = &MsgRequestDeposit{}

func NewMsgRequestDeposit(creator string,
	// riskProfile string,
	vaultID string,
	coin sdk.Coin, lockupPeriod LockupTypes, reserved []string) *MsgRequestDeposit {
	return &MsgRequestDeposit{
		Creator:      creator,
		VaultID:      vaultID,
		Coin:         coin,
		LockupPeriod: lockupPeriod,
		Reserved:     reserved,
	}
}

func (msg *MsgRequestDeposit) Route() string {
	return RouterKey
}

func (msg *MsgRequestDeposit) Type() string {
	return TypeMsgRequestDeposit
}

func (msg *MsgRequestDeposit) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgRequestDeposit) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgRequestDeposit) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}

	if msg.GetVaultID() == "" || !Contains(SupportedVaultTypes, msg.GetVaultID()) {
		return ErrInvalidVaultId
	}

	if msg.GetCoin().IsZero() || !msg.GetCoin().IsValid() {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidCoins, "deposit amount %s", msg.Coin)
	}

	v := LockupTypes_name[int32(msg.GetLockupPeriod())]
	if v == "" || v == "Invalid" {
		return ErrInvalidLockupType
	}

	if len(msg.Reserved) != int(ReservedFieldLenMap[msg.GetVaultID()]) {
		return ErrReservedFieldLength
	}

	return nil
}
