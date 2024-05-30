package keepers

import (
	"github.com/CosmWasm/wasmd/x/wasm"
	wasmkeeper "github.com/CosmWasm/wasmd/x/wasm/keeper"
	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
	"github.com/cosmos/cosmos-sdk/baseapp"
	"github.com/cosmos/cosmos-sdk/codec"
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	authztypes "github.com/cosmos/cosmos-sdk/x/authz"
	authzkeeper "github.com/cosmos/cosmos-sdk/x/authz/keeper"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
	capabilitykeeper "github.com/cosmos/cosmos-sdk/x/capability/keeper"
	capabilitytypes "github.com/cosmos/cosmos-sdk/x/capability/types"
	consensusparamtypes "github.com/cosmos/cosmos-sdk/x/consensus/types"
	crisiskeeper "github.com/cosmos/cosmos-sdk/x/crisis/keeper"
	crisistypes "github.com/cosmos/cosmos-sdk/x/crisis/types"
	distrkeeper "github.com/cosmos/cosmos-sdk/x/distribution/keeper"
	distrtypes "github.com/cosmos/cosmos-sdk/x/distribution/types"
	evidencekeeper "github.com/cosmos/cosmos-sdk/x/evidence/keeper"
	evidencetypes "github.com/cosmos/cosmos-sdk/x/evidence/types"
	"github.com/cosmos/cosmos-sdk/x/feegrant"
	feegrantkeeper "github.com/cosmos/cosmos-sdk/x/feegrant/keeper"
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
	"github.com/cosmos/cosmos-sdk/x/upgrade"
	upgradekeeper "github.com/cosmos/cosmos-sdk/x/upgrade/keeper"
	upgradetypes "github.com/cosmos/cosmos-sdk/x/upgrade/types"
	icqtypes "github.com/cosmos/ibc-apps/modules/async-icq/v7/types"
	ibcwasmtypes "github.com/cosmos/ibc-go/modules/light-clients/08-wasm/types"
	icacontrollerkeeper "github.com/cosmos/ibc-go/v7/modules/apps/27-interchain-accounts/controller/keeper"
	icacontrollertypes "github.com/cosmos/ibc-go/v7/modules/apps/27-interchain-accounts/controller/types"
	icahost "github.com/cosmos/ibc-go/v7/modules/apps/27-interchain-accounts/host"
	icahostkeeper "github.com/cosmos/ibc-go/v7/modules/apps/27-interchain-accounts/host/keeper"
	icahosttypes "github.com/cosmos/ibc-go/v7/modules/apps/27-interchain-accounts/host/types"
	"github.com/cosmos/ibc-go/v7/modules/apps/transfer"
	ibctransferkeeper "github.com/cosmos/ibc-go/v7/modules/apps/transfer/keeper"
	ibctransfertypes "github.com/cosmos/ibc-go/v7/modules/apps/transfer/types"
	ibcclient "github.com/cosmos/ibc-go/v7/modules/core/02-client"
	ibcclienttypes "github.com/cosmos/ibc-go/v7/modules/core/02-client/types"
	ibcporttypes "github.com/cosmos/ibc-go/v7/modules/core/05-port/types"
	ibchost "github.com/cosmos/ibc-go/v7/modules/core/exported"
	ibckeeper "github.com/cosmos/ibc-go/v7/modules/core/keeper"
	appparams "github.com/quasarlabs/quasarnode/app/params"
	owasm "github.com/quasarlabs/quasarnode/wasmbinding"
	epochsmodulekeeper "github.com/quasarlabs/quasarnode/x/epochs/keeper"
	epochsmoduletypes "github.com/quasarlabs/quasarnode/x/epochs/types"
	qoraclemodulekeeper "github.com/quasarlabs/quasarnode/x/qoracle/keeper"
	qosmo "github.com/quasarlabs/quasarnode/x/qoracle/osmosis"
	qosmokeeper "github.com/quasarlabs/quasarnode/x/qoracle/osmosis/keeper"
	qosmotypes "github.com/quasarlabs/quasarnode/x/qoracle/osmosis/types"
	qoraclemoduletypes "github.com/quasarlabs/quasarnode/x/qoracle/types"
	"github.com/quasarlabs/quasarnode/x/qtransfer"
	qtransferkeeper "github.com/quasarlabs/quasarnode/x/qtransfer/keeper"
	qtransfertypes "github.com/quasarlabs/quasarnode/x/qtransfer/types"
	qvestingmodulekeeper "github.com/quasarlabs/quasarnode/x/qvesting/keeper"
	qvestingmoduletypes "github.com/quasarlabs/quasarnode/x/qvesting/types"
	tfbindings "github.com/quasarlabs/quasarnode/x/tokenfactory/bindings"
	tfkeeper "github.com/quasarlabs/quasarnode/x/tokenfactory/keeper"
	tfmodulekeeper "github.com/quasarlabs/quasarnode/x/tokenfactory/keeper"
	tftypes "github.com/quasarlabs/quasarnode/x/tokenfactory/types"
	//consensus "github.com/cosmos/cosmos-sdk/x/consensus"
	consensusparamkeeper "github.com/cosmos/cosmos-sdk/x/consensus/keeper"
	//consensusparamtypes "github.com/cosmos/cosmos-sdk/x/consensus/types"
)

