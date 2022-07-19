package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgUpdateOsmosisParams = "update_osmosis_params"

var _ sdk.Msg = &MsgUpdateOsmosisParams{}

func NewMsgUpdateOsmosisParams(creator string) *MsgUpdateOsmosisParams {
	return &MsgUpdateOsmosisParams{
		Creator: creator,
	}
}

func (msg *MsgUpdateOsmosisParams) Route() string {
	return RouterKey
}

func (msg *MsgUpdateOsmosisParams) Type() string {
	return TypeMsgUpdateOsmosisParams
}

func (msg *MsgUpdateOsmosisParams) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgUpdateOsmosisParams) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgUpdateOsmosisParams) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}
