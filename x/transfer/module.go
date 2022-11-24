package transfer

import (
	"github.com/CosmWasm/wasmd/x/wasm"
	sdk "github.com/cosmos/cosmos-sdk/types"
	sdkerrors "github.com/cosmos/cosmos-sdk/types/errors"
	"github.com/cosmos/cosmos-sdk/types/module"
	"github.com/cosmos/ibc-go/v3/modules/apps/transfer"
	"github.com/cosmos/ibc-go/v3/modules/apps/transfer/keeper"
	channeltypes "github.com/cosmos/ibc-go/v3/modules/core/04-channel/types"

	transfermodulekeeper "github.com/quasarlabs/quasarnode/x/transfer/keeper"
	transfermoduletypes "github.com/quasarlabs/quasarnode/x/transfer/types"
)

/*
	In addition to original ack processing of ibc transfer acknowledgement we want to pass the acknowledgement to originating wasm contract.
	The package contains a code to achieve the purpose.
*/

type IBCModule struct {
	keeper     keeper.Keeper
	wasmKeeper *wasm.Keeper
	transfer.IBCModule
}

// NewIBCModule creates a new IBCModule given the keeper
func NewIBCModule(k transfermodulekeeper.KeeperTransferWrapper, wasmKeeper *wasm.Keeper) IBCModule {
	return IBCModule{
		keeper:     k.Keeper,
		IBCModule:  transfer.NewIBCModule(k.Keeper),
		wasmKeeper: wasmKeeper,
	}
}

// OnAcknowledgementPacket implements the IBCModule interface.
// Wrapper struct shadows(overrides) the OnAcknowledgementPacket method to achieve the package's purpose.
func (im IBCModule) OnAcknowledgementPacket(
	ctx sdk.Context,
	packet channeltypes.Packet,
	acknowledgement []byte,
	relayer sdk.AccAddress,
) error {
	err := im.IBCModule.OnAcknowledgementPacket(ctx, packet, acknowledgement, relayer)
	if err != nil {
		return sdkerrors.Wrap(err, "failed to process original OnAcknowledgementPacket")
	}
	return im.HandleAcknowledgement(ctx, packet, acknowledgement, relayer)
}

// OnTimeoutPacket implements the IBCModule interface.
func (im IBCModule) OnTimeoutPacket(
	ctx sdk.Context,
	packet channeltypes.Packet,
	relayer sdk.AccAddress,
) error {
	err := im.IBCModule.OnTimeoutPacket(ctx, packet, relayer)
	if err != nil {
		return sdkerrors.Wrap(err, "failed to process original OnTimeoutPacket")
	}
	return im.HandleTimeout(ctx, packet, relayer)
}

type AppModule struct {
	transfer.AppModule
	keeper transfermodulekeeper.KeeperTransferWrapper
}

// NewAppModule creates a new 20-transfer module
func NewAppModule(k transfermodulekeeper.KeeperTransferWrapper) AppModule {
	return AppModule{
		AppModule: transfer.NewAppModule(k.Keeper),
		keeper:    k,
	}
}

// RegisterServices registers module services.
func (am AppModule) RegisterServices(cfg module.Configurator) {
	transfermoduletypes.RegisterMsgServer(cfg.MsgServer(), am.keeper)
	transfermoduletypes.RegisterQueryServer(cfg.QueryServer(), am.keeper)
}
