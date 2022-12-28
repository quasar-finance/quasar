package qoracle

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/codec"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	"github.com/cosmos/cosmos-sdk/telemetry"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
	"github.com/gorilla/mux"
	"github.com/grpc-ecosystem/grpc-gateway/runtime"
	qbandkeeper "github.com/quasarlabs/quasarnode/x/qoracle/bandchain/keeper"
	qbandtypes "github.com/quasarlabs/quasarnode/x/qoracle/bandchain/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/client/cli"
	genesistypes "github.com/quasarlabs/quasarnode/x/qoracle/genesis/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/keeper"
	qosmokeeper "github.com/quasarlabs/quasarnode/x/qoracle/osmosis/keeper"
	qosmotypes "github.com/quasarlabs/quasarnode/x/qoracle/osmosis/types"
	"github.com/quasarlabs/quasarnode/x/qoracle/types"
	"github.com/spf13/cobra"
	abci "github.com/tendermint/tendermint/abci/types"
)

var (
	_ module.AppModule      = AppModule{}
	_ module.AppModuleBasic = AppModuleBasic{}
)

// ----------------------------------------------------------------------------
// AppModuleBasic
// ----------------------------------------------------------------------------

// AppModuleBasic implements the AppModuleBasic interface for the capability module.
type AppModuleBasic struct {
	cdc codec.BinaryCodec
}

func NewAppModuleBasic(cdc codec.BinaryCodec) AppModuleBasic {
	return AppModuleBasic{cdc: cdc}
}

// Name returns the capability module's name.
func (AppModuleBasic) Name() string {
	return types.ModuleName
}

func (AppModuleBasic) RegisterLegacyAminoCodec(cdc *codec.LegacyAmino) {}

// RegisterInterfaces registers the module's interface types
func (a AppModuleBasic) RegisterInterfaces(reg cdctypes.InterfaceRegistry) {
	qbandtypes.RegisterInterfaces(reg)
	qosmotypes.RegisterInterfaces(reg)
}

// DefaultGenesis returns the capability module's default genesis state.
func (AppModuleBasic) DefaultGenesis(cdc codec.JSONCodec) json.RawMessage {
	return cdc.MustMarshalJSON(genesistypes.DefaultGenesis())
}

// ValidateGenesis performs genesis state validation for the capability module.
func (AppModuleBasic) ValidateGenesis(cdc codec.JSONCodec, config client.TxEncodingConfig, bz json.RawMessage) error {
	var genState genesistypes.GenesisState
	if err := cdc.UnmarshalJSON(bz, &genState); err != nil {
		return fmt.Errorf("failed to unmarshal %s genesis state: %w", types.ModuleName, err)
	}
	return genState.Validate()
}

// RegisterRESTRoutes registers the capability module's REST service handlers.
func (AppModuleBasic) RegisterRESTRoutes(clientCtx client.Context, rtr *mux.Router) {}

// RegisterGRPCGatewayRoutes registers the gRPC Gateway routes for the module.
func (AppModuleBasic) RegisterGRPCGatewayRoutes(clientCtx client.Context, mux *runtime.ServeMux) {
	err := types.RegisterQueryHandlerClient(context.Background(), mux, types.NewQueryClient(clientCtx))
	if err != nil {
		panic(err)
	}

	err = qbandtypes.RegisterQueryHandlerClient(context.Background(), mux, qbandtypes.NewQueryClient(clientCtx))
	if err != nil {
		panic(err)
	}

	err = qosmotypes.RegisterQueryHandlerClient(context.Background(), mux, qosmotypes.NewQueryClient(clientCtx))
	if err != nil {
		panic(err)
	}
}

// GetTxCmd returns the capability module's root tx command.
func (a AppModuleBasic) GetTxCmd() *cobra.Command {
	return nil
}

// GetQueryCmd returns the capability module's root query command.
func (AppModuleBasic) GetQueryCmd() *cobra.Command {
	return cli.GetQueryCmd()
}

// ----------------------------------------------------------------------------
// AppModule
// ----------------------------------------------------------------------------

// AppModule implements the AppModule interface for the capability module.
type AppModule struct {
	AppModuleBasic

	keeper          keeper.Keeper
	bandchainKeeper qbandkeeper.Keeper
	osmosisKeeper   qosmokeeper.Keeper
}

func NewAppModule(
	cdc codec.Codec,
	keeper keeper.Keeper,
	bandchainKeeper qbandkeeper.Keeper,
	osmosisKeeper qosmokeeper.Keeper,
) AppModule {
	return AppModule{
		AppModuleBasic:  NewAppModuleBasic(cdc),
		keeper:          keeper,
		bandchainKeeper: bandchainKeeper,
		osmosisKeeper:   osmosisKeeper,
	}
}

