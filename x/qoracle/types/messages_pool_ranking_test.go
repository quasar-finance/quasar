package types

import (
	"testing"

	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/quasarlabs/quasarnode/testutil/sample"
	"github.com/stretchr/testify/require"
)

func TestMsgCreatePoolRanking_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgCreatePoolRanking
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgCreatePoolRanking{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid",
			msg: MsgCreatePoolRanking{
				Creator:            sample.AccAddressStr(),
				PoolIdsSortedByAPY: []string{"1"},
				PoolIdsSortedByTVL: []string{"1"},
				LastUpdatedTime:    1,
			},
		}, {
			name: "empty slice",
			msg: MsgCreatePoolRanking{
				Creator:         sample.AccAddressStr(),
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "zero LastUpdatedTime",
			msg: MsgCreatePoolRanking{
				Creator:            sample.AccAddressStr(),
				PoolIdsSortedByAPY: []string{"1"},
				PoolIdsSortedByTVL: []string{"1"},
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "unequal slice length",
			msg: MsgCreatePoolRanking{
				Creator:            sample.AccAddressStr(),
				PoolIdsSortedByAPY: []string{"1"},
				PoolIdsSortedByTVL: []string{"1", "2"},
				LastUpdatedTime:    1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "empty pool id",
			msg: MsgCreatePoolRanking{
				Creator:            sample.AccAddressStr(),
				PoolIdsSortedByAPY: []string{""},
				PoolIdsSortedByTVL: []string{""},
				LastUpdatedTime:    1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "repeated id",
			msg: MsgCreatePoolRanking{
				Creator:            sample.AccAddressStr(),
				PoolIdsSortedByAPY: []string{"1", "1"},
				PoolIdsSortedByTVL: []string{"1", "2"},
				LastUpdatedTime:    1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "id exist in one slice only",
			msg: MsgCreatePoolRanking{
				Creator:            sample.AccAddressStr(),
				PoolIdsSortedByAPY: []string{"1", "3"},
				PoolIdsSortedByTVL: []string{"1", "2"},
				LastUpdatedTime:    1,
			},
			err: sdkerrors.ErrInvalidRequest,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.msg.ValidateBasic()
			if tt.err != nil {
				require.ErrorIs(t, err, tt.err)
				return
			}
			require.NoError(t, err)
		})
	}
}

func TestMsgUpdatePoolRanking_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgUpdatePoolRanking
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgUpdatePoolRanking{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid address",
			msg: MsgUpdatePoolRanking{
				Creator:            sample.AccAddressStr(),
				PoolIdsSortedByAPY: []string{"1"},
				PoolIdsSortedByTVL: []string{"1"},
				LastUpdatedTime:    1,
			},
		}, {
			name: "empty slice",
			msg: MsgUpdatePoolRanking{
				Creator:         sample.AccAddressStr(),
				LastUpdatedTime: 1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "zero LastUpdatedTime",
			msg: MsgUpdatePoolRanking{
				Creator:            sample.AccAddressStr(),
				PoolIdsSortedByAPY: []string{"1"},
				PoolIdsSortedByTVL: []string{"1"},
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "unequal slice length",
			msg: MsgUpdatePoolRanking{
				Creator:            sample.AccAddressStr(),
				PoolIdsSortedByAPY: []string{"1"},
				PoolIdsSortedByTVL: []string{"1", "2"},
				LastUpdatedTime:    1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "empty pool id",
			msg: MsgUpdatePoolRanking{
				Creator:            sample.AccAddressStr(),
				PoolIdsSortedByAPY: []string{""},
				PoolIdsSortedByTVL: []string{""},
				LastUpdatedTime:    1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "repeated id",
			msg: MsgUpdatePoolRanking{
				Creator:            sample.AccAddressStr(),
				PoolIdsSortedByAPY: []string{"1", "1"},
				PoolIdsSortedByTVL: []string{"1", "2"},
				LastUpdatedTime:    1,
			},
			err: sdkerrors.ErrInvalidRequest,
		}, {
			name: "id exist in one slice only",
			msg: MsgUpdatePoolRanking{
				Creator:            sample.AccAddressStr(),
				PoolIdsSortedByAPY: []string{"1", "3"},
				PoolIdsSortedByTVL: []string{"1", "2"},
				LastUpdatedTime:    1,
			},
			err: sdkerrors.ErrInvalidRequest,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.msg.ValidateBasic()
			if tt.err != nil {
				require.ErrorIs(t, err, tt.err)
				return
			}
			require.NoError(t, err)
		})
	}
}

func TestMsgDeletePoolRanking_ValidateBasic(t *testing.T) {
	tests := []struct {
		name string
		msg  MsgDeletePoolRanking
		err  error
	}{
		{
			name: "invalid address",
			msg: MsgDeletePoolRanking{
				Creator: "invalid_address",
			},
			err: sdkerrors.ErrInvalidAddress,
		}, {
			name: "valid address",
			msg: MsgDeletePoolRanking{
				Creator: sample.AccAddressStr(),
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.msg.ValidateBasic()
			if tt.err != nil {
				require.ErrorIs(t, err, tt.err)
				return
			}
			require.NoError(t, err)
		})
	}
}
