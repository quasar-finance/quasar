package keeper

import (
	"context"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
)

type msgServer struct {
	Keeper
}

// NewMsgServerImpl returns an implementation of the MsgServer interface
// for the provided Keeper.
func NewMsgServerImpl(keeper Keeper) types.MsgServer {
	return &msgServer{Keeper: keeper}
}

var _ types.MsgServer = msgServer{}

func (k msgServer) AddDenomSymbolMappings(goCtx context.Context, msg *types.MsgAddDenomSymbolMappings) (*types.MsgAddDenomSymbolMappingsResponse, error) {
	/*
		if k.authority != msg.Creator {
			return nil, sdkerrors.Wrapf(govtypes.ErrInvalidSigner, "expected %s got %s", k.authority, msg.Creator)
		}
	*/
	ctx := sdk.UnwrapSDKContext(goCtx)

	for _, mapping := range msg.Mappings {
		k.SetDenomSymbolMapping(ctx, mapping)
	}

	return &types.MsgAddDenomSymbolMappingsResponse{}, nil
}

func (k msgServer) RemoveDenomSymbolMappings(goCtx context.Context, msg *types.MsgRemoveDenomSymbolMappings) (*types.MsgRemoveDenomSymbolMappingsResponse, error) {
	/*
		if k.authority != msg.Creator {
			return nil, errors.Wrapf(govtypes.ErrInvalidSigner, "expected %s got %s", k.authority, msg.Creator)
		}
	*/
	ctx := sdk.UnwrapSDKContext(goCtx)

	for _, denom := range msg.Denoms {
		k.DeleteDenomSymbolMapping(ctx, denom)
	}

	return &types.MsgRemoveDenomSymbolMappingsResponse{}, nil
}