const (
	AccountAddressPrefix = "quasar"
)

type AppKeepers struct {
	// Special keepers
	ParamsKeeper          paramskeeper.Keeper
	CapabilityKeeper      *capabilitykeeper.Keeper
	CrisisKeeper          *crisiskeeper.Keeper
	UpgradeKeeper         *upgradekeeper.Keeper
	ConsensusParamsKeeper consensusparamkeeper.Keeper

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
	EvidenceKeeper      evidencekeeper.Keeper
	TransferKeeper      ibctransferkeeper.Keeper
	FeeGrantKeeper      feegrantkeeper.Keeper
	WasmKeeper          *wasmkeeper.Keeper
	QTransferKeeper     qtransferkeeper.Keeper
	EpochsKeeper        *epochsmodulekeeper.Keeper
	QOsmosisKeeper      qosmokeeper.Keeper
	QOracleKeeper       qoraclemodulekeeper.Keeper
	QVestingKeeper      qvestingmodulekeeper.Keeper
	TfKeeper            tfmodulekeeper.Keeper
	AuthzKeeper         authzkeeper.Keeper
	ICAControllerKeeper icacontrollerkeeper.Keeper
	ICAHostKeeper       icahostkeeper.Keeper

	// IBC modules
	// transfer module
	RawIcs20TransferAppModule transfer.AppModule
	TransferStack             *qtransfer.IBCMiddleware
	Ics20WasmHooks            *qtransfer.WasmHooks
	HooksICS4Wrapper          qtransfer.ICS4Middleware

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
) {
	appKeepers.GenerateKeys()
	appKeepers.ParamsKeeper = appKeepers.initParamsKeeper(appCodec, cdc, appKeepers.keys[paramstypes.StoreKey], appKeepers.tkeys[paramstypes.TStoreKey])

	// set the BaseApp's parameter store
	appKeepers.ConsensusParamsKeeper = consensusparamkeeper.NewKeeper(
		appCodec,
		appKeepers.keys[consensusparamtypes.StoreKey],
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)
	bApp.SetParamStore(&appKeepers.ConsensusParamsKeeper)

	// add capability keeper and ScopeToModule for ibc module
	appKeepers.CapabilityKeeper = capabilitykeeper.NewKeeper(appCodec, appKeepers.keys[capabilitytypes.StoreKey], appKeepers.memKeys[capabilitytypes.MemStoreKey])

	// grant capabilities for the ibc and ibc-transfer modules
	appKeepers.ScopedIBCKeeper = appKeepers.CapabilityKeeper.ScopeToModule(ibchost.ModuleName)
	appKeepers.ScopedTransferKeeper = appKeepers.CapabilityKeeper.ScopeToModule(ibctransfertypes.ModuleName)
	appKeepers.ScopedWasmKeeper = appKeepers.CapabilityKeeper.ScopeToModule(wasmtypes.ModuleName)
	appKeepers.ScopedQOsmosisKeeper = appKeepers.CapabilityKeeper.ScopeToModule(qosmotypes.SubModuleName)
	appKeepers.ScopedICAControllerKeeper = appKeepers.CapabilityKeeper.ScopeToModule(icacontrollertypes.SubModuleName)
	appKeepers.ScopedICAHostKeeper = appKeepers.CapabilityKeeper.ScopeToModule(icahosttypes.SubModuleName)
	appKeepers.ScopedICQKeeper = appKeepers.CapabilityKeeper.ScopeToModule(icqtypes.ModuleName)
	appKeepers.CapabilityKeeper.Seal()

	appKeepers.CrisisKeeper = crisiskeeper.NewKeeper(
		appCodec,
		appKeepers.keys[crisistypes.StoreKey],
		//app.GetSubspace(crisistypes.ModuleName),
		invCheckPeriod,
		appKeepers.BankKeeper,
		authtypes.FeeCollectorName,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)

	appKeepers.UpgradeKeeper = upgradekeeper.NewKeeper(
		skipUpgradeHeights,
		appKeepers.keys[upgradetypes.StoreKey],
		appCodec,
		homePath,
		bApp,
		authtypes.NewModuleAddress(govtypes.ModuleName).String())
}

