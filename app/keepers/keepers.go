package keepers

import (
	"fmt"
	"github.com/cosmos/ibc-go/v8/modules/apps/27-interchain-accounts/controller"
	"os"

	"cosmossdk.io/log"
	storetypes "cosmossdk.io/store/types"
	evidencekeeper "cosmossdk.io/x/evidence/keeper"
	evidencetypes "cosmossdk.io/x/evidence/types"
	"cosmossdk.io/x/feegrant"
	feegrantkeeper "cosmossdk.io/x/feegrant/keeper"
	upgradekeeper "cosmossdk.io/x/upgrade/keeper"
	upgradetypes "cosmossdk.io/x/upgrade/types"
	"github.com/CosmWasm/wasmd/x/wasm"
	wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	"github.com/cosmos/cosmos-sdk/baseapp"
	"github.com/cosmos/cosmos-sdk/codec"
	"github.com/cosmos/cosmos-sdk/codec/address"
	"github.com/cosmos/cosmos-sdk/runtime"
	servertypes "github.com/cosmos/cosmos-sdk/server/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	authztypes "github.com/cosmos/cosmos-sdk/x/authz"
	authzkeeper "github.com/cosmos/cosmos-sdk/x/authz/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
	consensusparamkeeper "github.com/cosmos/cosmos-sdk/x/consensus/keeper"
	consensusparamtypes "github.com/cosmos/cosmos-sdk/x/consensus/types"
	crisiskeeper "github.com/cosmos/cosmos-sdk/x/crisis/keeper"
	crisistypes "github.com/cosmos/cosmos-sdk/x/crisis/types"
	distrkeeper "github.com/cosmos/cosmos-sdk/x/distribution/keeper"
	distrtypes "github.com/cosmos/cosmos-sdk/x/distribution/types"
	govkeeper "github.com/cosmos/cosmos-sdk/x/gov/keeper"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	govv1beta1 "github.com/cosmos/cosmos-sdk/x/gov/types/v1beta1"
	mintkeeper "github.com/cosmos/cosmos-sdk/x/mint/keeper"
	minttypes "github.com/cosmos/cosmos-sdk/x/mint/types"
	"github.com/cosmos/cosmos-sdk/x/params"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	paramstypes "github.com/cosmos/cosmos-sdk/x/params/types"
	paramproposal "github.com/cosmos/cosmos-sdk/x/params/types/proposal"
	slashingkeeper "github.com/cosmos/cosmos-sdk/x/slashing/keeper"
	slashingtypes "github.com/cosmos/cosmos-sdk/x/slashing/types"
	stakingkeeper "github.com/cosmos/cosmos-sdk/x/staking/keeper"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"
	pfmrouter "github.com/cosmos/ibc-apps/middleware/packet-forward-middleware/v8/packetforward"
	packetforwardkeeper "github.com/cosmos/ibc-apps/middleware/packet-forward-middleware/v8/packetforward/keeper"
	pfmtypes "github.com/cosmos/ibc-apps/middleware/packet-forward-middleware/v8/packetforward/types"
	icqtypes "github.com/cosmos/ibc-apps/modules/async-icq/v8/types"
	ibchooks "github.com/cosmos/ibc-apps/modules/ibc-hooks/v8"
	ibchookskeeper "github.com/cosmos/ibc-apps/modules/ibc-hooks/v8/keeper"
	ibchookstypes "github.com/cosmos/ibc-apps/modules/ibc-hooks/v8/types"
	ratelimit "github.com/cosmos/ibc-apps/modules/rate-limiting/v8"
	ratelimitkeeper "github.com/cosmos/ibc-apps/modules/rate-limiting/v8/keeper"
	ratelimittypes "github.com/cosmos/ibc-apps/modules/rate-limiting/v8/types"
	capabilitykeeper "github.com/cosmos/ibc-go/modules/capability/keeper"
	capabilitytypes "github.com/cosmos/ibc-go/modules/capability/types"
	ibcwasmkeeper "github.com/cosmos/ibc-go/modules/light-clients/08-wasm/keeper"
	ibcwasmtypes "github.com/cosmos/ibc-go/modules/light-clients/08-wasm/types"
	icacontrollerkeeper "github.com/cosmos/ibc-go/v8/modules/apps/27-interchain-accounts/controller/keeper"
	icacontrollertypes "github.com/cosmos/ibc-go/v8/modules/apps/27-interchain-accounts/controller/types"
	icahost "github.com/cosmos/ibc-go/v8/modules/apps/27-interchain-accounts/host"
	icahostkeeper "github.com/cosmos/ibc-go/v8/modules/apps/27-interchain-accounts/host/keeper"
	icahosttypes "github.com/cosmos/ibc-go/v8/modules/apps/27-interchain-accounts/host/types"
	"github.com/cosmos/ibc-go/v8/modules/apps/transfer"
	ibctransferkeeper "github.com/cosmos/ibc-go/v8/modules/apps/transfer/keeper"
	ibctransfertypes "github.com/cosmos/ibc-go/v8/modules/apps/transfer/types"
	ibcclient "github.com/cosmos/ibc-go/v8/modules/core/02-client"
	ibcclienttypes "github.com/cosmos/ibc-go/v8/modules/core/02-client/types"
	ibcporttypes "github.com/cosmos/ibc-go/v8/modules/core/05-port/types"
	ibchost "github.com/cosmos/ibc-go/v8/modules/core/exported"
	ibckeeper "github.com/cosmos/ibc-go/v8/modules/core/keeper"
	appparams "github.com/quasar-finance/quasar/app/params"
	"github.com/quasar-finance/quasar/wasmbindings"
	epochsmodulekeeper "github.com/quasar-finance/quasar/x/epochs/keeper"
	epochsmoduletypes "github.com/quasar-finance/quasar/x/epochs/types"
	tfbindings "github.com/quasar-finance/quasar/x/tokenfactory/bindings"
	tfkeeper "github.com/quasar-finance/quasar/x/tokenfactory/keeper"
	tfmodulekeeper "github.com/quasar-finance/quasar/x/tokenfactory/keeper"
	tftypes "github.com/quasar-finance/quasar/x/tokenfactory/types"
	feemarketkeeper "github.com/skip-mev/feemarket/x/feemarket/keeper"
	feemarkettypes "github.com/skip-mev/feemarket/x/feemarket/types"
)

