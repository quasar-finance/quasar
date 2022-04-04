package qbank

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/gorilla/mux"
	"github.com/grpc-ecosystem/grpc-gateway/runtime"
	"github.com/spf13/cobra"

	abci "github.com/tendermint/tendermint/abci/types"

	"github.com/abag/quasarnode/x/qbank/client/cli"
	"github.com/abag/quasarnode/x/qbank/keeper"
	"github.com/abag/quasarnode/x/qbank/types"
	"github.com/cosmos/cosmos-sdk/client"
	"github.com/cosmos/cosmos-sdk/codec"
	cdctypes "github.com/cosmos/cosmos-sdk/codec/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/module"
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

func (AppModuleBasic) RegisterCodec(cdc *codec.LegacyAmino) {
	types.RegisterCodec(cdc)
}

func (AppModuleBasic) RegisterLegacyAminoCodec(cdc *codec.LegacyAmino) {
	types.RegisterCodec(cdc)
}

// RegisterInterfaces registers the module's interface types
func (a AppModuleBasic) RegisterInterfaces(reg cdctypes.InterfaceRegistry) {
	types.RegisterInterfaces(reg)
}

// DefaultGenesis returns the capability module's default genesis state.
func (AppModuleBasic) DefaultGenesis(cdc codec.JSONCodec) json.RawMessage {
	return cdc.MustMarshalJSON(types.DefaultGenesis())
}

// ValidateGenesis performs genesis state validation for the capability module.
func (AppModuleBasic) ValidateGenesis(cdc codec.JSONCodec, config client.TxEncodingConfig, bz json.RawMessage) error {
	var genState types.GenesisState
	if err := cdc.UnmarshalJSON(bz, &genState); err != nil {
		return fmt.Errorf("failed to unmarshal %s genesis state: %w", types.ModuleName, err)
	}
	return genState.Validate()
}

// RegisterRESTRoutes registers the capability module's REST service handlers.
func (AppModuleBasic) RegisterRESTRoutes(clientCtx client.Context, rtr *mux.Router) {
}

// RegisterGRPCGatewayRoutes registers the gRPC Gateway routes for the module.
func (AppModuleBasic) RegisterGRPCGatewayRoutes(clientCtx client.Context, mux *runtime.ServeMux) {
	types.RegisterQueryHandlerClient(context.Background(), mux, types.NewQueryClient(clientCtx))
}

// GetTxCmd returns the capability module's root tx command.
func (a AppModuleBasic) GetTxCmd() *cobra.Command {
	return cli.GetTxCmd()
}

// GetQueryCmd returns the capability module's root query command.
func (AppModuleBasic) GetQueryCmd() *cobra.Command {
	return cli.GetQueryCmd(types.StoreKey)
}

// ----------------------------------------------------------------------------
// AppModule
// ----------------------------------------------------------------------------

// AppModule implements the AppModule interface for the capability module.
type AppModule struct {
	AppModuleBasic

	keeper        keeper.Keeper
	accountKeeper types.AccountKeeper
	bankKeeper    types.BankKeeper
}

func NewAppModule(
	cdc codec.Codec,
	keeper keeper.Keeper,
	accountKeeper types.AccountKeeper,
	bankKeeper types.BankKeeper,
) AppModule {
	return AppModule{
		AppModuleBasic: NewAppModuleBasic(cdc),
		keeper:         keeper,
		accountKeeper:  accountKeeper,
		bankKeeper:     bankKeeper,
	}
}

// Name returns the capability module's name.
func (am AppModule) Name() string {
	return am.AppModuleBasic.Name()
}

// Route returns the capability module's message routing key.
func (am AppModule) Route() sdk.Route {
	return sdk.NewRoute(types.RouterKey, NewHandler(am.keeper))
}

// QuerierRoute returns the capability module's query routing key.
func (AppModule) QuerierRoute() string { return types.QuerierRoute }

// LegacyQuerierHandler returns the capability module's Querier.
func (am AppModule) LegacyQuerierHandler(legacyQuerierCdc *codec.LegacyAmino) sdk.Querier {
	return nil
}

// RegisterServices registers a GRPC query service to respond to the
// module-specific GRPC queries.
func (am AppModule) RegisterServices(cfg module.Configurator) {
	types.RegisterQueryServer(cfg.QueryServer(), am.keeper)
}

// RegisterInvariants registers the capability module's invariants.
func (am AppModule) RegisterInvariants(_ sdk.InvariantRegistry) {}

// InitGenesis performs the capability module's genesis initialization It returns
// no validator updates.
func (am AppModule) InitGenesis(ctx sdk.Context, cdc codec.JSONCodec, gs json.RawMessage) []abci.ValidatorUpdate {
	var genState types.GenesisState
	// Initialize global index to index in genesis state
	cdc.MustUnmarshalJSON(gs, &genState)

	InitGenesis(ctx, am.keeper, genState)

	return []abci.ValidatorUpdate{}
}