// InitNormalKeepers initializes all 'normal' keepers (account, app, bank, auth, staking, distribution, slashing, transfer, gamm, IBC router, pool incentives, governance, mint, txfees keepers).
func (appKeepers *AppKeepers) InitNormalKeepers(
	appCodec codec.Codec,
	encodingConfig appparams.EncodingConfig,
	bApp *baseapp.BaseApp,
	maccPerms map[string][]string,
	dataDir string,
	wasmDir string,
	wasmConfig wasmtypes.WasmConfig,
	wasmOpts []wasmkeeper.Option,
	blockedAddress map[string]bool,
	ibcWasmConfig ibcwasmtypes.WasmConfig,
) {
	legacyAmino := encodingConfig.Amino

	accountKeeper := authkeeper.NewAccountKeeper(
		appCodec,
		appKeepers.keys[authtypes.StoreKey],
		authtypes.ProtoBaseAccount,
		maccPerms,
		AccountAddressPrefix,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)
	appKeepers.AccountKeeper = accountKeeper

	bankKeeper := bankkeeper.NewBaseKeeper(
		appCodec,
		appKeepers.keys[banktypes.StoreKey],
		appKeepers.AccountKeeper,
		blockedAddress,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)
	appKeepers.BankKeeper = bankKeeper

	stakingKeeper := stakingkeeper.NewKeeper(
		appCodec,
		appKeepers.keys[stakingtypes.StoreKey],
		appKeepers.AccountKeeper,
		appKeepers.BankKeeper,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)
	appKeepers.StakingKeeper = stakingKeeper

	mintKeeper := mintkeeper.NewKeeper(
		appCodec,
		appKeepers.keys[minttypes.StoreKey],
		appKeepers.StakingKeeper,
		appKeepers.AccountKeeper,
		appKeepers.BankKeeper,
		authtypes.FeeCollectorName,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)
	appKeepers.MintKeeper = mintKeeper

	distrKeeper := distrkeeper.NewKeeper(
		appCodec,
		appKeepers.keys[distrtypes.StoreKey],
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
		appKeepers.keys[slashingtypes.StoreKey],
		appKeepers.StakingKeeper,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)
	appKeepers.SlashingKeeper = slashingKeeper

	feeGrantKeeper := feegrantkeeper.NewKeeper(
		appCodec,
		appKeepers.keys[feegrant.StoreKey],
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
	)

	transferKeeper := ibctransferkeeper.NewKeeper(
		appCodec,
		appKeepers.keys[ibctransfertypes.StoreKey],
		appKeepers.GetSubspace(ibctransfertypes.ModuleName),
		appKeepers.IBCKeeper.ChannelKeeper,
		appKeepers.IBCKeeper.ChannelKeeper,
		&appKeepers.IBCKeeper.PortKeeper,
		appKeepers.AccountKeeper,
		appKeepers.BankKeeper,
		appKeepers.ScopedTransferKeeper,
	)
	appKeepers.TransferKeeper = transferKeeper

	appKeepers.RawIcs20TransferAppModule = transfer.NewAppModule(appKeepers.TransferKeeper)

	// Hooks Middleware
	hooksTransferModule := qtransfer.NewIBCMiddleware(transfer.NewIBCModule(appKeepers.TransferKeeper), &appKeepers.HooksICS4Wrapper)
	appKeepers.TransferStack = &hooksTransferModule

	appKeepers.ICAControllerKeeper = icacontrollerkeeper.NewKeeper(
		appCodec, appKeepers.keys[icacontrollertypes.StoreKey],
		appKeepers.GetSubspace(icacontrollertypes.SubModuleName),
		appKeepers.IBCKeeper.ChannelKeeper, // may be replaced with middleware such as ics29 fee
		appKeepers.IBCKeeper.ChannelKeeper,
		&appKeepers.IBCKeeper.PortKeeper,
		appKeepers.ScopedICAControllerKeeper,
		bApp.MsgServiceRouter(),
	)
	appKeepers.ICAHostKeeper = icahostkeeper.NewKeeper(
		appCodec,
		appKeepers.keys[icahosttypes.StoreKey],
		appKeepers.GetSubspace(icahosttypes.SubModuleName),
		appKeepers.IBCKeeper.ChannelKeeper, // can be replaced with rate limiter ICS 4 Wrapper
		appKeepers.IBCKeeper.ChannelKeeper,
		&appKeepers.IBCKeeper.PortKeeper,
		appKeepers.AccountKeeper,
		appKeepers.ScopedICAHostKeeper,
		bApp.MsgServiceRouter(),
	)

	//icaModule := ica.NewAppModule(&appKeepers.ICAControllerKeeper, &appKeepers.ICAHostKeeper)
	//
	//icaHostIBCModule := icahost.NewIBCModule(appKeepers.ICAHostKeeper)

	evidenceKeeper := evidencekeeper.NewKeeper(
		appCodec, appKeepers.keys[evidencetypes.StoreKey], appKeepers.StakingKeeper, appKeepers.SlashingKeeper,
	)
	appKeepers.EvidenceKeeper = *evidenceKeeper

	govRouter := govv1beta1.NewRouter()
	govRouter.
		AddRoute(govtypes.RouterKey, govv1beta1.ProposalHandler).
		AddRoute(paramproposal.RouterKey, params.NewParamChangeProposalHandler(appKeepers.ParamsKeeper)).
		AddRoute(upgradetypes.RouterKey, upgrade.NewSoftwareUpgradeProposalHandler(appKeepers.UpgradeKeeper)).
		AddRoute(ibcclienttypes.RouterKey, ibcclient.NewClientProposalHandler(appKeepers.IBCKeeper.ClientKeeper)).
		AddRoute(ibchost.RouterKey, ibcclient.NewClientProposalHandler(appKeepers.IBCKeeper.ClientKeeper))

	govConfig := govtypes.DefaultConfig()
	govKeeper := govkeeper.NewKeeper(
		appCodec,
		appKeepers.keys[govtypes.StoreKey],
		appKeepers.AccountKeeper,
		appKeepers.BankKeeper,
		appKeepers.StakingKeeper,
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
	//epochsModule := epochsmodule.NewAppModule(appCodec, appKeepers.EpochsKeeper)

	//qoracleModule := qoraclemodule.NewAppModule(appCodec, appKeepers.QOracleKeeper, appKeepers.QOsmosisKeeper)

	appKeepers.QTransferKeeper = qtransferkeeper.NewKeeper(
		appCodec,
		appKeepers.keys[qtransfertypes.ModuleName],
		appKeepers.GetSubspace(qtransfertypes.ModuleName),
		appKeepers.AccountKeeper,
	)
	//qtranserModule := qtransfer.NewAppModule(app.QTransferKeeper)

	appKeepers.QOracleKeeper = qoraclemodulekeeper.NewKeeper(
		appCodec,
		appKeepers.keys[qoraclemoduletypes.StoreKey],
		appKeepers.memKeys[qoraclemoduletypes.MemStoreKey],
		appKeepers.tkeys[qoraclemoduletypes.TStoreKey],
		appKeepers.GetSubspace(qoraclemoduletypes.ModuleName),
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
	)

	appKeepers.QOsmosisKeeper = qosmokeeper.NewKeeper(
		appCodec,
		appKeepers.keys[qosmotypes.StoreKey],
		appKeepers.GetSubspace(qosmotypes.SubModuleName),
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
		appKeepers.IBCKeeper.ClientKeeper,
		appKeepers.IBCKeeper.ChannelKeeper,
		appKeepers.IBCKeeper.ChannelKeeper,
		&appKeepers.IBCKeeper.PortKeeper,
		appKeepers.ScopedQOsmosisKeeper,
		appKeepers.QOracleKeeper,
	)
	//qosmoIBCModule := qosmo.NewIBCModule(app.QOsmosisKeeper)

	appKeepers.QOracleKeeper.RegisterPoolOracle(appKeepers.QOsmosisKeeper)
	appKeepers.QOracleKeeper.Seal()
	//qoracleModule := qoraclemodule.NewAppModule(appCodec, appKeepers.QOracleKeeper, appKeepers.QOsmosisKeeper)

	appKeepers.QTransferKeeper = qtransferkeeper.NewKeeper(
		appCodec,
		appKeepers.keys[qtransfertypes.ModuleName],
		appKeepers.GetSubspace(qtransfertypes.ModuleName),
		appKeepers.AccountKeeper,
	)
	//qtranserModule := qtransfer.NewAppModule(appKeepers.QTransferKeeper)

	appKeepers.QVestingKeeper = *qvestingmodulekeeper.NewKeeper(
		appCodec,
		appKeepers.keys[qvestingmoduletypes.StoreKey],
		appKeepers.keys[qvestingmoduletypes.MemStoreKey],
		appKeepers.GetSubspace(qvestingmoduletypes.ModuleName),
		appKeepers.AccountKeeper,
		appKeepers.BankKeeper,
	)

	//qvestingModule := qvestingmodule.NewAppModule(appCodec, appKeepers.QVestingKeeper, appKeepers.AccountKeeper, appKeepers.BankKeeper)

	// Authz
	appKeepers.AuthzKeeper = authzkeeper.NewKeeper(
		appKeepers.keys[authzkeeper.StoreKey],
		appCodec,
		bApp.MsgServiceRouter(),
		appKeepers.AccountKeeper,
	)

	// Set epoch hooks
	appKeepers.EpochsKeeper.SetHooks(
		epochsmoduletypes.NewMultiEpochHooks(
			appKeepers.QOsmosisKeeper.EpochHooks(),
		),
	)

	/// Token factory Module
	appKeepers.TfKeeper = tfkeeper.NewKeeper(appKeepers.keys[tftypes.StoreKey],
		appKeepers.GetSubspace(tftypes.ModuleName),
		appKeepers.AccountKeeper,
		appKeepers.BankKeeper,
		appKeepers.DistrKeeper,
	)
	//tfModule := tfmodule.NewAppModule(appKeepers.TfKeeper,
	//	appKeepers.AccountKeeper,
	//	appKeepers.BankKeeper,
	//)

	callback := owasm.NewCallbackPlugin(appKeepers.WasmKeeper, appKeepers.QTransferKeeper.GetQTransferAcc())

	// AUDIT CHECK IS THIS TYPE ASSERTION FOR TYPE CASTING INTERFACE TO STRUCT SAFE?
	tmpBankBaseKeeper := appKeepers.BankKeeper.(bankkeeper.BaseKeeper)

	wasmOpts = append(owasm.RegisterCustomPlugins(appKeepers.QOracleKeeper, &tmpBankBaseKeeper, callback), wasmOpts...)
	wasmOpts = append(tfbindings.RegisterCustomPlugins(&tmpBankBaseKeeper, &appKeepers.TfKeeper), wasmOpts...)

	// The last arguments can contain custom message handlers, and custom query handlers,
	// if we want to allow any custom callbacks
	supportedFeatures := "iterator,staking,stargate"

	wasmKeeper := wasmkeeper.NewKeeper(
		appCodec,
		appKeepers.keys[wasmtypes.StoreKey],
		// app.GetSubspace(wasm.ModuleName),
		appKeepers.AccountKeeper,
		appKeepers.BankKeeper,
		appKeepers.StakingKeeper,
		distrkeeper.NewQuerier(appKeepers.DistrKeeper),
		appKeepers.IBCKeeper.ChannelKeeper,
		appKeepers.IBCKeeper.ChannelKeeper,
		&appKeepers.IBCKeeper.PortKeeper,
		appKeepers.ScopedWasmKeeper,
		appKeepers.TransferKeeper,
		bApp.MsgServiceRouter(),
		bApp.GRPCQueryRouter(),
		wasmDir,
		wasmConfig,
		supportedFeatures,
		authtypes.NewModuleAddress(govtypes.ModuleName).String(),
		wasmOpts...,
	)
	appKeepers.WasmKeeper = &wasmKeeper

	ibcRouter := ibcporttypes.NewRouter()

	wasmHooks := qtransfer.NewWasmHooks(appKeepers.QTransferKeeper, *appKeepers.WasmKeeper)
	appKeepers.Ics20WasmHooks = &wasmHooks
	appKeepers.HooksICS4Wrapper = qtransfer.NewICS4Middleware(
		appKeepers.IBCKeeper.ChannelKeeper,
		appKeepers.Ics20WasmHooks,
	)

	// Register host and authentication routes
	// TODO_IMPORTANT - addition of qtransfer module
	// TODO_IMPORTANT - CROSS VERIFY wasm.NewIBCHandler(appKeepers.WasmKeeper, appKeepers.IBCKeeper.ChannelKeeper, appKeepers.IBCKeeper.ChannelKeeper))
	ibcRouter.
		AddRoute(wasmtypes.ModuleName, wasm.NewIBCHandler(appKeepers.WasmKeeper, appKeepers.IBCKeeper.ChannelKeeper, appKeepers.IBCKeeper.ChannelKeeper)).
		AddRoute(icahosttypes.SubModuleName, icahost.NewIBCModule(appKeepers.ICAHostKeeper)).
		AddRoute(ibctransfertypes.ModuleName, appKeepers.TransferStack).
		// AddRoute(ibctransfertypes.ModuleName, decoratedTransferIBCModule).
		// AddRoute(intergammmoduletypes.ModuleName, icaControllerIBCModule).
		AddRoute(qosmotypes.SubModuleName, qosmo.NewIBCModule(appKeepers.QOsmosisKeeper))
	//	AddRoute(qoraclemoduletypes.ModuleName, qoracleIBCModule)

	appKeepers.IBCKeeper.SetRouter(ibcRouter)

	// todo : check if all keeper initialised (one check finished)

}

// initParamsKeeper init params keeper and its subspaces
func (appKeepers *AppKeepers) initParamsKeeper(appCodec codec.BinaryCodec, legacyAmino *codec.LegacyAmino, key, tkey storetypes.StoreKey) paramskeeper.Keeper {
	paramsKeeper := paramskeeper.NewKeeper(appCodec, legacyAmino, key, tkey)

	paramsKeeper.Subspace(authtypes.ModuleName)
	paramsKeeper.Subspace(banktypes.ModuleName)
	paramsKeeper.Subspace(stakingtypes.ModuleName)
	paramsKeeper.Subspace(minttypes.ModuleName)
	paramsKeeper.Subspace(distrtypes.ModuleName)
	paramsKeeper.Subspace(slashingtypes.ModuleName)
	paramsKeeper.Subspace(govtypes.ModuleName)
	paramsKeeper.Subspace(crisistypes.ModuleName)
	paramsKeeper.Subspace(ibctransfertypes.ModuleName)
	paramsKeeper.Subspace(ibchost.ModuleName)
	paramsKeeper.Subspace(epochsmoduletypes.ModuleName)
	paramsKeeper.Subspace(qoraclemoduletypes.ModuleName)
	paramsKeeper.Subspace(qosmotypes.SubModuleName)
	paramsKeeper.Subspace(icacontrollertypes.SubModuleName).WithKeyTable(icacontrollertypes.ParamKeyTable())
	paramsKeeper.Subspace(icqtypes.ModuleName)
	paramsKeeper.Subspace(icahosttypes.SubModuleName)
	paramsKeeper.Subspace(wasmtypes.ModuleName)
	paramsKeeper.Subspace(qtransfertypes.ModuleName)
	paramsKeeper.Subspace(tftypes.ModuleName)
	paramsKeeper.Subspace(qvestingmoduletypes.ModuleName)
	paramsKeeper.Subspace(authztypes.ModuleName)

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
		qoraclemoduletypes.StoreKey,
		qosmotypes.StoreKey,
		icacontrollertypes.StoreKey,
		icahosttypes.StoreKey,
		icqtypes.StoreKey,
		wasmtypes.StoreKey,
		qtransfertypes.StoreKey,
		tftypes.StoreKey,
		qvestingmoduletypes.StoreKey, // TODO delete this if unused
		authzkeeper.StoreKey,
		consensusparamtypes.StoreKey,
		crisistypes.StoreKey,
	}
}