type AppKeepers struct {
	// Special keepers
	ParamsKeeper          paramskeeper.Keeper
	CapabilityKeeper      *capabilitykeeper.Keeper
	CrisisKeeper          *crisiskeeper.Keeper
	UpgradeKeeper         *upgradekeeper.Keeper
	ConsensusParamsKeeper *consensusparamkeeper.Keeper

	// make scoped keepers public for test purposes
	ScopedIBCKeeper           capabilitykeeper.ScopedKeeper
	ScopedTransferKeeper      capabilitykeeper.ScopedKeeper
	ScopedICAControllerKeeper capabilitykeeper.ScopedKeeper
	ScopedICAHostKeeper       capabilitykeeper.ScopedKeeper
	ScopedIntergammKeeper     capabilitykeeper.ScopedKeeper
	ScopedQOsmosisKeeper      capabilitykeeper.ScopedKeeper
	ScopedWasmKeeper          capabilitykeeper.ScopedKeeper
	ScopedICQKeeper           capabilitykeeper.ScopedKeeper

	// "Normal" keepers
	AccountKeeper       authkeeper.AccountKeeper
	BankKeeper          bankkeeper.Keeper
	StakingKeeper       *stakingkeeper.Keeper
	SlashingKeeper      slashingkeeper.Keeper
	MintKeeper          mintkeeper.Keeper
	DistrKeeper         distrkeeper.Keeper
	GovKeeper           govkeeper.Keeper
	IBCKeeper           *ibckeeper.Keeper // IBC Keeper must be a pointer in the app, so we can SetRouter on it correctly
	IBCHooksKeeper      *ibchookskeeper.Keeper
	EvidenceKeeper      evidencekeeper.Keeper
	TransferKeeper      ibctransferkeeper.Keeper
	IBCWasmClientKeeper *ibcwasmkeeper.Keeper
	FeeGrantKeeper      feegrantkeeper.Keeper
	WasmKeeper          *wasmkeeper.Keeper
	ContractKeeper      *wasmkeeper.PermissionedKeeper
	EpochsKeeper        *epochsmodulekeeper.Keeper
	TfKeeper            tfmodulekeeper.Keeper
	AuthzKeeper         authzkeeper.Keeper
	ICAControllerKeeper icacontrollerkeeper.Keeper
	ICAHostKeeper       *icahostkeeper.Keeper
	FeeMarketKeeper     *feemarketkeeper.Keeper

	PFMRouterKeeper *packetforwardkeeper.Keeper
	RatelimitKeeper ratelimitkeeper.Keeper

	// IBC modules
	// transfer module
	RateLimitModule  ratelimit.AppModule
	TransferStack    *ibchooks.IBCMiddleware
	TransferModule   transfer.AppModule
	Ics20WasmHooks   *ibchooks.WasmHooks
	HooksICS4Wrapper ibchooks.ICS4Middleware
	PFMRouterModule  pfmrouter.AppModule

	// keys to access the substores
	keys    map[string]*storetypes.KVStoreKey
	tkeys   map[string]*storetypes.TransientStoreKey
	memKeys map[string]*storetypes.MemoryStoreKey
}

