package types

import (
	"cosmossdk.io/errors"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
)

const TypeMsgTransmitIbcJoinPool = "transmit_ibc_join_pool"

var _ sdk.Msg = &MsgTransmitIbcJoinPool{}

func NewMsgTransmitIbcJoinPool(creator string, connectionId string, timoutTimestamp uint64, poolId uint64, shareOutAmount int64, tokenInMaxs []sdk.Coin) *MsgTransmitIbcJoinPool {
	return &MsgTransmitIbcJoinPool{
		Creator:          creator,
		ConnectionId:     connectionId,
		TimeoutTimestamp: timoutTimestamp,
		PoolId:           poolId,
		ShareOutAmount:   shareOutAmount,
		TokenInMaxs:      tokenInMaxs,
	}
}

func (msg *MsgTransmitIbcJoinPool) Route() string {
	return RouterKey
}

func (msg *MsgTransmitIbcJoinPool) Type() string {
	return TypeMsgTransmitIbcJoinPool
}

func (msg *MsgTransmitIbcJoinPool) GetSigners() []sdk.AccAddress {
	creator, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		panic(err)
	}
	return []sdk.AccAddress{creator}
}

func (msg *MsgTransmitIbcJoinPool) GetSignBytes() []byte {
	bz := ModuleCdc.MustMarshalJSON(msg)
	return sdk.MustSortJSON(bz)
}

func (msg *MsgTransmitIbcJoinPool) ValidateBasic() error {
	_, err := sdk.AccAddressFromBech32(msg.Creator)
	if err != nil {
		return errors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
	}
	if msg.ConnectionId == "" {
		return errors.Wrap(sdkerrors.ErrInvalidAddress, "connectionID cannot be empty")
	}
	if msg.ShareOutAmount == 0 {
		return errors.Wrap(sdkerrors.ErrInvalidRequest, "ShareInAmount cannot be 0")
	}
	if len(msg.TokenInMaxs) == 0 {
		return errors.Wrap(sdkerrors.ErrInvalidCoins, "coins cannot be empty")
	}
	if msg.TimeoutTimestamp == 0 {
		return errors.Wrap(sdkerrors.ErrInvalidRequest, "TimeoutTimestamp cannot be 0")
	}
	return nil
}
