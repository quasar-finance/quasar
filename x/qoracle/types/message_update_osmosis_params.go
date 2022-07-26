package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgUpdateOsmosisChainParams = "update_osmosis_params"

var _ sdk.Msg = &MsgUpdateOsmosisChainParams{}

func NewMsgUpdateOsmosisChainParams(creator string) *MsgUpdateOsmosisChainParams {
	return &MsgUpdateOsmosisChainParams{
		Creator: creator,
	}
}

func (msg *MsgUpdateOsmosisChainParams) Route() string {
	return RouterKey
}

func (msg *MsgUpdateOsmosisChainParams) Type() string {
	return TypeMsgUpdateOsmosisChainParams
}

func (msg *MsgUpdateOsmosisChainParams) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgUpdateOsmosisChainParams) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgUpdateOsmosisChainParams) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	return nil
}