// InitSpecialKeepers initiates special keepers (crisis appkeeper, upgradekeeper, params keeper)
func (appKeepers *AppKeepers) InitSpecialKeepers(
	appCodec codec.Codec,
	bApp *baseapp.BaseApp,
	cdc *codec.LegacyAmino,
	invCheckPeriod uint,
	skipUpgradeHeights map[int64]bool,
	homePath string,
	logger log.Logger,
	appOpts servertypes.AppOptions,
) {
	appKeepers.GenerateKeys()

	/*
		configure state listening capabilities using AppOptions
		we are doing nothing with the returned streamingServices and waitGroup in this case
	*/
	// load state streaming if enabled

	if err := bApp.RegisterStreamingServices(appOpts, appKeepers.keys); err != nil {
		logger.Error("failed to load state streaming", "err", err)
		os.Exit(1)
	}

	appKeepers.ParamsKeeper = appKeepers.initParamsKeeper(appCodec, cdc, appKeepers.keys[paramstypes.StoreKey], appKeepers.tkeys[paramstypes.TStoreKey])

	consensusParamsKeeper := consensusparamkeeper.NewKeeper(appCodec,
		runtime.NewKVStoreService(appKeepers.keys[consensusparamtypes.StoreKey]),
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
		runtime.EventService{})

	appKeepers.ConsensusParamsKeeper = &consensusParamsKeeper
	bApp.SetParamStore(appKeepers.ConsensusParamsKeeper.ParamsStore)

	// add capability keeper and ScopeToModule for ibc module
	appKeepers.CapabilityKeeper = capabilitykeeper.NewKeeper(appCodec, appKeepers.keys[capabilitytypes.StoreKey], appKeepers.memKeys[capabilitytypes.MemStoreKey])

	// grant capabilities for the ibc and ibc-transfer modules
	appKeepers.ScopedIBCKeeper = appKeepers.CapabilityKeeper.ScopeToModule(ibchost.ModuleName)
	appKeepers.ScopedTransferKeeper = appKeepers.CapabilityKeeper.ScopeToModule(ibctransfertypes.ModuleName)
	appKeepers.ScopedWasmKeeper = appKeepers.CapabilityKeeper.ScopeToModule(wasmtypes.ModuleName)
	// appKeepers.ScopedQOsmosisKeeper = appKeepers.CapabilityKeeper.ScopeToModule(qosmotypes.SubModuleName)
	appKeepers.ScopedICAControllerKeeper = appKeepers.CapabilityKeeper.ScopeToModule(icacontrollertypes.SubModuleName)
	appKeepers.ScopedICAHostKeeper = appKeepers.CapabilityKeeper.ScopeToModule(icahosttypes.SubModuleName)
	appKeepers.ScopedICQKeeper = appKeepers.CapabilityKeeper.ScopeToModule(icqtypes.ModuleName)
	appKeepers.CapabilityKeeper.Seal()

	appKeepers.CrisisKeeper = crisiskeeper.NewKeeper(
		appCodec,
		runtime.NewKVStoreService(appKeepers.keys[crisistypes.StoreKey]),
		invCheckPeriod,
		appKeepers.BankKeeper,
		authtypes.FeeCollectorName,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
		address.NewBech32Codec(sdk.GetConfig().GetBech32AccountAddrPrefix()),
	)

	appKeepers.UpgradeKeeper = upgradekeeper.NewKeeper(
		skipUpgradeHeights,
		runtime.NewKVStoreService(appKeepers.keys[upgradetypes.StoreKey]),
		appCodec,
		homePath,
		bApp,
		authtypes.NewModuleAddress(govtypes.ModuleName).String())
}