// InitModule will initialize the qoracle moudule. It should only be
// called once and as an alternative to InitGenesis.
func (am AppModule) InitModule(ctx sdk.Context, bandchainParams qbandtypes.Params, osmosisParams qosmotypes.Params) {
	am.bandchainKeeper.SetParams(ctx, bandchainParams)
	if err := am.bandchainKeeper.BindPort(ctx, qbandtypes.PortID); err != nil {
		panic(fmt.Sprintf("could not claim port capability: %v", err))
	}

	am.osmosisKeeper.SetParams(ctx, osmosisParams)
	if err := am.osmosisKeeper.BindPort(ctx, qosmotypes.PortID); err != nil {
		panic(fmt.Sprintf("could not claim port capability: %v", err))
	}
}

// RegisterInvariants implements the AppModule interface
func (AppModule) RegisterInvariants(ir sdk.InvariantRegistry) {
}

// Route implements the AppModule interface
func (AppModule) Route() sdk.Route {
	return sdk.NewRoute(types.RouterKey, nil)
}

// NewHandler implements the AppModule interface
func (AppModule) NewHandler() sdk.Handler {
	return nil
}

// QuerierRoute implements the AppModule interface
func (AppModule) QuerierRoute() string {
	return types.QuerierRoute
}

// LegacyQuerierHandler implements the AppModule interface
func (am AppModule) LegacyQuerierHandler(legacyQuerierCdc *codec.LegacyAmino) sdk.Querier {
	return nil
}

// RegisterServices registers a GRPC query service to respond to the
// module-specific GRPC queries.
func (am AppModule) RegisterServices(cfg module.Configurator) {
	types.RegisterQueryServer(cfg.QueryServer(), am.keeper)

	qbandtypes.RegisterQueryServer(cfg.QueryServer(), am.bandchainKeeper)

	qosmotypes.RegisterMsgServer(cfg.MsgServer(), qosmokeeper.NewMsgServerImpl(am.osmosisKeeper))
	qosmotypes.RegisterQueryServer(cfg.QueryServer(), am.osmosisKeeper)
}

// InitGenesis performs the capability module's genesis initialization It returns
// no validator updates.
func (am AppModule) InitGenesis(ctx sdk.Context, cdc codec.JSONCodec, gs json.RawMessage) []abci.ValidatorUpdate {
	var genesisState genesistypes.GenesisState
	cdc.MustUnmarshalJSON(gs, &genesisState)

	am.keeper.SetParams(ctx, genesisState.Params)
	qbandkeeper.InitGenesis(ctx, am.bandchainKeeper, genesisState.BandchainGenesisState)
	qosmokeeper.InitGenesis(ctx, am.osmosisKeeper, genesisState.OsmosisGenesisState)

	return []abci.ValidatorUpdate{}
}

// ExportGenesis returns the capability module's exported genesis state as raw JSON bytes.
func (am AppModule) ExportGenesis(ctx sdk.Context, cdc codec.JSONCodec) json.RawMessage {
	gs := genesistypes.NewGenesisState(
		am.keeper.GetParams(ctx),
		qbandkeeper.ExportGenesis(ctx, am.bandchainKeeper),
		qosmokeeper.ExportGenesis(ctx, am.osmosisKeeper),
	)

	return cdc.MustMarshalJSON(gs)
}

// ConsensusVersion implements ConsensusVersion.
func (AppModule) ConsensusVersion() uint64 { return 2 }

// BeginBlock executes all ABCI BeginBlock logic respective to the capability module.
// BeginBlocker calls InitMemStore to assert that the memory store is initialized.
// It's safe to run multiple times.
func (am AppModule) BeginBlock(ctx sdk.Context, _ abci.RequestBeginBlock) {
	defer telemetry.ModuleMeasureSince(types.ModuleName, time.Now(), telemetry.MetricKeyBeginBlocker)

	am.keeper.InitMemStore(ctx)
}

// EndBlock executes all ABCI EndBlock logic respective to the capability module. It
// returns no validator updates.
func (am AppModule) EndBlock(ctx sdk.Context, _ abci.RequestEndBlock) []abci.ValidatorUpdate {
	defer telemetry.ModuleMeasureSince(types.ModuleName, time.Now(), telemetry.MetricKeyEndBlocker)

	am.keeper.UpdateMemStore(ctx)

	return []abci.ValidatorUpdate{}
}
