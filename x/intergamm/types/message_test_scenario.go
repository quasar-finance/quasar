package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgTestScenario = "test_scenario"

var _ sdk.Msg = &MsgTestScenario{}

func NewMsgTestScenario(creator string, scenario string) *MsgTestScenario {
	return &MsgTestScenario{
		Creator:  creator,
		Scenario: scenario,
	}
}

func (msg *MsgTestScenario) Route() string {
	return RouterKey
}

func (msg *MsgTestScenario) Type() string {
	return TypeMsgTestScenario
}

func (msg *MsgTestScenario) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgTestScenario) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgTestScenario) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	if msg.Scenario == "" {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidAddress, "scenario cannot be empty")
	}
	return nil
}