// InitNormalKeepers initializes all 'normal' keepers (account, app, bank, auth, staking, distribution, slashing, transfer, IBC router, governance, mint keepers).
func (appKeepers *AppKeepers) InitNormalKeepers(
	appCodec codec.Codec,
	bApp *baseapp.BaseApp,
	legacyAmino *codec.LegacyAmino,
	maccPerms map[string][]string,
	dataDir string,
	wasmDir string,
	wasmConfig wasmtypes.WasmConfig,
	wasmOpts []wasmkeeper.Option,
	blockedAddress map[string]bool,
	ibcWasmConfig ibcwasmtypes.WasmConfig,
) {
	accountKeeper := authkeeper.NewAccountKeeper(
		appCodec,
		runtime.NewKVStoreService(appKeepers.keys[authtypes.StoreKey]),
		authtypes.ProtoBaseAccount,
		maccPerms,
		address.NewBech32Codec(sdk.GetConfig().GetBech32AccountAddrPrefix()),
		sdk.GetConfig().GetBech32AccountAddrPrefix(),
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)

	appKeepers.AccountKeeper = accountKeeper

	bankKeeper := bankkeeper.NewBaseKeeper(
		appCodec,
		runtime.NewKVStoreService(appKeepers.keys[banktypes.StoreKey]),
		appKeepers.AccountKeeper,
		blockedAddress,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
		bApp.Logger(),
	)
	appKeepers.BankKeeper = bankKeeper

	stakingKeeper := stakingkeeper.NewKeeper(
		appCodec,
		runtime.NewKVStoreService(appKeepers.keys[stakingtypes.StoreKey]),
		appKeepers.AccountKeeper,
		appKeepers.BankKeeper,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
		address.NewBech32Codec(appparams.Bech32PrefixValAddr),
		address.NewBech32Codec(appparams.Bech32PrefixConsAddr),
	)
	appKeepers.StakingKeeper = stakingKeeper

	appKeepers.FeeMarketKeeper = feemarketkeeper.NewKeeper(
		appCodec,
		appKeepers.keys[feemarkettypes.StoreKey],
		appKeepers.AccountKeeper,
		&DefaultFeemarketDenomResolver{},
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)

	mintKeeper := mintkeeper.NewKeeper(
		appCodec,
		runtime.NewKVStoreService(appKeepers.keys[minttypes.StoreKey]),
		appKeepers.StakingKeeper,
		appKeepers.AccountKeeper,
		appKeepers.BankKeeper,
		authtypes.FeeCollectorName,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)
	appKeepers.MintKeeper = mintKeeper

	distrKeeper := distrkeeper.NewKeeper(
		appCodec,
		runtime.NewKVStoreService(appKeepers.keys[distrtypes.StoreKey]),
		appKeepers.AccountKeeper,
		appKeepers.BankKeeper,
		appKeepers.StakingKeeper,
		authtypes.FeeCollectorName,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)
	appKeepers.DistrKeeper = distrKeeper

	slashingKeeper := slashingkeeper.NewKeeper(
		appCodec,
		legacyAmino,
		runtime.NewKVStoreService(appKeepers.keys[slashingtypes.StoreKey]),
		appKeepers.StakingKeeper,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)
	appKeepers.SlashingKeeper = slashingKeeper

	feeGrantKeeper := feegrantkeeper.NewKeeper(
		appCodec,
		runtime.NewKVStoreService(appKeepers.keys[feegrant.StoreKey]),
		appKeepers.AccountKeeper)
	appKeepers.FeeGrantKeeper = feeGrantKeeper

	// register the staking hooks
	// NOTE: stakingKeeper above is passed by reference, so that it will contain these hooks
	appKeepers.StakingKeeper.SetHooks(
		stakingtypes.NewMultiStakingHooks(
			appKeepers.DistrKeeper.Hooks(),
			appKeepers.SlashingKeeper.Hooks(),
		),
	)

	// Create IBC Keeper
	appKeepers.IBCKeeper = ibckeeper.NewKeeper(
		appCodec,
		appKeepers.keys[ibchost.StoreKey],
		appKeepers.GetSubspace(ibchost.ModuleName),
		appKeepers.StakingKeeper,
		appKeepers.UpgradeKeeper,
		appKeepers.ScopedIBCKeeper,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)

	// Configure the hooks keeper
	hooksKeeper := ibchookskeeper.NewKeeper(
		appKeepers.keys[ibchookstypes.StoreKey],
	)
	appKeepers.IBCHooksKeeper = &hooksKeeper

	wasmHooks := ibchooks.NewWasmHooks(appKeepers.IBCHooksKeeper, nil, appparams.Bech32PrefixAccAddr)
	appKeepers.Ics20WasmHooks = &wasmHooks

	// Configure the rate limit keeper
	rateLimitKeeper := *ratelimitkeeper.NewKeeper(
		appCodec,
		runtime.NewKVStoreService(appKeepers.keys[ratelimittypes.StoreKey]),
		appKeepers.GetSubspace(ratelimittypes.ModuleName),
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
		appKeepers.BankKeeper,
		appKeepers.IBCKeeper.ChannelKeeper,
		appKeepers.IBCKeeper.ChannelKeeper, // ICS4Wrapper
	)

	appKeepers.RatelimitKeeper = rateLimitKeeper

	ibcWasmClientKeeper := ibcwasmkeeper.NewKeeperWithConfig(
		appCodec,
		runtime.NewKVStoreService(appKeepers.keys[ibcwasmtypes.StoreKey]),
		appKeepers.IBCKeeper.ClientKeeper,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
		ibcWasmConfig,
		bApp.GRPCQueryRouter(),
	)

	appKeepers.IBCWasmClientKeeper = &ibcWasmClientKeeper

	// PFMRouterKeeper must be created before TransferKeeper
	appKeepers.PFMRouterKeeper = packetforwardkeeper.NewKeeper(
		appCodec,
		appKeepers.keys[pfmtypes.StoreKey],
		nil, // Will be zero-value here. Reference is set later on with SetTransferKeeper.
		appKeepers.IBCKeeper.ChannelKeeper,
		appKeepers.DistrKeeper,
		appKeepers.BankKeeper,
		appKeepers.RatelimitKeeper, // ICS4Wrapper
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)

	transferKeeper := ibctransferkeeper.NewKeeper(
		appCodec,
		appKeepers.keys[ibctransfertypes.StoreKey],
		appKeepers.GetSubspace(ibctransfertypes.ModuleName),
		appKeepers.PFMRouterKeeper, // ICS4 Wrapper: PFM Router middleware
		appKeepers.IBCKeeper.ChannelKeeper,
		appKeepers.IBCKeeper.PortKeeper,
		appKeepers.AccountKeeper,
		appKeepers.BankKeeper,
		appKeepers.ScopedTransferKeeper,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)

	appKeepers.TransferKeeper = transferKeeper
	// Must be called on PFMRouter AFTER TransferKeeper initialized
	appKeepers.PFMRouterKeeper.SetTransferKeeper(appKeepers.TransferKeeper)

	appKeepers.ICAControllerKeeper = icacontrollerkeeper.NewKeeper(
		appCodec, appKeepers.keys[icacontrollertypes.StoreKey],
		appKeepers.GetSubspace(icacontrollertypes.SubModuleName),
		appKeepers.IBCKeeper.ChannelKeeper, // may be replaced with middleware such as ics29 fee
		appKeepers.IBCKeeper.ChannelKeeper,
		appKeepers.IBCKeeper.PortKeeper,
		appKeepers.ScopedICAControllerKeeper,
		bApp.MsgServiceRouter(),
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)

	icaHostKeeper := icahostkeeper.NewKeeper(
		appCodec,
		appKeepers.keys[icahosttypes.StoreKey],
		appKeepers.GetSubspace(icahosttypes.SubModuleName),
		appKeepers.IBCKeeper.ChannelKeeper, // can be replaced with rate limiter ICS 4 Wrapper
		appKeepers.IBCKeeper.ChannelKeeper,
		appKeepers.IBCKeeper.PortKeeper,
		appKeepers.AccountKeeper,
		appKeepers.ScopedICAHostKeeper,
		bApp.MsgServiceRouter(),
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)
	icaHostKeeper.WithQueryRouter(bApp.GRPCQueryRouter())
	appKeepers.ICAHostKeeper = &icaHostKeeper

	evidenceKeeper := evidencekeeper.NewKeeper(
		appCodec,
		runtime.NewKVStoreService(appKeepers.keys[evidencetypes.StoreKey]),
		appKeepers.StakingKeeper, appKeepers.SlashingKeeper,
		address.NewBech32Codec(sdk.GetConfig().GetBech32AccountAddrPrefix()),
		runtime.ProvideCometInfoService(),
	)

	appKeepers.EvidenceKeeper = *evidenceKeeper

	// TODO - SDK 50
	govRouter := govv1beta1.NewRouter()
	govRouter.
		AddRoute(govtypes.RouterKey, govv1beta1.ProposalHandler).
		AddRoute(paramproposal.RouterKey, params.NewParamChangeProposalHandler(appKeepers.ParamsKeeper)).
		// AddRoute(upgradetypes.RouterKey, upgrade.NewSoftwareUpgradeProposalHandler(appKeepers.UpgradeKeeper)).
		// AddRoute(ibcclienttypes.RouterKey, ibcclient.NewClientProposalHandler(appKeepers.IBCKeeper.ClientKeeper)).
		AddRoute(ibchost.RouterKey, ibcclient.NewClientProposalHandler(appKeepers.IBCKeeper.ClientKeeper))

	govConfig := govtypes.DefaultConfig()
	govKeeper := govkeeper.NewKeeper(
		appCodec,
		runtime.NewKVStoreService(appKeepers.keys[govtypes.StoreKey]),
		// appKeepers.keys[govtypes.StoreKey],
		appKeepers.AccountKeeper,
		appKeepers.BankKeeper,
		appKeepers.StakingKeeper,
		appKeepers.DistrKeeper,
		bApp.MsgServiceRouter(),
		govConfig,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)

	appKeepers.GovKeeper = *govKeeper
	govKeeper.SetLegacyRouter(govRouter)
	appKeepers.GovKeeper = *govKeeper.SetHooks(
		govtypes.NewMultiGovHooks(
		// register the governance hooks
		),
	)

	appKeepers.EpochsKeeper = epochsmodulekeeper.NewKeeper(appCodec, appKeepers.keys[epochsmoduletypes.StoreKey])
	appKeepers.ContractKeeper = wasmkeeper.NewDefaultPermissionKeeper(appKeepers.WasmKeeper)

	// Authz
	appKeepers.AuthzKeeper = authzkeeper.NewKeeper(
		runtime.NewKVStoreService(appKeepers.keys[authzkeeper.StoreKey]),
		appCodec,
		bApp.MsgServiceRouter(),
		appKeepers.AccountKeeper,
	)

	// Set epoch hooks
	appKeepers.EpochsKeeper.SetHooks(
		epochsmoduletypes.NewMultiEpochHooks(
		// appKeepers.QOsmosisKeeper.EpochHooks(),
		),
	)

	/// Token factory Module
	appKeepers.TfKeeper = tfkeeper.NewKeeper(
		appKeepers.keys[tftypes.StoreKey],
		appKeepers.GetSubspace(tftypes.ModuleName),
		maccPerms,
		appKeepers.AccountKeeper,
		appKeepers.BankKeeper,
		appKeepers.DistrKeeper,
	)

	// set token factory contract keeper
	appKeepers.TfKeeper.SetContractKeeper(appKeepers.ContractKeeper)

	// TODO - SDK 50
	// callback := owasm.NewCallbackPlugin(appKeepers.WasmKeeper, appKeepers.QTransferKeeper.GetQTransferAcc())

	tmpBankBaseKeeper := appKeepers.BankKeeper.(bankkeeper.BaseKeeper)

	wasmOpts = append(tfbindings.RegisterCustomPlugins(&tmpBankBaseKeeper, &appKeepers.TfKeeper), wasmOpts...)

	queryPlugins := wasmkeeper.WithQueryPlugins(
		&wasmkeeper.QueryPlugins{
			Stargate: wasmkeeper.AcceptListStargateQuerier(wasmbindings.AcceptedStargateQueries(), bApp.GRPCQueryRouter(), appCodec),
			Grpc:     wasmkeeper.AcceptListGrpcQuerier(wasmbindings.AcceptedStargateQueries(), bApp.GRPCQueryRouter(), appCodec),
		})

	wasmOpts = append(wasmOpts, queryPlugins)

	wasmKeeper := wasmkeeper.NewKeeper(
		appCodec,
		runtime.NewKVStoreService(appKeepers.keys[wasmtypes.StoreKey]),
		appKeepers.AccountKeeper,
		appKeepers.BankKeeper,
		appKeepers.StakingKeeper,
		distrkeeper.NewQuerier(appKeepers.DistrKeeper),
		appKeepers.IBCKeeper.ChannelKeeper,
		appKeepers.IBCKeeper.ChannelKeeper,
		appKeepers.IBCKeeper.PortKeeper,
		appKeepers.ScopedWasmKeeper,
		appKeepers.TransferKeeper,
		bApp.MsgServiceRouter(),
		bApp.GRPCQueryRouter(),
		wasmDir,
		wasmConfig,
		wasmkeeper.BuiltInCapabilities(),
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
		wasmOpts...,
	)

	appKeepers.WasmKeeper = &wasmKeeper

	//IBC hooks
	appKeepers.Ics20WasmHooks.ContractKeeper = appKeepers.WasmKeeper
	appKeepers.HooksICS4Wrapper = ibchooks.NewICS4Middleware(
		appKeepers.IBCKeeper.ChannelKeeper,
		appKeepers.Ics20WasmHooks,
	)

	//Middleware Stacks
	appKeepers.TransferModule = transfer.NewAppModule(appKeepers.TransferKeeper)
	appKeepers.PFMRouterModule = pfmrouter.NewAppModule(appKeepers.PFMRouterKeeper, appKeepers.GetSubspace(pfmtypes.ModuleName))
	appKeepers.RateLimitModule = ratelimit.NewAppModule(appCodec, appKeepers.RatelimitKeeper)

	// Create Transfer Stack (from bottom to top of stack)
	// - core IBC
	// - ibcfee
	// - ratelimit
	// - pfm
	// - provider (not required for us)
	// - transfer
	//
	// This is how transfer stack will work in the end:
	// * RecvPacket -> IBC core -> Fee -> RateLimit -> PFM -> Provider -> Transfer (AddRoute)
	// * SendPacket -> Transfer -> Provider -> PFM -> RateLimit -> Fee -> IBC core (ICS4Wrapper)

	var transferStack ibcporttypes.IBCModule
	transferStack = transfer.NewIBCModule(appKeepers.TransferKeeper)
	transferStack = pfmrouter.NewIBCMiddleware(
		transferStack,
		appKeepers.PFMRouterKeeper,
		0, // retries on timeout
		packetforwardkeeper.DefaultForwardTransferPacketTimeoutTimestamp,
		packetforwardkeeper.DefaultRefundTransferPacketTimeoutTimestamp,
	)
	transferStack = ratelimit.NewIBCMiddleware(appKeepers.RatelimitKeeper, transferStack)

	// hooks middleware
	hooksTransferModule := ibchooks.NewIBCMiddleware(transferStack, &appKeepers.HooksICS4Wrapper)
	appKeepers.TransferStack = &hooksTransferModule

	ibcRouter := ibcporttypes.NewRouter()

	// Register host and authentication routes
	ibcRouter.
		AddRoute(icahosttypes.SubModuleName, icahost.NewIBCModule(*appKeepers.ICAHostKeeper)).
		AddRoute(ibctransfertypes.ModuleName, appKeepers.TransferStack).
		AddRoute(wasmtypes.ModuleName, wasm.NewIBCHandler(appKeepers.WasmKeeper,
			appKeepers.IBCKeeper.ChannelKeeper, appKeepers.IBCKeeper.ChannelKeeper)).
		AddRoute(icacontrollertypes.SubModuleName, controller.NewIBCMiddleware(nil, appKeepers.ICAControllerKeeper))

	appKeepers.IBCKeeper.SetRouter(ibcRouter)
}