// ExportGenesis returns the capability module's exported genesis state as raw JSON bytes.
func (am AppModule) ExportGenesis(ctx sdk.Context, cdc codec.JSONCodec) json.RawMessage {
	genState := ExportGenesis(ctx, am.keeper)
	return cdc.MustMarshalJSON(genState)
}

// ConsensusVersion implements ConsensusVersion.
func (AppModule) ConsensusVersion() uint64 { return 2 }

// BeginBlock executes all ABCI BeginBlock logic respective to the capability module.
func (am AppModule) BeginBlock(ctx sdk.Context, _ abci.RequestBeginBlock) {
	BeginBlocker(ctx, am.keeper)
}

// EndBlock executes all ABCI EndBlock logic respective to the capability module. It
// returns no validator updates.
func (am AppModule) EndBlock(_ sdk.Context, _ abci.RequestEndBlock) []abci.ValidatorUpdate {
	return []abci.ValidatorUpdate{}
}

// ----------------------------------------------------------------------------
// abci end blocker
// ----------------------------------------------------------------------------

func BeginBlocker(ctx sdk.Context, k keeper.Keeper) {

	// TODO - Implement the algorithm to update the current withdrable amount
	// Iterativelly collect all the lock up periods and get the respective kv store
	// for the deposited amount on a given day.
	// lockup : lockupPeriods
	// 	calc : previousDay = today - lockupPeriod
	// 	get amount from key previousDay/lockupPeriod/denom
	// 	if amount != 0 ; add amount to the lockupWithdrable

	logger := k.Logger(ctx)
	logger.Info(fmt.Sprintf("Entered Qbank BeginBlocker|modulename=%s|blockheight=%d", types.ModuleName, ctx.BlockHeight()))

	// Logic - Iterate over EpochLockup part of CreateEpochLockupUserDenomDepositKey
	// Get K = UserDenom, V = sdk.coin

	prefix := types.UserDenomDepositKBP
	// TODO : Assume one block as one epoch day for now. Reduce 7 to get block last 7th block
	pk := types.CreateEpochLockupUserKey(uint64(ctx.BlockHeight()-7), types.LockupTypes_Days_7, "/")
	prefix = append(prefix, pk...)
	// prefixkey := types.CreateEpochLockupUserKey(uint64(ctx.BlockHeight()), types.LockupTypes_Days_7, "/")

	/////////////////////////////////////////////////////

	// ITERATOR #1
	iterator := func(key []byte, val sdk.Coin) error {
		// iterator := func(key, val []byte) error {
		logger.Info(fmt.Sprintf("Qbank BeginBlocker Callbackfunction|Iterator|modulename=%s|blockheight=%d|Key=%v|Value=%v",
			types.ModuleName, ctx.BlockHeight(), string(key), val))

		return nil
	}

	err := k.Iterate(ctx, prefix, iterator)
	if err != nil {
		panic(err)
	}

	/*
		// ITERATOR #2
		// closure function for updating withdrable amount
		withdrableClosure := func(key, val []byte) error {
			var coin sdk.Coin
			k.GetCdc().MustUnmarshal(val, &coin)
			logger.Info(fmt.Sprintf("Qbank BeginBlocker Callbackfunction|Iterator2|modulename=%s|blockheight=%d|Key=%v|Value=%v",
				types.ModuleName, ctx.BlockHeight(), string(key), coin))

			return nil
		}
	*/

	// TODO | AUDIT | This logic for the Withdrable amount to changed based on the new Orion vault
	// receipt token design.
	for lockupEnm, lockupStr := range types.LockupTypes_name {

		prefix := types.UserDenomDepositKBP
		// TODO : Assume one block as one epoch day for now. Reduce 7 to get block last 7th block
		lockupdays := int64(types.Lockupdays[lockupStr])
		pk := types.CreateEpochLockupUserKey(uint64(ctx.BlockHeight()-lockupdays), types.LockupTypes(lockupEnm), "/")
		prefix = append(prefix, pk...)

		logger.Info(fmt.Sprintf("Qbank BeginBlocker LockupTypes_name|k=%d|v=%s|modulename=%s|blockheight=%d|prefix=%s",
			lockupEnm, lockupStr, types.ModuleName, ctx.BlockHeight(), string(prefix)))

		err2 := k.ProcessWithdrable(ctx, prefix)
		if err2 != nil {
			panic(err2)
		}

	}

	// iterate(ctx sdk.Context, prefix []byte, cb func(key, val []byte) error) error

	// k.Iterate(ctx, []byte(""), interface{})

}
