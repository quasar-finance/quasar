package types

import (
	"cosmossdk.io/errors"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgTransmitIbcJoinSwapExternAmountIn = "transmit_ibc_join_swap_extern_amount_in"

var _ sdk.Msg = &MsgTransmitIbcJoinSwapExternAmountIn{}

func NewMsgTransmitIbcJoinSwapExternAmountIn(creator string, connectionId string, timoutTimestamp uint64, poolId uint64, shareOutMinAmount int64, tokenIn sdk.Coin) *MsgTransmitIbcJoinSwapExternAmountIn {
	return &MsgTransmitIbcJoinSwapExternAmountIn{
		Creator:           creator,
		ConnectionId:      connectionId,
		TimeoutTimestamp:  timoutTimestamp,
		PoolId:            poolId,
		ShareOutMinAmount: shareOutMinAmount,
		TokenIn:           tokenIn,
	}
}

func (msg *MsgTransmitIbcJoinSwapExternAmountIn) Route() string {
	return RouterKey
}

func (msg *MsgTransmitIbcJoinSwapExternAmountIn) Type() string {
	return TypeMsgTransmitIbcJoinSwapExternAmountIn
}

func (msg *MsgTransmitIbcJoinSwapExternAmountIn) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgTransmitIbcJoinSwapExternAmountIn) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgTransmitIbcJoinSwapExternAmountIn) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return errors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	if msg.ConnectionId == "" {
		return errors.Wrap(sdkerrors.ErrInvalidAddress, "connectionID cannot be empty")
	}
	if msg.ShareOutMinAmount == 0 {
		return errors.Wrap(sdkerrors.ErrInvalidRequest, "ShareInAmount cannot be 0")
	}
	if msg.TokenIn.Amount == sdk.ZeroInt() || msg.TokenIn.Denom == "" {
		return errors.Wrap(sdkerrors.ErrInvalidCoins, "tokenIn cannot have nil field ()")
	}
	if msg.TimeoutTimestamp == 0 {
		return errors.Wrap(sdkerrors.ErrInvalidRequest, "TimeoutTimestamp cannot be 0")
	}
	return nil
}