// initParamsKeeper init params keeper and its subspaces
func (appKeepers *AppKeepers) initParamsKeeper(appCodec codec.BinaryCodec, legacyAmino *codec.LegacyAmino, key, tkey storetypes.StoreKey) paramskeeper.Keeper {
	paramsKeeper := paramskeeper.NewKeeper(appCodec, legacyAmino, key, tkey)
	keyTable := ibcclienttypes.ParamKeyTable()
	paramsKeeper.Subspace(authtypes.ModuleName)
	paramsKeeper.Subspace(banktypes.ModuleName)
	paramsKeeper.Subspace(stakingtypes.ModuleName)
	paramsKeeper.Subspace(minttypes.ModuleName)
	paramsKeeper.Subspace(distrtypes.ModuleName)
	paramsKeeper.Subspace(slashingtypes.ModuleName)
	paramsKeeper.Subspace(govtypes.ModuleName)
	paramsKeeper.Subspace(crisistypes.ModuleName)
	paramsKeeper.Subspace(ibctransfertypes.ModuleName).WithKeyTable(ibctransfertypes.ParamKeyTable())
	paramsKeeper.Subspace(ibchost.ModuleName).WithKeyTable(keyTable)
	paramsKeeper.Subspace(epochsmoduletypes.ModuleName)
	paramsKeeper.Subspace(icacontrollertypes.SubModuleName).WithKeyTable(icacontrollertypes.ParamKeyTable())
	paramsKeeper.Subspace(icqtypes.ModuleName)
	paramsKeeper.Subspace(icahosttypes.SubModuleName).WithKeyTable(icahosttypes.ParamKeyTable())
	paramsKeeper.Subspace(wasmtypes.ModuleName)
	paramsKeeper.Subspace(tftypes.ModuleName)
	paramsKeeper.Subspace(authztypes.ModuleName)
	paramsKeeper.Subspace(ratelimittypes.ModuleName).WithKeyTable(ratelimittypes.ParamKeyTable())
	paramsKeeper.Subspace(pfmtypes.ModuleName).WithKeyTable(pfmtypes.ParamKeyTable())
	paramsKeeper.Subspace(ibchookstypes.ModuleName)

	return paramsKeeper
}

func KVStoreKeys() []string {
	return []string{
		authtypes.StoreKey,
		banktypes.StoreKey,
		stakingtypes.StoreKey,
		minttypes.StoreKey,
		distrtypes.StoreKey,
		slashingtypes.StoreKey,
		govtypes.StoreKey,
		paramstypes.StoreKey,
		ibchost.StoreKey,
		upgradetypes.StoreKey,
		feegrant.StoreKey,
		evidencetypes.StoreKey,
		ibctransfertypes.StoreKey,
		capabilitytypes.StoreKey,
		epochsmoduletypes.StoreKey,
		icacontrollertypes.StoreKey,
		icahosttypes.StoreKey,
		icqtypes.StoreKey,
		ibcwasmtypes.StoreKey,
		wasmtypes.StoreKey,
		tftypes.StoreKey,
		authzkeeper.StoreKey,
		consensusparamtypes.StoreKey,
		crisistypes.StoreKey,
		feemarkettypes.StoreKey,
		ibchookstypes.StoreKey,
		ratelimittypes.StoreKey,
		pfmtypes.StoreKey,
	}
}

type DefaultFeemarketDenomResolver struct{}

func (r *DefaultFeemarketDenomResolver) ConvertToDenom(_ sdk.Context, coin sdk.DecCoin, denom string) (sdk.DecCoin, error) {
	if coin.Denom == denom {
		return coin, nil
	}

	return sdk.DecCoin{}, fmt.Errorf("error resolving denom")
}

func (r *DefaultFeemarketDenomResolver) ExtraDenoms(_ sdk.Context) ([]string, error) {
	return []string{}, nil
}
